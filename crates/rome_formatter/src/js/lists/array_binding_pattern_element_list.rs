use crate::{
    join_elements, soft_line_break_or_space, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsArrayBindingPatternElementList;

impl ToFormatElement for JsArrayBindingPatternElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self.clone(), || token(","))?,
        ))
    }
}
