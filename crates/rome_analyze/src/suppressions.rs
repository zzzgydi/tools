//! A low-effort WIP implementation of suppressions.

use std::collections::HashMap;

use anyhow::Result;
use rslint_parser::{SyntaxNode, SyntaxToken, TextRange};

#[derive(Debug)]
pub struct Suppressions {
	pub suppressions: HashMap<String, Vec<TextRange>>,
}

impl Suppressions {
	pub fn ranges(&self, key: &str) -> Vec<TextRange> {
		match self.suppressions.get(key) {
			Some(r) => r.clone(),
			None => Vec::new(),
		}
	}

	pub fn match_range(&self, key: &str, range: TextRange) -> bool {
		match self.suppressions.get(key) {
			Some(ranges) => ranges.iter().any(|r| r.contains_range(range)),
			None => false,
		}
	}
}

pub fn compute(node: SyntaxNode) -> Result<Suppressions> {
	let mut suppressions: HashMap<String, Vec<TextRange>> = HashMap::new();

	for token in node.descendants_tokens() {
		for piece in token
			.leading_trivia()
			.pieces()
			// TODO: SyntaxTriviaPieceComments doesn't have text methods
			.filter(|p| p.as_comments().is_some())
		{
			let mut splits = piece.text().split_whitespace();

			if splits.nth(1) != Some("rome-ignore") {
				continue;
			}

			if let Some(range) = suppressed_range(&token) {
				for split in splits {
					match suppressions.get_mut(split) {
						Some(ranges) => ranges.push(range),
						None => {
							suppressions.insert(split.to_string(), vec![range]);
						}
					}
				}
			}
		}
	}
	Ok(Suppressions { suppressions })
}

// TODO: Improve this logic
fn suppressed_range(token: &SyntaxToken) -> Option<TextRange> {
	let mut node = token.parent()?;

	while !node.parent()?.text_trimmed().to_string().contains("\n") {
		node = node.parent()?;
	}
	Some(node.text_trimmed_range())
}
