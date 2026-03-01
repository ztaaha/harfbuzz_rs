use crate::{Font, Owned, bindings::hb_font_t};

extern "C" {
    fn hb_ft_font_create_referenced(face: freetype_sys::FT_Face) -> *mut hb_font_t;
}

impl<'a> Font<'a> {
    pub fn from_freetype_face(mut face: freetype::Face<&'a [u8]>) -> Owned<Font<'a>> {
        unsafe {
            let hb_font_ptr = hb_ft_font_create_referenced(face.raw_mut());
            Owned::from_raw(hb_font_ptr)
        }
    }
}