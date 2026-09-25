#![allow(unused_upper_case_globals)]

macro_rules! new_hook {
    ($orig:ident, $hook:ident) => (
        let hachimi = crate::core::Hachimi::instance();
        if !hachimi.config.load().disabled_hooks.contains(stringify!($hook)) {
            info!("new_hook!: {}", stringify!($hook));
            if ($orig != 0) {
                let res = hachimi.interceptor.hook($orig as usize, $hook as *const () as usize);
                if let Err(e) = res {
                    error!("{}", e);
                }
            }
            else {
                error!("{} is null", stringify!($hook));
            }
        }
        else {
            info!("[DISABLED] new_hook!: {}", stringify!($hook));
        }
    );
}

macro_rules! get_assembly_image_or_return {
    ($var_name:ident, $assembly_name:tt) => {
        let $var_name = match crate::il2cpp::symbols::get_assembly_image(cstr!($assembly_name)) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
    }
}

macro_rules! get_class_or_return {
    ($image:ident, $namespace:tt, $class_name:ident) => {
        let $class_name = match crate::il2cpp::symbols::get_class($image, cstr!($namespace), cstr!($class_name)) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
    }
}

macro_rules! find_nested_class_or_return {
    ($parent:ident, $class_name:ident) => {
        let $class_name = match crate::il2cpp::symbols::find_nested_class($parent, cstr!($class_name)) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
    }
}

// shorter ver of making an addr wrapper function
macro_rules! def_method_wrapper_fn {
    ($name:tt, $addr:tt, $ret:ty, $($v:ident: $t:ty),*) => {
        static mut $addr: usize = 0;
        pub fn $name($($v: $t),*) -> $ret {
            let orig_fn: extern "C" fn($($v: $t),*) -> $ret = unsafe { std::mem::transmute($addr) };
            orig_fn($($v),*)
        }
    };
}

macro_rules! impl_addr_wrapper_fn {
    ($name:tt, $addr:tt, $ret:ty, $($v:ident: $t:ty),*) => {
        pub fn $name($($v: $t),*) -> $ret {
            let orig_fn: extern "C" fn($($v: $t),*) -> $ret = unsafe { std::mem::transmute($addr) };
            orig_fn($($v),*)
        }
    };
}

macro_rules! impl_enum_eq {
    ($ty:ty) => {
        impl PartialEq<$ty> for i32 {
            fn eq(&self, other: &$ty) -> bool { *self == *other as i32 }
        }
        impl PartialEq<i32> for $ty {
            fn eq(&self, other: &i32) -> bool { *self as i32 == *other }
        }
    };
}

macro_rules! impl_enum_ord {
    ($ty:ty) => {
        impl PartialOrd<$ty> for i32 {
            fn partial_cmp(&self, other: &$ty) -> Option<std::cmp::Ordering> {
                self.partial_cmp(&(*other as i32))
            }
        }
        impl PartialOrd<i32> for $ty {
            fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
                (*self as i32).partial_cmp(other)
            }
        }
    };
}

macro_rules! def_field_value_accessors {
    ($get_name:tt, $set_name:tt, $field:tt, $t:ty) => {
        static mut $field: *mut FieldInfo = 0 as _;
        pub fn $get_name(this: *mut Il2CppObject) -> $t {
            crate::il2cpp::symbols::get_field_value(this, unsafe { $field })
        }
        pub fn $set_name(this: *mut Il2CppObject, value: $t) {
            crate::il2cpp::symbols::set_field_value(this, unsafe { $field }, &value)
        }
    };
    (get $get_name:tt, $field:tt, $t:ty) => {
        static mut $field: *mut FieldInfo = 0 as _;
        pub fn $get_name(this: *mut Il2CppObject) -> $t {
            crate::il2cpp::symbols::get_field_value(this, unsafe { $field })
        }
    };
    (set $set_name:tt, $field:tt, $t:ty) => {
        static mut $field: *mut FieldInfo = 0 as _;
        pub fn $set_name(this: *mut Il2CppObject, value: $t) {
            crate::il2cpp::symbols::set_field_value(this, unsafe { $field }, &value)
        }
    };
}

macro_rules! def_field_object_accessors {
    ($get_name:tt, $set_name:tt, $field:tt, $t:ty) => {
        static mut $field: *mut FieldInfo = 0 as _;
        pub fn $get_name(this: *mut Il2CppObject) -> *mut $t {
            crate::il2cpp::symbols::get_field_object_value(this, unsafe { $field })
        }
        pub fn $set_name(this: *mut Il2CppObject, value: *mut $t) {
            crate::il2cpp::symbols::set_field_object_value(this, unsafe { $field }, value)
        }
    };
}

pub mod mscorlib;

pub mod UnityEngine_CoreModule;
pub mod UnityEngine_AssetBundleModule;
pub mod UnityEngine_TextRenderingModule;
pub mod UnityEngine_ImageConversionModule;

pub mod Unity_RenderPipelines_Universal_Runtime;
pub mod UnityEngine_UI;
pub mod UnityEngine_UIModule;
pub mod Unity_TextMeshPro;

#[cfg(target_os = "windows")]
pub mod UnityEngine_InputLegacyModule;
#[cfg(target_os = "windows")]
pub mod Unity_InputSystem;

pub mod Cute_UI_Assembly;
pub mod Cute_Cri_Assembly;
pub mod CriMw_CriWare_Runtime;
mod DOTween;

#[cfg(target_os = "android")]
mod Cute_Core_Assembly;

pub fn init() {
    info!("Initializing il2cpp hooks (no-translation build)");

    mscorlib::init();

    UnityEngine_CoreModule::init();
    UnityEngine_TextRenderingModule::init();
    UnityEngine_ImageConversionModule::init();
    Unity_RenderPipelines_Universal_Runtime::init();
    UnityEngine_UI::init();
    UnityEngine_UIModule::init();
    Unity_TextMeshPro::init();

    #[cfg(target_os = "windows")]
    {
        UnityEngine_InputLegacyModule::init();
        Unity_InputSystem::init();
    }

    // Translation-only asset, SQL and Plugins hook families are intentionally
    // not initialized in this build. Their source files remain temporarily so
    // upstream bug fixes stay easy to compare, but no hooks are installed.
    umamusume::init();
    Cute_Cri_Assembly::init();
    CriMw_CriWare_Runtime::init();
    DOTween::init();

    #[cfg(target_os = "android")]
    Cute_Core_Assembly::init();

    info!("Hooking finished (no-translation build)");
}
