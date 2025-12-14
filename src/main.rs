use std::{cell::RefCell, rc::Rc};

use windows::{
    Win32::{Foundation::*, UI::WindowsAndMessaging::*},
    core::*,
};

use crate::{
    composition_host::CompositionHost,
    window::{Window, WindowOptions},
};

mod composition_host;
mod window;

fn main() -> Result<()> {
    let options = WindowOptions::builder().build();

    let mut window = Window::new(options).expect("Unable to create window");

    let mut composition_host = CompositionHost::new(window.handle)?;

    window.run(move |hwnd, message, _, _| {
        if message == WM_QUIT {
            composition_host.close();
        }
        None
    });

    println!("{}", Error::from_thread());

    Ok(())
}
