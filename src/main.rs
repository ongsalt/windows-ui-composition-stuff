use taffy::prelude::*;
use windows::{
    Win32::{Foundation::*, UI::WindowsAndMessaging::*},
    core::*,
};

use crate::{
    composition_host::CompositionHost,
    kit::{
        attribute::Attribute,
        tree::Tree,
    },
    window::{Window, WindowOptions},
};

mod composition_host;
mod kit;
mod window;

fn main() -> Result<()> {
    let options = WindowOptions::builder().build();
    let mut window = Window::new(options).expect("Unable to create window");
    window.use_mica();

    let mut composition_host = CompositionHost::new(window.handle)?;
    composition_host.init_root().unwrap();

    let (w, h) = window.size();

    // we shuold pass in the size tho
    let mut tree = Tree::new(composition_host);
    let root = tree.create_root();

    tree.set_attribute(
        root,
        Attribute::Size(Size {
            height: length(h),
            width: length(w),
        }),
    );

    // let div = tree.new_div();
    // println!("{:.?}", tree.nodes);

    window.show();
    window.run(move |hwnd, message, wparam, lparam| match message {
        WM_QUIT => {
            // renderer.close();
            None
        }
        WM_SIZE => {
            let w = (lparam.0 & 0xffff) as u16 as f32;
            let h = (lparam.0 >> 16) as u16 as f32;
            // renderer.resize(w, h);
            tree.set_attribute(
                root,
                Attribute::Size(Size {
                    height: length(h),
                    width: length(w),
                }),
            );
            None
        }
        WM_KEYDOWN => {
            // insert_stuff(&mut composition_host, random::<f32>() * 2000.0, 0.0);
            // println!("Adding rect");
            None
        }
        _ => None,
    });

    println!("{}", Error::from_thread());

    Ok(())
}
