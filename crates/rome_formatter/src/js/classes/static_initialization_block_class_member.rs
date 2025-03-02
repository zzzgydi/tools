use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    block_indent, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsStaticInitializationBlockClassMember;

impl ToFormatElement for JsStaticInitializationBlockClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let static_token = self.static_token().format(formatter)?;
        let separated = formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(block_indent(format_elements![
                    open_token_trailing,
                    formatter.format_list(self.statements()),
                    close_token_leading,
                ]))
            },
            &self.r_curly_token()?,
        )?;
        Ok(format_elements![static_token, space_token(), separated])
    }
}
