use windows::{
    Win32::{
        Foundation::*,
        Graphics::{
            Dwm::{
                DWMSBT_MAINWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DwmExtendFrameIntoClientArea,
                DwmSetWindowAttribute,
            },
            Gdi::{COLOR_WINDOW, COLOR_WINDOWFRAME, HBRUSH},
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::{Controls::MARGINS, WindowsAndMessaging::*},
    },
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
                hbrBackground: HBRUSH(COLOR_WINDOWFRAME.0 as _),

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

    pub fn size(&self) -> (f32, f32) {
        unsafe {
            let mut lprect = std::mem::zeroed();
            GetWindowRect(self.handle, &mut lprect).unwrap();
            (
                (lprect.right - lprect.top) as f32,
                (lprect.bottom - lprect.top) as f32,
            )
        }
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }

    pub fn use_mica(&self) {
        unsafe {
            let margins = MARGINS {
                cxLeftWidth: -1, // -1 means "Entire Window"
                cxRightWidth: -1,
                cyTopHeight: -1,
                cyBottomHeight: -1,
            };

            // This removes the black background and lets Mica show through
            _ = DwmExtendFrameIntoClientArea(self.handle, &margins);
            _ = DwmSetWindowAttribute(
                self.handle,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_MAINWINDOW as *const _ as _,
                std::mem::size_of::<i32>() as u32,
            );
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
