pub mod TextGenerator;
pub mod Font;
pub mod TextMesh;

#[repr(i32)]
pub enum TextAnchor {
    UpperLeft,
    UpperCenter,
    UpperRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    LowerLeft,
    LowerCenter,
    LowerRight,
}
impl TryFrom<i32> for TextAnchor {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, ()> {
        if value < 0 || value > 8 {
            return Err(());
        }
        Ok(unsafe { std::mem::transmute(value) })
    }
}

pub fn init() {
    get_assembly_image_or_return!(image, "UnityEngine.TextRenderingModule.dll");

    // PopulateWithErrors and TextMesh.set_text are translation/template hooks.
    // Font still provides a type object used by non-translation code.
    TextGenerator::init(image, install_translation_hook = false);
    Font::init(image);
    TextMesh::init(image, install_translation_hook = false);
}
