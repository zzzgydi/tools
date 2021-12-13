use std::marker::PhantomData;

use tracing::trace;

use crate::{cursor, Language, NodeOrToken, SyntaxElement, SyntaxNode, SyntaxToken};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SyntaxDetachedNode<L: Language> {
	raw: cursor::detached::SyntaxDetachedNode,
	_p: PhantomData<L>,
}

impl<L: Language> SyntaxDetachedNode<L> {
	pub fn syntax_node(&self) -> SyntaxNode<L> {
		SyntaxNode::new_root(self.raw.green.clone())
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SyntaxDetachedToken<L: Language> {
	raw: cursor::detached::SyntaxDetachedToken,
	_p: PhantomData<L>,
}

impl<L: Language> SyntaxDetachedToken<L> {
	pub fn syntax_token(&self) -> SyntaxToken<L> {
		trace!("Syntax Token");
		SyntaxToken {
			raw: cursor::SyntaxToken::new_detached(&self.raw.green),
			_p: PhantomData,
		}
	}
}

pub type SyntaxDetachedElement<L> = NodeOrToken<SyntaxDetachedNode<L>, SyntaxDetachedToken<L>>;

impl<L: Language> SyntaxDetachedElement<L> {
	pub fn syntax_element(&self) -> SyntaxElement<L> {
		match self {
			NodeOrToken::Node(it) => NodeOrToken::Node(it.syntax_node()),
			NodeOrToken::Token(it) => NodeOrToken::Token(it.syntax_token()),
		}
	}
}

impl<L: Language> From<SyntaxDetachedNode<L>> for SyntaxDetachedElement<L> {
	fn from(node: SyntaxDetachedNode<L>) -> SyntaxDetachedElement<L> {
		NodeOrToken::Node(node)
	}
}

impl<L: Language> From<SyntaxDetachedToken<L>> for SyntaxDetachedElement<L> {
	fn from(token: SyntaxDetachedToken<L>) -> SyntaxDetachedElement<L> {
		NodeOrToken::Token(token)
	}
}

impl<L: Language> From<SyntaxNode<L>> for SyntaxDetachedNode<L> {
	fn from(node: SyntaxNode<L>) -> SyntaxDetachedNode<L> {
		let raw = cursor::detached::SyntaxDetachedNode {
			green: node.raw.green().into(),
		};
		SyntaxDetachedNode {
			raw,
			_p: PhantomData,
		}
	}
}

impl<L: Language> From<SyntaxToken<L>> for SyntaxDetachedToken<L> {
	fn from(token: SyntaxToken<L>) -> SyntaxDetachedToken<L> {
		let raw = cursor::detached::SyntaxDetachedToken {
			green: token.raw.green().to_owned(),
		};
		SyntaxDetachedToken {
			raw,
			_p: PhantomData,
		}
	}
}

impl<L: Language> From<SyntaxNode<L>> for SyntaxDetachedElement<L> {
	fn from(node: SyntaxNode<L>) -> SyntaxDetachedElement<L> {
		SyntaxDetachedElement::Node(node.into())
	}
}

impl<L: Language> From<SyntaxToken<L>> for SyntaxDetachedElement<L> {
	fn from(token: SyntaxToken<L>) -> SyntaxDetachedElement<L> {
		SyntaxDetachedElement::Token(token.into())
	}
}

impl<L: Language> From<SyntaxElement<L>> for SyntaxDetachedElement<L> {
	fn from(element: SyntaxElement<L>) -> SyntaxDetachedElement<L> {
		match element {
			NodeOrToken::Node(it) => NodeOrToken::Node(it.into()),
			NodeOrToken::Token(it) => NodeOrToken::Token(it.into()),
		}
	}
}
