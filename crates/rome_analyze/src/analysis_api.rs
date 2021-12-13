use std::sync::Arc;

use anyhow::Result;
use rslint_parser::{parse_text, AstNode, SyntaxNode, TextRange};
use tracing::{error, trace};

use crate::{
	analyzers,
	assist_context::AssistContext,
	assists,
	suppressions::{self, Suppressions},
	Analysis, AnalyzeDiagnostic, AnalyzerContext, AnalyzerDatabase, AnalyzerResult, FileId,
	RootDatabase, SourceDatabase,
};

#[derive(Default)]
pub struct AnalysisAPI {
	db: RootDatabase,
}

impl AnalysisAPI {
	pub fn new() -> Self {
		Self {
			db: Default::default(),
		}
	}

	pub fn set_file_text(&mut self, file_id: FileId, text: impl Into<Arc<String>>) {
		self.db.set_file_text(file_id, text.into())
	}
}

// These should be on a database snapshot, but the current database contains
// non-Send SyntaxNodes. Should either make SyntaxNodes Send (using atomics)
// or remove SyntaxNode as a possible query parameter/return type.
impl AnalysisAPI {
	pub fn get_file_text(&self, file_id: FileId) -> Arc<String> {
		self.db.file_text(file_id)
	}

	pub fn parse(&self, file_id: FileId) -> SyntaxNode {
		let text = self.db.file_text(file_id);
		parse_text(&text, file_id.0).syntax()
	}

	pub fn suppressions(&self, file_id: FileId) -> Result<Suppressions> {
		let node = self.parse(file_id);
		suppressions::compute(node)
	}

	pub fn query_nodes<T: AstNode>(&self, file_id: FileId) -> impl Iterator<Item = T> {
		trace!("Querying Nodes!, {:?}", std::any::type_name::<T>());
		let nodes = self.db.nodes(file_id, T::can_cast);
		let len = nodes.len();
		(0..len).filter_map(move |i| T::cast(nodes[i].clone()))
	}

	pub fn query_nodes_in_range<T: AstNode>(
		&self,
		file_id: FileId,
		range: TextRange,
	) -> impl Iterator<Item = T> {
		trace!("Querying Nodes in range!, {:?}", std::any::type_name::<T>());
		let nodes = self.db.nodes_in_range(file_id, T::can_cast, range);
		let len = nodes.len();
		(0..len).filter_map(move |i| T::cast(nodes[i].clone()))
	}

	pub fn diagnostics(&self, file_id: FileId) -> Result<Vec<AnalyzeDiagnostic>> {
		let ctx = AnalyzerContext::new(self, file_id);
		let suppressions = self.suppressions(file_id)?;

		let diagnostics = analyzers::diagnostic_providers()
			.filter_map(|a| -> Option<Vec<_>> {
				let ranges = suppressions.ranges(a.name);

				let analyze_fn = a.analyze;
				match analyze_fn(&ctx) {
					Ok(signal) => {
						let diagnostics = signal
							.into_diagnostics()
							.filter(|d| !ranges.iter().any(|r| r.contains_range(d.range)))
							.collect();
						Some(diagnostics)
					}
					Err(e) => {
						error!("Err: {:?}", e);
						None
					}
				}
			})
			.flatten()
			.collect();

		Ok(diagnostics)
	}

	pub fn assists(&self, file_id: FileId, cursor_range: TextRange) -> AnalyzerResult {
		trace!("Assists Cursor Range: {:?}", cursor_range);
		let a_ctx = AnalyzerContext::new(self, file_id);
		let ctx = AssistContext::new(&a_ctx, cursor_range);

		let mut signals = vec![];

		for provider in assists::all() {
			let analyze_fn = provider.analyze;
			match analyze_fn(&ctx) {
				Ok(signal) => signals.extend(signal.signals.into_iter()),

				Err(e) => {
					error!("Err: {:?}", e);
				}
			}
		}

		Ok(signals.into())
	}

	pub fn analyze(&self, file_id: FileId) -> AnalyzerResult {
		let ctx = AnalyzerContext::new(self, file_id);
		let suppressions = self.suppressions(file_id)?;

		let mut signals = vec![];

		for analyzer in analyzers::all() {
			let analyze_fn = analyzer.analyze;
			match analyze_fn(&ctx) {
				Ok(signal) => {
					for s in signal.signals {
						if s.is_diagnostic() && suppressions.match_range(analyzer.name, s.range()) {
							continue;
						}
						signals.push(s);
					}
				}
				Err(e) => error!("Err: {:?}", e),
			}
		}
		Ok(Analysis { signals })
	}
}
