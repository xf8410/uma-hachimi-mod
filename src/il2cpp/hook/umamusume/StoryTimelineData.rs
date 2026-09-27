use crate::il2cpp::{
    symbols::{get_field_from_name, get_field_object_value, get_field_value, set_field_value},
    types::*,
};
use widestring::Utf16Str;

static mut CLASS: *mut Il2CppClass = std::ptr::null_mut();
pub fn class() -> *mut Il2CppClass { unsafe { CLASS } }

static mut TITLE_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_Title(this: *mut Il2CppObject) -> *mut Il2CppString {
    get_field_object_value(this, unsafe { TITLE_FIELD })
}

static mut BLOCKLIST_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_BlockList(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { BLOCKLIST_FIELD })
}

static mut TYPEWRITECOUNTPERSECOND_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_TypewriteCountPerSecond(this: *mut Il2CppObject) -> i32 {
    get_field_value(this, unsafe { TYPEWRITECOUNTPERSECOND_FIELD })
}

static mut LENGTH_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_Length(this: *mut Il2CppObject) -> i32 {
    get_field_value(this, unsafe { LENGTH_FIELD })
}
pub fn set_Length(this: *mut Il2CppObject, value: i32) {
    set_field_value(this, unsafe { LENGTH_FIELD }, &value);
}

/// Dormant callback retained so the old AssetBundle source still type-checks.
/// The no-translation build does not initialize AssetBundle, therefore this
/// function is never called.
pub fn on_LoadAsset(_bundle: *mut Il2CppObject, _this: *mut Il2CppObject, _name: &Utf16Str) {}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, StoryTimelineData);
    unsafe {
        CLASS = StoryTimelineData;
        TITLE_FIELD = get_field_from_name(StoryTimelineData, c"Title");
        BLOCKLIST_FIELD = get_field_from_name(StoryTimelineData, c"BlockList");
        TYPEWRITECOUNTPERSECOND_FIELD = get_field_from_name(StoryTimelineData, c"TypewriteCountPerSecond");
        LENGTH_FIELD = get_field_from_name(StoryTimelineData, c"Length");
    }
}
