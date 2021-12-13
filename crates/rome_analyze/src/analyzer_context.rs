use rslint_parser::{AstNode, SyntaxNode, TextRange};

use crate::{AnalysisAPI, FileId};

pub struct AnalyzerContext<'a> {
	pub file_id: FileId,
	host: &'a AnalysisAPI,
}

impl<'a> AnalyzerContext<'a> {
	pub fn new(host: &'a AnalysisAPI, file_id: FileId) -> Self {
		Self { host, file_id }
	}

	pub fn tree(&self) -> SyntaxNode {
		self.host.parse(self.file_id)
	}

	pub fn query_nodes<T: AstNode>(&self) -> impl Iterator<Item = T> {
		self.host.query_nodes(self.file_id)
	}

	pub fn query_nodes_in_range<T: AstNode>(&self, range: TextRange) -> impl Iterator<Item = T> {
		self.host.query_nodes_in_range(self.file_id, range)
	}
}
