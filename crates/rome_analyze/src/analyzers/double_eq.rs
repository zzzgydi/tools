use anyhow::Result;
use rslint_parser::{ast::JsBinaryExpression, make, SyntaxToken};

use crate::{
	Action, ActionCategory, Analysis, AnalyzeDiagnostic, Analyzer, AnalyzerContext,
	DiagnosticCategory, SyntaxEdit,
};

pub fn create() -> Analyzer {
	Analyzer {
		name: "DoubleEq",
		diagnostic_categories: vec![DiagnosticCategory::NoDoubleEq],
		action_categories: vec![ActionCategory::Suggestion],
		analyze,
	}
}

fn analyze(ctx: &AnalyzerContext) -> Result<Analysis> {
	let mut signal = Analysis::default();

	for node in ctx.query_nodes::<JsBinaryExpression>().filter(|n| {
		n.operator()
			.ok()
			.filter(|op| op.text_trimmed() == "==")
			.is_some()
	}) {
		if let Ok(op) = node.operator() {
			let range = op.text_trimmed_range();
			let message = format!("rome: do not use == operator");
			let mut diag = AnalyzeDiagnostic::new(range, message, DiagnosticCategory::NoDoubleEq);

			let token: SyntaxToken = make::token_from_text(op.kind(), "===");

			let edits = vec![SyntaxEdit::Replace {
				target: node.operator().unwrap().into(),
				replacement: token.into(),
			}];
			let action = Action {
				title: "rome: Change to ===".into(),
				range,
				edits,
				category: ActionCategory::Suggestion,
			};

			diag.add_action(action);
			signal.add_diagnostic(diag);
		}
	}

	Ok(signal)
}
