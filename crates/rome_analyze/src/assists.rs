pub mod swap_cond;

use anyhow::Result;
use once_cell::sync::Lazy;

use crate::{assist_context::AssistContext, ActionCategory, Analysis};

pub struct AssistProvider {
	pub name: &'static str,
	pub action_categories: Vec<ActionCategory>,
	pub analyze: fn(&AssistContext) -> Result<Analysis>,
}
pub fn all() -> impl Iterator<Item = &'static AssistProvider> {
	ALL_ASSIST_PROVIDERS.iter()
}

static ALL_ASSIST_PROVIDERS: Lazy<Vec<AssistProvider>> = Lazy::new(|| vec![swap_cond::create()]);
