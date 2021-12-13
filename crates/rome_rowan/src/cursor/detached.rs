use crate::{GreenNode, GreenToken, NodeOrToken};

use super::{SyntaxNode, SyntaxToken};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SyntaxDetachedNode {
	pub(crate) green: GreenNode,
}

impl From<SyntaxNode> for SyntaxDetachedNode {
	fn from(node: SyntaxNode) -> SyntaxDetachedNode {
		let green = node.clone_subtree().green_ref().to_owned();
		SyntaxDetachedNode { green }
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SyntaxDetachedToken {
	pub(crate) green: GreenToken,
}

impl From<SyntaxToken> for SyntaxDetachedToken {
	fn from(token: SyntaxToken) -> SyntaxDetachedToken {
		let green = token.green().to_owned();
		SyntaxDetachedToken { green }
	}
}

pub type SyntaxDetachedElement = NodeOrToken<SyntaxDetachedNode, SyntaxDetachedToken>;

impl From<SyntaxDetachedNode> for SyntaxDetachedElement {
	fn from(node: SyntaxDetachedNode) -> SyntaxDetachedElement {
		NodeOrToken::Node(node)
	}
}

impl From<SyntaxDetachedToken> for SyntaxDetachedElement {
	fn from(token: SyntaxDetachedToken) -> SyntaxDetachedElement {
		NodeOrToken::Token(token)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn assert_send_sync() {
		fn f<T: Send + Sync>() {}
		f::<SyntaxDetachedNode>();
		f::<SyntaxDetachedToken>();
		f::<SyntaxDetachedElement>();
	}
}
