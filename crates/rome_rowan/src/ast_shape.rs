use crate::api::RawLanguage;
use crate::Language;

pub trait AstTreeShape: Language {
	/// Verifies if the `children` fit the expected shape of `kind` node.
	fn fits_shape_of(
		kind: &Self::Kind,
		children_len: usize,
		children_kinds: impl Iterator<Item = Option<Self::Kind>>,
	) -> bool;
}

impl AstTreeShape for RawLanguage {
	fn fits_shape_of(
		_: &Self::Kind,
		_: usize,
		_: impl Iterator<Item = Option<Self::Kind>>,
	) -> bool {
		true
	}
}
