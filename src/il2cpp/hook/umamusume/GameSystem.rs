use std::sync::Arc;
use crate::{
    core::{gui::{GameOpts, GAME_OPTS_CACHE}, Hachimi},
    il2cpp::{
        sql::{get_champions_resources, get_champions_live_max_year},
        symbols::{SingletonLike, get_method_addr},
        types::*,
        utils::umamusume_enum_options,
    },
};

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass { unsafe { CLASS } }
pub fn instance() -> *mut Il2CppObject {
    let Some(singleton) = SingletonLike::new(class()) else { return 0 as _; };
    singleton.instance()
}

static mut SOFTWARERESET_ADDR: usize = 0;
impl_addr_wrapper_fn!(SoftwareReset, SOFTWARERESET_ADDR, (), this: *mut Il2CppObject);

fn init_game_opts() {
    let opts = GameOpts {
        champions_resources: Arc::new(get_champions_resources()),
        champions_live_max_year: get_champions_live_max_year(),
        font_color_options: Arc::new(umamusume_enum_options(c"FontColorType")),
        outline_size_options: Arc::new(umamusume_enum_options(c"OutlineSizeType")),
        outline_color_options: Arc::new(umamusume_enum_options(c"OutlineColorType")),
    };
    match GAME_OPTS_CACHE.lock() {
        Ok(mut slot) => *slot = Some(opts),
        Err(poisoned) => {
            warn!("GAME_OPTS_CACHE mutex poisoned, recovering");
            *poisoned.into_inner() = Some(opts);
        }
    }
}

pub fn on_game_initialized() {
    Hachimi::instance().init_character_data();
    Hachimi::instance().init_skill_info();
    init_game_opts();

    #[cfg(target_os = "android")]
    crate::android::utils::set_audio_capture_policy_all();

    let hachimi = Hachimi::instance();
    let callbacks = hachimi.plugin_init_callbacks.lock().unwrap();
    for (callback, userdata) in callbacks.iter() {
        if *callback == 0 { continue; }
        let callback: unsafe extern "C" fn(*mut std::ffi::c_void) = unsafe { std::mem::transmute(*callback) };
        unsafe { callback(*userdata as *mut std::ffi::c_void); }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, GameSystem);
    unsafe {
        CLASS = GameSystem;
        SOFTWARERESET_ADDR = get_method_addr(GameSystem, c"SoftwareReset", 0);
    }
    // Do not hook InitializeGame, Update, or LateUpdate. The companion SO owns
    // those game-loop entry points in the coexistence build.
}
