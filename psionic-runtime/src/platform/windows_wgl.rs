use glow::Context;
use raw_window_handle::{HasRawWindowHandle, HasWindowHandle, RawWindowHandle};
use std::ffi::{CString, c_void};
use std::{mem, ptr};
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::Graphics::OpenGL::*;
use windows_sys::Win32::System::LibraryLoader::*;

pub fn create_gl_context(window: &winit::window::Window) -> (Context, impl Fn() + 'static) {
    let handle = window.window_handle().unwrap().as_raw();

    let hwnd = match handle {
        RawWindowHandle::Win32(h) => h.hwnd.get() as HWND,
        _ => panic!("Unsupported Windows window handle"),
    };

    unsafe {
        let hdc = GetDC(hwnd);

        let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();

        pfd.nSize = mem::size_of::<PIXELFORMATDESCRIPTOR> as u16;

        pfd.nVersion = 1;
        pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
        pfd.iPixelType = PFD_TYPE_RGBA;
        pfd.cColorBits = 24;
        pfd.cDepthBits = 24;
        pfd.cStencilBits = 8;
        pfd.iLayerType = PFD_MAIN_PLANE as u8;

        let pf = ChoosePixelFormat(hdc, &pfd);
        SetPixelFormat(hdc, pf, &pfd);

        let hglrc = wglCreateContext(hdc);
        wglMakeCurrent(hdc, hglrc);

        let gl = Context::from_loader_function(|s| {
            let name = CString::new(s).unwrap();
            let ptr = wglGetProcAddress(s.as_ptr());

            match ptr {
                None => {
                    static mut GL_LIB: Option<HMODULE> = None;
                    if GL_LIB == None {
                        GL_LIB = Some(LoadLibraryA(b"opengl32.dll\0".as_ptr() as *const u8));
                    }
                    let core_ptr =  GetProcAddress(GL_LIB.unwrap(), s.as_ptr());

                    match core_ptr {
                        None => {}
                        Some(c_p) => return c_p as *const c_void
                    }
                }
                Some(ptr) => {
                    return ptr as *const c_void
                }
            }

            ptr::null()
        });

        let swap = move || {
            SwapBuffers(hdc);
        };

        (gl, swap)
    }
}
