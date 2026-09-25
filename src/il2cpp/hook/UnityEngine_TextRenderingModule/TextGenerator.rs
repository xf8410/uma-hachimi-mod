use std::ops::Not;

use crate::{core::{template, Hachimi}, il2cpp::{ext::{Il2CppStringExt, StringExt}, symbols::get_method_addr, types::*}};

type PopulateWithErrorsFn = extern "C" fn(
    this: *mut Il2CppObject, str: *mut Il2CppString,
    settings: TextGenerationSettings_t, context: *mut Il2CppObject
) -> bool;
extern "C" fn PopulateWithErrors(
    this: *mut Il2CppObject, str_: *mut Il2CppString,
    mut settings: TextGenerationSettings_t, context: *mut Il2CppObject
) -> bool {
    let orig_fn = get_orig_fn!(PopulateWithErrors, PopulateWithErrorsFn);
    let localized_data = &Hachimi::instance().localized_data.load();
    let hashed_dict = &localized_data.hashed_dict;
    let mut new_str: Option<&String> = None;
    let mut has_template: bool = false;
    let ld_str: String;
    let hashed_text = hashed_dict.is_empty().not()
        .then(|| hashed_dict.get(&unsafe { (*str_).hash() }))
        .flatten();
    if let Some(text) = hashed_text {
        new_str = hashed_text;
        has_template = text.contains('$');
    }
    else if !localized_data.localize_dict.is_empty() || !localized_data.text_data_dict.is_empty() {
        let utf_str = unsafe { (*str_).as_utf16str() };
        if utf_str.as_slice().contains(&36) {
            has_template = true;
            ld_str = utf_str.to_string();
            new_str = Some(&ld_str);
        }
    }
    if let Some(text) = new_str {
        if has_template {
            let mut template_context = TemplateContext { settings: &mut settings };
            let tpl_text = &Hachimi::instance().template_parser.eval_with_context(text, &mut template_context);
            orig_fn(this, tpl_text.to_il2cpp_string(), settings, context)
        }
        else {
            orig_fn(this, text.to_il2cpp_string(), settings, context)
        }
    }
    else {
        orig_fn(this, str_, settings, context)
    }
}

struct TemplateContext<'a> { settings: &'a mut TextGenerationSettings_t }
impl<'a> template::Context for TemplateContext<'a> {
    fn on_filter_eval(&mut self, _name: &str, _args: &[template::Token]) -> Option<String> {
        match _name {
            "nb" => {
                self.settings.horizontalOverflow = TextOverflow_Allow;
                self.settings.generateOutOfBounds = true;
                self.settings.resizeTextForBestFit = false;
            }
            "anchor" => {
                let value = _args.get(0)?;
                let template::Token::NumberLit(anchor_num) = *value else { return None; };
                let anchor = (anchor_num as i32) - 1;
                if anchor < 0 || anchor > 8 { return None; }
                self.settings.textAnchor = anchor;
            }
            "scale" => {
                let value = _args.get(0)?;
                let template::Token::NumberLit(percentage) = *value else { return None; };
                self.settings.fontSize = (self.settings.fontSize as f64 * (percentage / 100.0)) as i32;
                self.settings.resizeTextForBestFit = false;
            }
            "ho" => {
                let value = _args.get(0)?;
                let template::Token::NumberLit(overflow_num) = *value else { return None; };
                if overflow_num != 0 && overflow_num != 1 { return None; }
                self.settings.horizontalOverflow = overflow_num;
            }
            "vo" => {
                let value = _args.get(0)?;
                let template::Token::NumberLit(overflow_num) = *value else { return None; };
                if overflow_num != 0 && overflow_num != 1 { return None; }
                self.settings.verticalOverflow = overflow_num;
            }
            "ls" => {
                let value = _args.get(0)?;
                let template::Token::NumberLit(ls) = *value else { return None; };
                self.settings.lineSpacing = ls as f32;
            }
            "ub" => self.settings.updateBounds = true,
            "afit" => {
                let value = _args.get(0)?;
                let template::Token::NumberLit(state) = *value else { return None; };
                self.settings.resizeTextForBestFit = state != 0.0;
                if self.settings.resizeTextForBestFit {
                    self.settings.generateOutOfBounds = false;
                    self.settings.resizeTextMaxSize = self.settings.fontSize;
                    if let Some(template::Token::NumberLit(min_size)) = _args.get(1) {
                        if *min_size > 0.0 { self.settings.resizeTextMinSize = *min_size as _; }
                    }
                    if let Some(template::Token::NumberLit(max_size)) = _args.get(2) {
                        if *max_size > 0.0 { self.settings.resizeTextMaxSize = *max_size as _; }
                    }
                }
            }
            "minw" => {
                let value = _args.get(0)?;
                let template::Token::NumberLit(minwidth_num) = *value else { return None; };
                if *minwidth_num > self.settings.generationExtents.x { self.settings.generationExtents.x = *minwidth_num; }
            }
            "minh" => {
                let value = _args.get(0)?;
                let template::Token::NumberLit(minheight_num) = *value else { return None; };
                if *minheight_num > self.settings.generationExtents.y { self.settings.generationExtents.y = *minheight_num; }
            }
            _ => return None,
        }
        Some(String::new())
    }
}

pub struct IgnoreTGFiltersContext();
impl template::Context for IgnoreTGFiltersContext {
    fn on_filter_eval(&mut self, _name: &str, _args: &[template::Token]) -> Option<String> {
        match _name {
            "nb" | "anchor" | "scale" | "ho" | "vo" | "ls" | "ub" | "afit" | "minw" | "minh" => Some(String::new()),
            _ => None,
        }
    }
}

pub fn init(UnityEngine_TextRenderingModule: *const Il2CppImage, install_translation_hook: bool) {
    get_class_or_return!(UnityEngine_TextRenderingModule, UnityEngine, TextGenerator);
    if install_translation_hook {
        let PopulateWithErrors_addr = get_method_addr(TextGenerator, c"PopulateWithErrors", 3);
        new_hook!(PopulateWithErrors_addr, PopulateWithErrors);
    }
}
