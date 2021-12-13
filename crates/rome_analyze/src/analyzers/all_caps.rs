use anyhow::Result;
use rslint_parser::{ast, AstNode};

use crate::{Analysis, AnalyzeDiagnostic, Analyzer, AnalyzerContext, DiagnosticCategory, Signal};

pub fn create() -> Analyzer {
	Analyzer {
		name: "AllCaps",
		diagnostic_categories: vec![DiagnosticCategory::NoAllCaps],
		action_categories: vec![],
		analyze,
	}
}

fn analyze(ctx: &AnalyzerContext) -> Result<Analysis> {
	let signals = ctx
		.query_nodes::<ast::JsIdentifierBinding>()
		.filter(|n| n.text().to_uppercase() == n.text())
		.map(|n| -> Signal {
			let message = format!("Frome: the name {} is in all caps.", n.text());
			let range = n.syntax().text_trimmed_range();

			AnalyzeDiagnostic::new(range, message, DiagnosticCategory::NoAllCaps).into()
		})
		.collect();
	Ok(Analysis { signals })
}
