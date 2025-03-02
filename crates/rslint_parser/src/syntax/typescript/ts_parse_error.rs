use crate::{
    parser::{expected_any, ToDiagnostic},
    Parser,
};
use rslint_errors::Diagnostic;
use std::ops::Range;

pub(crate) fn expected_ts_enum_member(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["identifier", "string literal", "computed name"], range).to_diagnostic(p)
}
