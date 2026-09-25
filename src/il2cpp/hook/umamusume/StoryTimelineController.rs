use std::sync::{atomic::{self, AtomicI32}, Mutex};

use crate::il2cpp::{symbols::{get_method_addr, GCHandle}, types::*};

static mut GET_ISFINISHED_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_IsFinished, GET_ISFINISHED_ADDR, bool, this: *mut Il2CppObject);

static mut GET_TIMELINEDATA_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_TimelineData, GET_TIMELINEDATA_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

/// Kept as an ABI-visible state surface for plugins. It is no longer populated
/// by Hachimi because the companion SO owns the GotoBlock hook.
pub static CURRENT: Mutex<Option<GCHandle>> = Mutex::new(None);
static LAST_BLOCK_ID: AtomicI32 = AtomicI32::new(-1);

pub fn last_block_id() -> i32 { LAST_BLOCK_ID.load(atomic::Ordering::Relaxed) }

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, StoryTimelineController);
    unsafe {
        GET_ISFINISHED_ADDR = get_method_addr(StoryTimelineController, c"get_IsFinished", 0);
        GET_TIMELINEDATA_ADDR = get_method_addr(StoryTimelineController, c"get_TimelineData", 0);
    }
    // Do not hook GotoBlock here. The companion 3.28.2 SO owns it.
}
