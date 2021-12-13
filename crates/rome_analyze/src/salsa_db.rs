use std::sync::Arc;

use rslint_parser::{parse_text, SyntaxDetachedNode, SyntaxKind, SyntaxNode, TextRange};
use tracing::trace;

use crate::FileId;

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: salsa::Database {
	#[salsa::input]
	fn file_text(&self, file_id: FileId) -> Arc<String>;

	fn parse(&self, file_id: FileId) -> SyntaxDetachedNode;
}

fn parse(db: &dyn SourceDatabase, file_id: FileId) -> SyntaxDetachedNode {
	let text = db.file_text(file_id);
	parse_text(&text, file_id.0).syntax().into()
}

impl salsa::Database for RootDatabase {}

// SyntaxNodes are not `Send`
// impl salsa::ParallelDatabase for RootDatabase {
// 	fn snapshot(&self) -> salsa::Snapshot<Self> {
// 		salsa::Snapshot::new(RootDatabase {
// 			storage: self.storage.snapshot(),
// 		})
// 	}
// }

#[salsa::query_group(AnalyzerDatabaseStorage)]
pub trait AnalyzerDatabase: SourceDatabase {
	fn nodes(&self, file_id: FileId, can_cast: fn(SyntaxKind) -> bool) -> Arc<Vec<SyntaxNode>>;

	fn nodes_in_range(
		&self,
		file_id: FileId,
		can_cast: fn(SyntaxKind) -> bool,
		range: TextRange,
	) -> Arc<Vec<SyntaxNode>>;
}

fn nodes(
	db: &dyn AnalyzerDatabase,
	file_id: FileId,
	can_cast: fn(SyntaxKind) -> bool,
) -> Arc<Vec<SyntaxNode>> {
	trace!("Querying Salsa for Nodes!");
	let tree = db.parse(file_id).syntax_node();
	let nodes = tree.descendants().filter(|n| can_cast(n.kind())).collect();
	Arc::new(nodes)
}

fn nodes_in_range(
	db: &dyn AnalyzerDatabase,
	file_id: FileId,
	can_cast: fn(SyntaxKind) -> bool,
	range: TextRange,
) -> Arc<Vec<SyntaxNode>> {
	trace!("Querying Salsa for Nodes in Range!");
	let nodes = db.nodes(file_id, can_cast);
	let nodes = nodes
		.iter()
		.filter(|n| n.text_range().contains_range(range))
		.cloned()
		.collect();
	Arc::new(nodes)
}

#[salsa::database(SourceDatabaseStorage, AnalyzerDatabaseStorage)]
#[derive(Default)]
pub struct RootDatabase {
	storage: salsa::Storage<Self>,
}
