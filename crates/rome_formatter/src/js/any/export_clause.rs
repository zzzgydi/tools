//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyExportClause;
impl ToFormatElement for JsAnyExportClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsExportFunctionClause(node) => node.to_format_element(formatter),
            Self::JsExportDefaultFunctionClause(node) => node.to_format_element(formatter),
            Self::JsExportClassClause(node) => node.to_format_element(formatter),
            Self::JsExportDefaultClassClause(node) => node.to_format_element(formatter),
            Self::JsExportVariableClause(node) => node.to_format_element(formatter),
            Self::JsExportDefaultExpressionClause(node) => node.to_format_element(formatter),
            Self::JsExportNamedClause(node) => node.to_format_element(formatter),
            Self::JsExportFromClause(node) => node.to_format_element(formatter),
            Self::JsExportNamedFromClause(node) => node.to_format_element(formatter),
        }
    }
}
