use std::{mem, time::Duration};

use rand::random;
use windows::{
    UI::{Colors, Composition::ContainerVisual},
    Win32::{
        Foundation::*,
        Graphics::Gdi::{BeginPaint, EndPaint},
        UI::WindowsAndMessaging::*,
    },
    core::*,
};
use windows_numerics::{Vector2, Vector3};

use crate::{
    composition_host::CompositionHost,
    window::{Window, WindowOptions},
};

mod composition_host;
mod window;

fn insert_stuff(host: &mut CompositionHost, x: f32, y: f32) {
    let layers = host
        .target
        .Root()
        .unwrap()
        .cast::<ContainerVisual>()
        .unwrap()
        .Children()
        .unwrap();

    let layer = host.compositor.CreateSpriteVisual().unwrap();

    let element = host.compositor.CreateSpriteVisual().unwrap();
    let brush = host
        .compositor
        .CreateColorBrushWithColor(Colors::Red().unwrap())
        .unwrap();
    element.SetBrush(&brush).unwrap();
    element.SetSize(Vector2::new(64.0, 64.0)).unwrap();
    element.SetOffset(Vector3::new(x, y, 0.0)).unwrap();

    let animation = host.compositor.CreateVector3KeyFrameAnimation().unwrap();
    let bottom = 600f32 - element.Size().unwrap().Y;
    animation
        .InsertKeyFrame(1.0, Vector3::new(element.Offset().unwrap().X, bottom, 0.0))
        .unwrap();

    animation
        .SetDuration(Duration::from_secs(2).into())
        .unwrap();
    element
        .StartAnimation(&"Offset".into(), &animation)
        .unwrap();

    layers.InsertAtTop(&element).unwrap();

    layers.InsertAtTop(&layer).unwrap();
}

fn main() -> Result<()> {
    unsafe {
        IsGUIThread(true).unwrap();
    }

    let options = WindowOptions::builder().build();
    let mut window = Window::new(options).expect("Unable to create window");
    window.use_mica();

    let mut composition_host = CompositionHost::new(window.handle)?;
    composition_host.init_root().unwrap();

    insert_stuff(&mut composition_host, 0.0, 0.0);
    insert_stuff(&mut composition_host, 100.0, 0.0);
    insert_stuff(&mut composition_host, 200.0, 0.0);

    window.show();

    window.run(move |hwnd, message, wparam, lparam| match message {
        WM_QUIT => {
            composition_host.close();
            None
        }
        // WM_ERASEBKGND => {
        //     // Return 1 (true) to say "I handled it" (by doing nothing)
        //     Some(LRESULT(1))
        // }
        WM_KEYDOWN => {
            insert_stuff(&mut composition_host, random(), random());
            println!("Adding shit");
            None
        }
        _ => None,
    });

    println!("{}", Error::from_thread());

    Ok(())
}
