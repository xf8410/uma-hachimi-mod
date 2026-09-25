pub mod Text;
pub mod CanvasScaler;
pub mod EventSystem;
pub mod LayoutElement;
pub mod LayoutRebuilder;
pub mod Image;
pub mod LayoutGroup;
pub mod VerticalLayoutGroup;
pub mod HorizontalOrVerticalLayoutGroup;
pub mod ContentSizeFitter;

pub fn init() {
    get_assembly_image_or_return!(image, "UnityEngine.UI.dll");

    // Resolve Text method addresses used by non-translation code, but do not
    // install its auto-translation set_text hook.
    Text::init(image, install_translation_hook = false);
    CanvasScaler::init(image);
    EventSystem::init(image);
    LayoutElement::init(image);
    LayoutRebuilder::init(image);
    Image::init(image);
    LayoutGroup::init(image);
    HorizontalOrVerticalLayoutGroup::init(image);
    ContentSizeFitter::init(image);
}
