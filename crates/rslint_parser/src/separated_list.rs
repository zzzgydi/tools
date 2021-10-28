use crate::{AstNode, JsLanguage, NodeOrToken};
use rome_rowan::{Language, SyntaxElement, SyntaxNode, SyntaxToken};

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SyntaxSeparatedElement<L: Language> {
	pub node: Option<SyntaxNode<L>>,
	pub separator: Option<SyntaxToken<L>>,
}

impl<L: Language> SyntaxSeparatedElement<L> {
	pub fn new(node: Option<SyntaxNode<L>>, separator: Option<SyntaxToken<L>>) -> Self {
		Self { node, separator }
	}
}

#[derive(Debug, Clone)]
pub struct SyntaxSeparatedList<L: Language> {
	raw: SyntaxNode<L>,
	ph: PhantomData<L>,
}

impl<L: Language> SyntaxSeparatedList<L> {
	pub fn new(raw: SyntaxNode<L>) -> Self {
		Self {
			raw,
			ph: PhantomData,
		}
	}

	pub fn nodes(&self) -> impl Iterator<Item = Option<SyntaxNode<L>>> {
		self.iter().map(|e| e.node)
	}

	pub fn separators(&self) -> impl Iterator<Item = SyntaxToken<L>> {
		// Only trailing separators are optional, that's why calling flatten is safe here.
		self.iter().map(|e| e.separator).flatten()
	}

	pub fn trailing_separator(&self) -> Option<SyntaxToken<L>> {
		self.iter().last().and_then(|last| last.separator)
	}

	pub fn has_trailing_separator(&self) -> bool {
		self.trailing_separator().is_some()
	}

	pub fn iter(&self) -> impl Iterator<Item = SyntaxSeparatedElement<L>> {
		SyntaxSeparatedListIterator {
			inner: self.raw.children_with_tokens(),
			ph: PhantomData,
		}
	}
}

struct SyntaxSeparatedListIterator<I, L> {
	inner: I,
	ph: PhantomData<L>,
}

impl<L: Language, I: Iterator<Item = SyntaxElement<L>>> Iterator
	for SyntaxSeparatedListIterator<I, L>
{
	type Item = SyntaxSeparatedElement<L>;

	fn next(&mut self) -> Option<Self::Item> {
		Some(match self.inner.next()? {
			NodeOrToken::Node(node) => SyntaxSeparatedElement::new(
				Some(node),
				match self.inner.next() {
					Some(NodeOrToken::Node(_)) => panic!(
						"Invalid separated list. Two nodes that aren't separated by a separator."
					),
					Some(NodeOrToken::Token(separator)) => Some(separator),
					None => None,
				},
			),
			NodeOrToken::Token(separator) => SyntaxSeparatedElement::new(None, Some(separator)),
		})
	}
}

pub struct AstSeparatedElement<N> {
	pub node: Option<N>,
	pub separator: Option<crate::SyntaxToken>,
}

impl<N: AstNode> AstSeparatedElement<N> {
	fn new(node: Option<N>, separator: Option<crate::SyntaxToken>) -> Self {
		Self { node, separator }
	}
}

#[derive(Debug, Clone)]
pub struct AstSeparatedList<N> {
	inner: SyntaxSeparatedList<JsLanguage>,
	ph: PhantomData<N>,
}

impl<N: AstNode> AstSeparatedList<N> {
	pub fn new(raw: crate::SyntaxNode) -> Self {
		Self {
			inner: SyntaxSeparatedList::new(raw),
			ph: PhantomData,
		}
	}

	pub fn separators(&self) -> impl Iterator<Item = crate::SyntaxToken> {
		self.inner.separators().map(crate::SyntaxToken::from)
	}

	pub fn nodes(&self) -> impl Iterator<Item = Option<N>> {
		self.inner
			.nodes()
			.map(|node| node.map(|node| N::cast(node).unwrap()))
	}

	pub fn iter(&self) -> impl Iterator<Item = AstSeparatedElement<N>> {
		self.inner.iter().map(|syntax| {
			AstSeparatedElement::new(
				syntax.node.map(|node| N::cast(node).unwrap()),
				syntax.separator.map(crate::SyntaxToken::from),
			)
		})
	}

