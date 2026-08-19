use glow::Context;
use khronos_egl as egl;
use khronos_egl::NativeDisplayType;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawWindowHandle, RawDisplayHandle};

/*
pub fn create_gl_context(window: &winit::window::Window) -> (Context, impl Fn() + 'static) {
    let handle = window.window_handle().unwrap().as_raw();
    let display_handle = window.display_handle().unwrap().as_raw();

    let  native_window = match handle {
        RawWindowHandle::Xlib(h) =>  h.window as egl::NativeWindowType,
        RawWindowHandle::Wayland(h) => h.surface as egl::NativeWindowType,
        _ => panic!("Unsupported Pi window handle")
    };

    let native_display = match display_handle {
        RawDisplayHandle::Xlib(d) => d.display as egl::NativeDisplayType,
        RawDisplayHandle::Wayland(d) => d.display as NativeDisplayType,
        _ => panic!("Unsupported Pi display handle")
    };

    let display = egl::


}
*/