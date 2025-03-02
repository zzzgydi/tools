//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyAssignment;
impl ToFormatElement for JsAnyAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsIdentifierAssignment(node) => node.to_format_element(formatter),
            Self::JsStaticMemberAssignment(node) => node.to_format_element(formatter),
            Self::JsComputedMemberAssignment(node) => node.to_format_element(formatter),
            Self::JsParenthesizedAssignment(node) => node.to_format_element(formatter),
            Self::JsUnknownAssignment(node) => node.to_format_element(formatter),
        }
    }
}
