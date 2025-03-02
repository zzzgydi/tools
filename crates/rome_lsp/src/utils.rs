use std::collections::HashMap;

use crate::line_index::{LineCol, LineIndex};
use lspower::lsp::{self, CodeAction, CodeActionKind, Diagnostic, TextEdit, Url, WorkspaceEdit};
use rome_analyze::{DiagnosticExt, Indel, TextAction};
use tracing::trace;

use rslint_parser::{TextRange, TextSize};

pub(crate) fn position(line_index: &LineIndex, offset: TextSize) -> lsp::Position {
    let line_col = line_index.line_col(offset);
    lsp::Position::new(line_col.line, line_col.col)
}

pub(crate) fn range(line_index: &LineIndex, range: TextRange) -> lsp::Range {
    let start = position(line_index, range.start());
    let end = position(line_index, range.end());
    lsp::Range::new(start, end)
}

pub(crate) fn offset(line_index: &LineIndex, position: lsp::Position) -> TextSize {
    let line_col = LineCol {
        line: position.line as u32,
        col: position.character as u32,
    };
    line_index.offset(line_col)
}

pub(crate) fn text_range(line_index: &LineIndex, range: lsp::Range) -> TextRange {
    let start = offset(line_index, range.start);
    let end = offset(line_index, range.end);
    TextRange::new(start, end)
}

pub(crate) fn text_edit(line_index: &LineIndex, indel: &Indel) -> TextEdit {
    let text_range = indel.range;
    let range = range(line_index, text_range);
    let new_text = indel.text.clone();
    TextEdit { range, new_text }
}

pub(crate) fn text_action_to_lsp(
    action: &TextAction,
    line_index: &LineIndex,
    uri: Url,
    diagnostics: Option<Vec<Diagnostic>>,
) -> CodeAction {
    trace!("Action to LSP");
    let edits = action
        .edits
        .iter()
        .map(|r| text_edit(line_index, r))
        .collect();

    let mut text_edits = HashMap::new();
    text_edits.insert(uri, edits);
    let changes = Some(text_edits);
    let edit = WorkspaceEdit {
        changes,
        document_changes: None,
        change_annotations: None,
    };

    CodeAction {
        title: action.title.clone(),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics,
        edit: Some(edit),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    }
}

/// Convert an [rslint_errors::Diagnostic] to a [lsp::Diagnostic], using the span
/// of the diagnostic's primary label as the diagnostic range.
/// Requires a [LineIndex] to convert a byte offset range to the line/col range
/// expected by LSP.
pub(crate) fn diagnostic_to_lsp(
    diagnostic: rslint_errors::Diagnostic,
    line_index: &LineIndex,
) -> Option<lsp::Diagnostic> {
    let text_range = diagnostic.primary_text_range()?;
    let lsp_range = crate::utils::range(line_index, text_range);
    let message = diagnostic.title;
    let code = diagnostic.code.map(lspower::lsp::NumberOrString::String);
    let source = Some("rome".into());
    let diagnostic = Diagnostic::new(lsp_range, None, code, source, message, None, None);
    Some(diagnostic)
}
