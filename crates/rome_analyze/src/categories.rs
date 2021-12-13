#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[non_exhaustive]
pub enum DiagnosticCategory {
	NoDoubleEq,
	NoAllCaps,
	Misc,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ActionCategory {
	SafeFix,
	Suggestion,
	Refactor,
}
