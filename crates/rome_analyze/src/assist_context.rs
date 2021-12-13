#![allow(unused)]
use rslint_parser::{AstNode, SyntaxNode, SyntaxToken, TextRange, TextSize, TokenAtOffset};

use crate::{AnalyzerContext, FileId};

pub struct AssistContext<'a> {
	file_id: FileId,
	cursor_range: TextRange,
	offset: TextSize,
	analyzer_context: &'a AnalyzerContext<'a>,
}

impl<'a> AssistContext<'a> {
	pub(crate) fn new(analyzer_context: &'a AnalyzerContext, cursor_range: TextRange) -> Self {
		let offset = cursor_range.start();
		Self {
			cursor_range,
			offset,
			analyzer_context,
			file_id: analyzer_context.file_id,
		}
	}

	pub(crate) fn range(&self) -> TextRange {
		self.cursor_range
	}

	pub(crate) fn tree(&self) -> SyntaxNode {
		self.analyzer_context.tree()
	}

	pub(crate) fn query_nodes<T: AstNode>(&self) -> impl Iterator<Item = T> {
		self.analyzer_context.query_nodes()
	}

	pub(crate) fn query_nodes_in_range<T: AstNode>(
		&self,
		range: TextRange,
	) -> impl Iterator<Item = T> {
		self.analyzer_context.query_nodes_in_range(range)
	}

	pub(crate) fn token_at_offset(&self) -> TokenAtOffset<SyntaxToken> {
		self.tree().token_at_offset(self.offset)
	}
}
