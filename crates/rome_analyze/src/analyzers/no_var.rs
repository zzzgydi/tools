use rslint_parser::{ast, AstNode};

use crate::{signals::DiagnosticExt, Analysis, Analyzer, AnalyzerContext};

pub fn create() -> Analyzer {
    Analyzer {
        name: "noVar",
        action_categories: vec![],
        analyze,
    }
}

fn analyze(ctx: &AnalyzerContext) -> Option<Analysis> {
    ctx.query_nodes::<ast::JsVariableDeclaration>()
        .filter(|n| n.is_var())
        .map(|n| ctx.error(n.range(), "Do not use var").into_signal())
        .collect()
}
