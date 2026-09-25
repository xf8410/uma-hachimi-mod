use crate::il2cpp::{
    symbols::{get_field_from_name, get_field_object_value, get_field_value, set_field_value},
    types::*,
};

/// Native story-timeline accessors for the no-translation build.
///
/// Translation JSON, automatic Sugoi requests, text replacement and duration
/// rewriting are intentionally not part of this module.  The 3-frame training
/// work will add its own opt-in observer/compressor on these accessors instead
/// of reusing the translation patcher.
static mut CLASS: *mut Il2CppClass = std::ptr::null_mut();
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

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
