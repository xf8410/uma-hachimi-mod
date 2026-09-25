use crate::{core::template, types::*};

/// Context used by translation code that explicitly ignores TextGenerator
/// filters. The no-translation build keeps the small type for source/API
/// compatibility but installs no TextGenerator hook.
pub struct IgnoreTGFiltersContext();

impl template::Context for IgnoreTGFiltersContext {
    fn on_filter_eval(&mut self, name: &str, _args: &[template::Token]) -> Option<String> {
        match name {
            "nb" | "anchor" | "scale" | "ho" | "vo" | "ls" | "ub" | "afit" | "minw" | "minh" => Some(String::new()),
            _ => None,
        }
    }
}

pub fn init(_UnityEngine_TextRenderingModule: *const Il2CppImage) {}
