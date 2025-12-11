use crate::{hook::pvz::widget_container::ORIGINAL_DRAW_ALL, pvz::graphics::graphics::Graphics};

#[repr(C)]
pub struct WidgetContainer;
pub struct ModalFlags;

pub extern "thiscall" fn DrawAll(
    this: *mut WidgetContainer,
    theFlags: *mut ModalFlags,
    g: *mut Graphics,
) {
    ORIGINAL_DRAW_ALL.wait()(this, theFlags, g);
}