	pub fn has_trailing_separator(&self) -> bool {
		self.inner.has_trailing_separator()
	}

	pub fn trailing_separator(&self) -> Option<crate::SyntaxToken> {
		self.inner.trailing_separator()
	}
}

#[cfg(test)]
mod tests {
	use crate::separated_list::{SyntaxSeparatedElement, SyntaxSeparatedList};
	use crate::{GreenNode, JsLanguage, SyntaxKind};
	use rome_rowan::{GreenToken, NodeOrToken, SyntaxNode};

	fn new_node(kind: SyntaxKind, children: Vec<NodeOrToken<GreenNode, GreenToken>>) -> GreenNode {
		GreenNode::new(rome_rowan::SyntaxKind(kind.into()), children)
	}

	fn new_token(kind: SyntaxKind, text: &str) -> GreenToken {
		GreenToken::new(rome_rowan::SyntaxKind(kind.into()), text)
	}

	fn create_separated_list(elements: Vec<Option<i32>>) -> SyntaxSeparatedList<JsLanguage> {
		let mut children: Vec<NodeOrToken<GreenNode, GreenToken>> = Vec::new();

		for (index, element) in elements.iter().enumerate() {
			if index > 0 {
				children.push(new_token(SyntaxKind::COMMA, ",").into());
			}

			if let Some(number) = element {
				children.push(
					new_node(
						SyntaxKind::LITERAL,
						vec![new_token(SyntaxKind::NUMBER, format!("{}", number).as_str()).into()],
					)
					.into(),
				)
			}
		}

		let list_root =
			SyntaxNode::<JsLanguage>::new_root(new_node(SyntaxKind::ARG_LIST, children));

		SyntaxSeparatedList::new(list_root)
	}

	fn assert_nodes<T: Iterator<Item = Option<SyntaxNode<JsLanguage>>>>(
		nodes: T,
		expected: Vec<&str>,
	) {
		let actual_as_string: Vec<String> = nodes
			.map(|node| match node {
				Some(node) => node.text().to_string(),
				_ => String::from("None"),
			})
			.collect();

		assert_eq!(actual_as_string, expected);
	}

	fn assert_elements<T: IntoIterator<Item = SyntaxSeparatedElement<JsLanguage>>>(
		elements: T,
		expected: Vec<&str>,
	) {
		let actual_as_string: Vec<String> = elements
			.into_iter()
			.map(|element| {
				let mut expected_str = String::new();
				if let Some(node) = element.node {
					expected_str.push_str(node.text().to_string().as_str());
				}

				if let Some(separator) = element.separator {
					expected_str.push_str(separator.text());
				}

				expected_str
			})
			.collect();

		assert_eq!(actual_as_string, expected);
	}

	#[test]
	fn well_formed_list() {
		let list = create_separated_list(vec![
			Some(1),
			Some(2),
			Some(3),
			// trailing comma
			None,
		]);

		// ,,,
		assert_eq!(list.separators().count(), 3);
		assert_nodes(list.nodes(), vec!["1", "2", "3"]);
		assert_elements(list.iter(), vec!["1,", "2,", "3,"]);
		assert!(list.has_trailing_separator());
	}

	#[test]
	fn with_missing_element() {
		let list = create_separated_list(vec![Some(1), None, Some(3)]);

		assert_eq!(list.separators().count(), 2);
		assert_nodes(list.nodes(), vec!["1", "None", "3"]);
		assert_elements(list.iter(), vec!["1,", ",", "3"]);
	}

	#[test]
	fn with_missing_elements() {
		let list = create_separated_list(vec![None, None, None]);

		assert_eq!(list.separators().count(), 2);
		assert_nodes(list.nodes(), vec!["None", "None"]);
		assert_elements(list.iter(), vec![",", ","]);
	}

	#[test]
	fn no_trailing_comma() {
		let list = create_separated_list(vec![Some(1), Some(2)]);

		assert_eq!(list.clone().separators().count(), 1);
		assert!(!list.has_trailing_separator());
	}
}
