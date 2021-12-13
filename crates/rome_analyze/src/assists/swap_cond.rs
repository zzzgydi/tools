use anyhow::Result;
use rslint_parser::{ast::JsBinaryExpression, AstNode};

use crate::{assist_context::AssistContext, Action, ActionCategory, Analysis, Signal, SyntaxEdit};

use super::AssistProvider;

pub fn create() -> AssistProvider {
	AssistProvider {
		name: "SwapCond",
		action_categories: vec![ActionCategory::Refactor],
		analyze,
	}
}

fn analyze(ctx: &AssistContext) -> Result<Analysis> {
	let signals = ctx
		.query_nodes_in_range::<JsBinaryExpression>(ctx.range())
		.filter_map(|node| {
			let lhs = node.left().ok()?;
			let rhs = node.right().ok()?;
			let edits = vec![
				SyntaxEdit::Replace {
					target: lhs.syntax().clone().into(),
					replacement: rhs.syntax().clone().into(),
				},
				SyntaxEdit::Replace {
					target: rhs.syntax().clone().into(),
					replacement: lhs.syntax().clone().into(),
				},
			];
			let signal = Signal::Action(Action {
				title: "rome: swap BinExp".into(),
				range: node.syntax().text_trimmed_range(),
				edits,
				category: ActionCategory::Refactor,
			});
			Some(signal)
		})
		.collect();
	Ok(Analysis { signals })
}
