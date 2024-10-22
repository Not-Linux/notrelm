#![allow(dead_code)]

use gtk::{
    pango::ffi::PANGO_SCALE, 
    prelude::{DialogExtManual, GtkWindowExt, ImageExt, StyleContextExt, WidgetExt}, 
    IconSize, ResponseType,
};

use super::Size;

pub trait NotWidgetExt {
    fn set_request_size(&self, size: Size);

    fn set_classes(&self, classes: &[&str]);

    fn set_font_family(&self, family: &str);

    fn set_font_size(&self, size: u32);
}

impl<W: WidgetExt> NotWidgetExt for W {
    fn set_request_size(&self, size: Size) {
        self.set_size_request(size.width as i32, size.height as i32);
    }
    
    fn set_classes(&self, classes: &[&str]) {
        let ctx = self.style_context();

        for class in classes {
            ctx.add_class(class);
        }
    }

    fn set_font_family(&self, family: &str) {
        let ctx = self.pango_context();

        let mut font = ctx.font_description().unwrap_or_default();
        font.set_family(family);
        ctx.set_font_description(Some(&font));
    }
    
    fn set_font_size(&self, size: u32) {
        let ctx = self.pango_context();

        let mut font = ctx.font_description().unwrap_or_default();
        font.set_size(size.max(1) as i32 * PANGO_SCALE);
        ctx.set_font_description(Some(&font));
    }
}

pub trait NotWindowExt {
    fn set_size(&self, size: Size);
}

impl<W: GtkWindowExt> NotWindowExt for W {
    fn set_size(&self, size: Size) {
        self.set_default_size(size.width as i32, size.height as i32);
    }
}

pub trait NotImageExt {
    fn set_icon(&self, name_size: (&str, IconSize));
}

impl<I: ImageExt> NotImageExt for I {
    fn set_icon(&self, name_size: (&str, IconSize)) {
        self.set_from_icon_name(Some(name_size.0), name_size.1);
    }
}

pub trait NotDialogExt {
    fn set_buttons(&self, buttons: &[(&str, ResponseType)]);
}

impl<D: DialogExtManual> NotDialogExt for D {
    fn set_buttons(&self, buttons: &[(&str, ResponseType)]) {
        self.add_buttons(buttons);
    }
}