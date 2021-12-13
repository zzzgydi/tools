pub(crate) mod all_caps;
pub(crate) mod double_eq;

use once_cell::sync::Lazy;

use crate::Analyzer;

pub fn all() -> impl Iterator<Item = &'static Analyzer> {
	ALL_ANALYZERS.iter()
}

pub fn diagnostic_providers() -> impl Iterator<Item = &'static Analyzer> {
	ALL_ANALYZERS
		.iter()
		.filter(|a| !a.diagnostic_categories.is_empty())
}

#[allow(unused)]
pub fn action_providers() -> impl Iterator<Item = &'static Analyzer> {
	ALL_ANALYZERS
		.iter()
		.filter(|a| !a.action_categories.is_empty())
}

static ALL_ANALYZERS: Lazy<Vec<Analyzer>> =
	Lazy::new(|| vec![all_caps::create(), double_eq::create()]);
