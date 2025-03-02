//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsModuleRef;
impl ToFormatElement for TsModuleRef {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsExternalModuleRef(node) => node.to_format_element(formatter),
            Self::TsAnyName(node) => node.to_format_element(formatter),
        }
    }
}
