//! Incomplete experiment with enabling bidirectional one-to-many relationships between
//! diagnostics and code actions.

use crate::{Action, Diagnostic};

#[derive(Default)]
pub struct Analysis {
	diagnostics: Vec<Diagnostic>,
	actions: Vec<Action>,
	links: Vec<Link>,
}

impl Analysis {
	pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) -> DiagnosticId {
		self.diagnostics.push(diagnostic);
		DiagnosticId(self.diagnostics.len())
	}

	pub fn add_action(&mut self, action: Action) -> ActionId {
		self.actions.push(action);
		ActionId(self.actions.len())
	}
	pub fn add_diagnostic_with_action(
		&mut self,
		diagnostic: Diagnostic,
		action: Action,
	) -> (DiagnosticId, ActionId) {
		let diagnostic_id = self.add_diagnostic(diagnostic);
		let action_id = self.add_action(action);

		self.add_link(Link::diagnostic_to_action(diagnostic_id, action_id));

		(diagnostic_id, action_id)
	}

	pub fn add_diagnostic_with_actions(
		&mut self,
		diagnostic: Diagnostic,
		actions: impl Iterator<Item = Action>,
	) -> DiagnosticId {
		let diagnostic_id = self.add_diagnostic(diagnostic);
		for action in actions {
			let action_id = self.add_action(action);
			self.add_link(Link::diagnostic_to_action(diagnostic_id, action_id));
		}
		diagnostic_id
	}

	fn add_link(&mut self, link: Link) {
		self.links.push(link);
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticId(usize);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ActionId(usize);

enum AnalysisItem {}

enum Association {
	DiagnosticToAction,
	ActionToDiagnostic,
}

struct Link {
	association: Association,
	diagnostic_id: DiagnosticId,
	action_id: ActionId,
}

impl Link {
	fn diagnostic_to_action(diagnostic_id: DiagnosticId, action_id: ActionId) -> Self {
		Self {
			association: Association::DiagnosticToAction,
			diagnostic_id,
			action_id,
		}
	}

	fn action_to_diagnostic(action_id: ActionId, diagnostic_id: DiagnosticId) -> Self {
		Self {
			association: Association::ActionToDiagnostic,
			diagnostic_id,
			action_id,
		}
	}
}
