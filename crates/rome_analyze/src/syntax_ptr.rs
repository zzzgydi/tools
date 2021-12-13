//! SyntaxNode isn't thread-safe, so this is a temporary solution for tracking specific nodes
//! and tokens across threads.

use rslint_parser::{NodeOrToken, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, TextRange};

#[derive(Debug, Clone, Copy)]
pub struct SyntaxNodePtr {
	kind: SyntaxKind,
	range: TextRange,
	trimmed_range: TextRange,
}

impl SyntaxNodePtr {
	pub fn kind(&self) -> SyntaxKind {
		self.kind
	}

	pub fn range(&self) -> TextRange {
		self.range
	}

	pub fn trimmed_range(&self) -> TextRange {
		self.trimmed_range
	}
}

impl SyntaxTokenPtr {
	pub fn kind(&self) -> SyntaxKind {
		self.kind
	}

	pub fn range(&self) -> TextRange {
		self.range
	}

	pub fn trimmed_range(&self) -> TextRange {
		self.trimmed_range
	}
}

impl SyntaxElementPtr {
	pub fn kind(&self) -> SyntaxKind {
		match self {
			SyntaxElementPtr::Node(it) => it.kind(),
			SyntaxElementPtr::Token(it) => it.kind(),
		}
	}

	pub fn range(&self) -> TextRange {
		match self {
			SyntaxElementPtr::Node(it) => it.range(),
			SyntaxElementPtr::Token(it) => it.range(),
		}
	}

	pub fn trimmed_range(&self) -> TextRange {
		match self {
			SyntaxElementPtr::Node(it) => it.trimmed_range(),
			SyntaxElementPtr::Token(it) => it.trimmed_range(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct SyntaxTokenPtr {
	kind: SyntaxKind,
	range: TextRange,
	trimmed_range: TextRange,
}

#[derive(Debug, Clone, Copy)]
pub enum SyntaxElementPtr {
	Node(SyntaxNodePtr),
	Token(SyntaxTokenPtr),
}

impl From<SyntaxNode> for SyntaxNodePtr {
	fn from(n: SyntaxNode) -> Self {
		let kind = n.kind();
		let range = n.text_range();
		let trimmed_range = n.text_trimmed_range();

		Self {
			kind,
			range,
			trimmed_range,
		}
	}
}

impl From<SyntaxToken> for SyntaxTokenPtr {
	fn from(t: SyntaxToken) -> Self {
		let kind = t.kind();
		let range = t.text_range();
		let trimmed_range = t.text_trimmed_range();

		Self {
			kind,
			range,
			trimmed_range,
		}
	}
}

impl From<SyntaxNodePtr> for SyntaxElementPtr {
	fn from(p: SyntaxNodePtr) -> Self {
		SyntaxElementPtr::Node(p)
	}
}

impl From<SyntaxNode> for SyntaxElementPtr {
	fn from(n: SyntaxNode) -> Self {
		SyntaxElementPtr::Node(n.into())
	}
}

impl From<SyntaxTokenPtr> for SyntaxElementPtr {
	fn from(p: SyntaxTokenPtr) -> Self {
		SyntaxElementPtr::Token(p)
	}
}

impl From<SyntaxToken> for SyntaxElementPtr {
	fn from(t: SyntaxToken) -> Self {
		SyntaxElementPtr::Token(t.into())
	}
}

impl From<SyntaxElement> for SyntaxElementPtr {
	fn from(e: SyntaxElement) -> Self {
		match e {
			NodeOrToken::Node(it) => it.into(),
			NodeOrToken::Token(it) => it.into(),
		}
	}
}
