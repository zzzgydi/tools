use crate::{
    empty_element, format_elements, hard_line_break, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::{ast::JsDirectiveList, AstNodeList};

impl ToFormatElement for JsDirectiveList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if !self.is_empty() {
            Ok(format_elements![
                formatter.format_list(self.clone()),
                hard_line_break()
            ])
        } else {
            Ok(empty_element())
        }
    }
}
