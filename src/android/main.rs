use std::{ffi::CStr, os::raw::c_void, sync::Arc, sync::atomic::Ordering};
use jni::{sys::jint, JavaVM};
use once_cell::sync::OnceCell;

use crate::core::{hachimi::{LocalizedData, TLAutoUpdaterMode}, Hachimi};

use super::{hook, plugin_loader};

#[allow(non_camel_case_types)]
type JniOnLoadFn = extern "C" fn(vm: JavaVM, reserved: *mut c_void) -> jint;

const LIBRARY_NAME: &CStr = c"libmain_orig.so";
const JNI_ONLOAD_NAME: &CStr = c"JNI_OnLoad";

static JAVA_VM: OnceCell<JavaVM> = OnceCell::new();

pub(crate) fn java_vm() -> Option<&'static JavaVM> {
    JAVA_VM.get()
}

#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: JavaVM, reserved: *mut c_void) -> jint {
    let orig_fn: JniOnLoadFn;
    unsafe {
        let handle = libc::dlopen(LIBRARY_NAME.as_ptr(), libc::RTLD_LAZY);
        orig_fn = std::mem::transmute(libc::dlsym(handle, JNI_ONLOAD_NAME.as_ptr()));
    }

    if !Hachimi::init() {
        return orig_fn(vm, reserved);
    }

    // The mod is an enhancement core, not a translation core. Enforce the
    // no-translation profile in memory after the ordinary config load, so an
    // old external config cannot re-enable text, texture, font, or updater
    // paths at runtime. This is intentionally not written back to the user's
    // config file.
    let hachimi = Hachimi::instance();
    let mut config = (**hachimi.config.load()).clone();
    config.disable_translations = true;
    config.translator_mode = false;
    config.auto_translate_stories = false;
    config.auto_translate_localize = false;
    config.apply_atlas_workaround = false;
    config.replace_to_builtin_font = false;
    config.disable_skill_name_translation = true;
    config.disable_factor_name_translation = true;
    config.skill_data_desc = false;
    config.selected_tl_repo_id = None;
    config.translation_repo_index = None;
    config.localized_data_dir = None;
    config.sugoi_url = None;
    config.tl_auto_updater_mode = TLAutoUpdaterMode::Disabled;
    config.tl_auto_updater_interval_sec = 0;
    config.disable_auto_update_check = true;
    config.lazy_translation_updates = false;
    config.etag_translation_updates = false;
    config.target_fps = Some(60);
    config.ui_scale = 1.0;
    config.gui_scale = 1.0;
    config.render_scale = 1.0;
    config.virtual_res_mult = 1.0;
    config.ui_animation_scale = 1.0;
    config.story_tcps_multiplier = 1.0;
    config.skip_first_time_setup = true;
    hachimi.config.store(Arc::new(config));
    hachimi.target_fps.store(60, Ordering::Release);
    hachimi.localized_data.store(Arc::new(LocalizedData::default()));
    info!("No-translation profile active: translations off, UI scale 1.0, target FPS 60");

    let vm_ptr = vm.get_java_vm_pointer();
    let _vm = JavaVM::from_raw(vm_ptr).unwrap();
    let _ = JAVA_VM.set(vm);
    let hachimi = Hachimi::instance();
    *hachimi.plugins.lock().unwrap() = plugin_loader::load_libraries();
    let env = _vm.get_env().unwrap();
    hook::init(env.get_raw());

    info!("JNI_OnLoad");
    let vm_for_orig = JavaVM::from_raw(vm_ptr).unwrap();
    orig_fn(vm_for_orig, reserved)
}
