use windows::System::DispatcherQueueController;
use windows::UI::Composition::Desktop::DesktopWindowTarget;
use windows::UI::Composition::*;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::WinRT::Composition::{ICompositorDesktopInterop, ICompositorInterop};
use windows::Win32::System::WinRT::{
    CreateDispatcherQueueController, DQTAT_COM_ASTA, DQTYPE_THREAD_CURRENT, DispatcherQueueOptions,
};
use windows::core::Interface;
use windows::core::Result;

pub struct CompositionHost {
    pub compositor: Compositor,
    pub target: DesktopWindowTarget,
    pub dispatcher_queue_controller: DispatcherQueueController,
}

impl CompositionHost {
    pub fn new(hwnd: HWND) -> Result<Self> {
        let options = DispatcherQueueOptions {
            dwSize: size_of::<DispatcherQueueOptions>() as _,
            apartmentType: DQTAT_COM_ASTA,
            threadType: DQTYPE_THREAD_CURRENT,
        };

        unsafe {
            let controller = CreateDispatcherQueueController(options)?;
            let compositor = Compositor::new()?;
            let interop: ICompositorDesktopInterop = compositor.cast()?;
            let target = interop.CreateDesktopWindowTarget(hwnd, false)?;

            Ok(Self {
                compositor,
                dispatcher_queue_controller: controller,
                target,
            })
        }
    }

    pub fn close(&mut self) {
        self.target.Close();
        self.dispatcher_queue_controller.ShutdownQueueAsync();
        self.compositor.Close();
    }
}

impl Drop for CompositionHost {
    fn drop(&mut self) {
        self.close()
    }
}
