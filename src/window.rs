use windows::{
    Win32::{Foundation::*, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::*},
    core::*,
};

use bon::Builder;
type ProcFn = dyn FnMut(HWND, u32, WPARAM, LPARAM) -> Option<LRESULT>;

pub struct Window {
    pub handle: HWND,
}

struct WindowCallback {
    callback: Box<dyn FnMut(HWND, u32, WPARAM, LPARAM) -> Option<LRESULT>>,
}

#[derive(Builder)]
// pub struct WindowOptions<F: FnMut(HWND, u32, WPARAM, LPARAM) -> Option<LRESULT> + 'static> {
pub struct WindowOptions {
    #[builder(default = 400)]
    w: i32,
    #[builder(default = 300)]
    h: i32,
    // callback: F,
}

impl Window {
    pub fn new(options: WindowOptions) -> Result<Self> {
        unsafe {
            let class_name = w!("sdlklkfjidsfuih");

            let instance = GetModuleHandleW(None)?;
            let window_class = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance.into(),
                lpszClassName: class_name,

                style: CS_HREDRAW | CS_VREDRAW,

                lpfnWndProc: Some(Self::wnd_proc),
                ..Default::default()
            };

            let hres = RegisterClassW(&window_class);
            if hres == 0 {
                return Err(Error::from_thread());
            }

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                w!("asjhiusufdki"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                Some(instance.into()),
                None,
            )?;

            Ok(Self { handle: hwnd })
        }
    }

    pub fn run<F: FnMut(HWND, u32, WPARAM, LPARAM) -> Option<LRESULT> + 'static>(
        &mut self,
        callback: F,
    ) {
        unsafe {
            let callback = Box::new(WindowCallback {
                callback: Box::new(callback),
            });
            // Bruhhh
            let raw_ptr = Box::into_raw(callback);
            SetWindowLongPtrW(self.handle, GWLP_USERDATA, raw_ptr as isize);

            let mut message = MSG::default();
            while GetMessageW(&mut message, None, 0, 0).into() {
                _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }

    extern "system" fn wnd_proc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            let user_data = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if user_data != 0 {
                // bruh
                let p = &mut *(user_data as *mut WindowCallback);

                if let Some(result) = (p.callback)(hwnd, message, wparam, lparam) {
                    return result;
                }
            }

            match message {
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcW(hwnd, message, wparam, lparam),
            }
        }
    }
}
