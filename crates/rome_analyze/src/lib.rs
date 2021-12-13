// mod analysis;
mod analysis_api;
mod analyzer_context;
mod analyzers;
mod assist_context;
mod assists;
mod categories;
mod salsa_db;
mod suppressions;
mod syntax_ptr;

use anyhow::Result;
pub use categories::{ActionCategory, DiagnosticCategory};
use rslint_parser::{NodeOrToken, SyntaxDetachedElement, TextSize};
use rslint_parser::{SyntaxElement, TextRange};
pub use salsa_db::AnalyzerDatabase;
pub use salsa_db::RootDatabase;
pub use salsa_db::SourceDatabase;

pub use analysis_api::AnalysisAPI;
pub use analyzer_context::AnalyzerContext;
use syntax_ptr::SyntaxElementPtr;
#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

#[derive(Debug, Clone)]
pub struct AnalyzeDiagnostic {
	pub range: TextRange,
	pub message: String,
	pub category: DiagnosticCategory,
	pub actions: Vec<Action>,
}

impl AnalyzeDiagnostic {
	pub fn new(range: TextRange, message: String, category: DiagnosticCategory) -> Self {
		Self {
			range,
			message,
			category,
			actions: Vec::new(),
		}
	}

	pub fn with_actions(range: TextRange, message: String, category: DiagnosticCategory) -> Self {
		Self {
			range,
			message,
			category,
			actions: Vec::new(),
		}
	}

	pub fn add_action(&mut self, action: Action) {
		self.actions.push(action);
	}
}

#[derive(Debug, Clone)]
pub struct Action {
	pub title: String,
	pub range: TextRange,
	pub edits: Vec<SyntaxEdit>,
	// pub edits: Vec<Edit>,
	pub category: ActionCategory,
}

pub struct TextAction {
	pub title: String,
	pub target: TextRange,
	pub edits: Vec<Indel>,
	pub category: ActionCategory,
}

impl From<Action> for TextAction {
	fn from(a: Action) -> Self {
		let edits = a.edits.into_iter().map(Indel::from).collect();
		TextAction {
			title: a.title,
			target: a.range,
			edits,
			category: a.category,
		}
	}
}

impl From<SyntaxEdit> for Indel {
	fn from(edit: SyntaxEdit) -> Self {
		match edit {
			SyntaxEdit::Remove { target } => Indel {
				text: String::default(),
				range: target.text_trimmed_range(),
			},
			SyntaxEdit::Insert { offset, element } => {
				let text = match element {
					NodeOrToken::Node(it) => it.text_trimmed().into(),
					NodeOrToken::Token(it) => it.text_trimmed().into(),
				};
				let range = TextRange::new(offset, offset);
				Indel { text, range }
			}
			SyntaxEdit::Replace {
				target,
				replacement,
			} => {
				let text = match replacement {
					NodeOrToken::Node(it) => it.text_trimmed().into(),
					NodeOrToken::Token(it) => it.text_trimmed().into(),
				};
				let range = target.text_trimmed_range();
				Indel { text, range }
			}
		}
	}
}

/// A single insert or deletion of text.
pub struct Indel {
	pub text: String,
	pub range: TextRange,
}

#[derive(Debug, Clone)]
pub enum SyntaxEdit {
	Remove {
		target: SyntaxElement,
	},
	Insert {
		offset: TextSize,
		element: SyntaxElement,
	},
	Replace {
		target: SyntaxElement,
		replacement: SyntaxElement,
	},
}

/// This is a thread-safe version of SyntaxEdit but is still buggy.
#[derive(Debug, Clone)]
pub enum Edit {
	Remove {
		target: SyntaxElementPtr,
	},
	Insert {
		offset: TextSize,
		element: SyntaxDetachedElement,
	},
	Replace {
		target: SyntaxElementPtr,
		replacement: SyntaxDetachedElement,
	},
}

#[derive(Default, Debug)]
pub struct Analysis {
	pub signals: Vec<Signal>,
}

impl Analysis {
	pub fn into_diagnostics(self) -> impl Iterator<Item = AnalyzeDiagnostic> {
		self.signals.into_iter().filter_map(|s| match s {
			Signal::Diagnostic(it) => Some(it),
			Signal::Action(_) => None,
		})
	}

	pub fn iter_diagnostics<'a>(&'a self) -> impl Iterator<Item = &'a AnalyzeDiagnostic> {
		self.signals.iter().filter_map(|s| match s {
			Signal::Diagnostic(it) => Some(it),
			Signal::Action(_) => None,
		})
	}

	pub fn into_actions(self) -> impl Iterator<Item = Action> {
		self.signals
			.into_iter()
			// TODO: There must be a better way to do this.
			.map(|s| match s {
				Signal::Action(a) => vec![a].into_iter(),
				Signal::Diagnostic(d) => d.actions.into_iter(),
			})
			.flatten()
	}
}

impl From<Vec<Signal>> for Analysis {
	fn from(signals: Vec<Signal>) -> Self {
		Self { signals }
	}
}

impl Analysis {
	fn add_diagnostic(&mut self, d: AnalyzeDiagnostic) {
		self.signals.push(d.into());
	}

	#[allow(unused)]
	fn add_action(&mut self, a: Action) {
		self.signals.push(a.into());
	}
}

type AnalyzerResult = Result<Analysis>;

#[derive(Debug)]
pub enum Signal {
	Diagnostic(AnalyzeDiagnostic),
	Action(Action),
}

impl Signal {
	pub fn is_diagnostic(&self) -> bool {
		match self {
			Signal::Diagnostic(_) => true,
			_ => false,
		}
	}

	pub fn is_action(&self) -> bool {
		match self {
			Signal::Action(_) => true,
			_ => false,
		}
	}

	pub fn range(&self) -> TextRange {
		match self {
			Signal::Diagnostic(it) => it.range,
			Signal::Action(it) => it.range,
		}
	}
}

#[derive(Debug)]
pub struct DiagnosticWithActions {
	pub diagnostic: AnalyzeDiagnostic,
	pub actions: Vec<Action>,
}

impl From<AnalyzeDiagnostic> for Signal {
	fn from(d: AnalyzeDiagnostic) -> Self {
		Self::Diagnostic(d)
	}
}

impl From<Action> for Signal {
	fn from(a: Action) -> Self {
		Self::Action(a)
	}
}

pub struct Analyzer {
	pub name: &'static str,
	pub diagnostic_categories: Vec<DiagnosticCategory>,
	pub action_categories: Vec<ActionCategory>,
	analyze: fn(&AnalyzerContext) -> AnalyzerResult,
}

// struct AnalysisAggregate {
// 	analyses: HashMap<&'static str, Analysis>,
// 	suppressions: Option<Suppressions>,
// }
