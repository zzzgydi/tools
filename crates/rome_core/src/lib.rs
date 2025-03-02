use crate::file_handlers::unknown::UnknownFileHandler;
use crate::file_handlers::{javascript::JsFileHandler, ExtensionHandler, Language};
use file_handlers::json::JsonFileHandler;
use rome_path::RomePath;

pub mod file_handlers;

/// Features available for each language
struct Features {
    js: JsFileHandler,
    json: JsonFileHandler,
    unknown: UnknownFileHandler,
}

pub struct App {
    /// features available throughout the application
    features: Features,
}

impl Default for App {
    fn default() -> Self {
        Self {
            features: Features {
                js: JsFileHandler {},
                json: JsonFileHandler {},
                unknown: UnknownFileHandler::default(),
            },
        }
    }
}

impl App {
    pub fn new() -> Self {
        Default::default()
    }

    /// Return a [Language] from a string
    pub fn get_language<L: Into<Language>>(&self, file_extension: L) -> Language {
        file_extension.into()
    }

    /// Check if the current language is supported
    pub fn is_language_supported<L: Into<Language>>(&self, file_extension: L) -> bool {
        Language::Unknown != file_extension.into()
    }

    /// Return the features that are available for JavaScript
    pub fn get_js_features(&self) -> &JsFileHandler {
        &self.features.js
    }

    /// Return the features that are available for JSON
    pub fn get_json_features(&self) -> &JsonFileHandler {
        &self.features.json
    }

    /// Features available to a language that is not supported
    pub fn get_unknown_features(&self) -> &UnknownFileHandler {
        &self.features.unknown
    }

    /// Checks if the current file can be formatted
    pub fn can_format(&self, rome_path: &RomePath) -> bool {
        let language = self.get_language(rome_path.extension().expect("Could not read the file"));

        match language {
            Language::Js => self.features.js.capabilities().format,
            Language::Json => self.features.json.capabilities().format,
            Language::Unknown => self.features.unknown.capabilities().format,
        }
    }

    /// Checks if the current file can be analyzed for linting rules
    pub fn can_lint(&self, rome_path: &RomePath) -> bool {
        let language = self.get_language(rome_path.extension().expect("Could not read the file"));

        match language {
            Language::Js => self.features.js.capabilities().lint,
            Language::Json => self.features.json.capabilities().lint,
            Language::Unknown => self.features.unknown.capabilities().lint,
        }
    }
}
