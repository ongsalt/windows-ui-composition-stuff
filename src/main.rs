use windows::{
    UI::Colors,
    Win32::{Foundation::*, UI::WindowsAndMessaging::*},
    core::*,
};

use crate::{
    composition_host::CompositionHost,
    kit::{
        math::{Margin, SizePreference},
        node::DivNode,
        renderer::{RenderContext, Renderer},
    },
    window::{Window, WindowOptions},
};

mod composition_host;
mod kit;
mod window;

fn main() -> Result<()> {
    // unsafe {
    //     IsGUIThread(true).unwrap();
    // }

    let options = WindowOptions::builder().build();
    let mut window = Window::new(options).expect("Unable to create window");
    window.use_mica();

    let mut composition_host = CompositionHost::new(window.handle)?;
    composition_host.init_root().unwrap();

    let mut ctx = RenderContext::new(&composition_host.compositor);

    let root_node = {
        let mut div = DivNode::new(&mut ctx);
        div.set_prefered_w(SizePreference::FillAvailable);
        div.set_prefered_h(SizePreference::FillAvailable);
        // div.set_background_color(Colors::Red().unwrap(), &mut ctx);

        let mut create_smth = || {
            let mut node = DivNode::new(&mut ctx);
            node.set_prefered_h(SizePreference::Fixed(64.0));
            node.set_prefered_w(SizePreference::Fixed(64.0));
            node.set_background_color(Colors::White().unwrap(), &mut ctx);
            node.set_border_width(1.0);
            node.set_border_color(Colors::LightGray().unwrap(), &mut ctx);
            node.set_corner_radius(8.0);
            node.set_margin(Margin::top(4.0));
            node
        };

        // div.add_children(create_smth());
        // div.add_children(create_smth());
        // div.add_children(create_smth());
        // div.add_children(create_smth());

        let section = {
            let mut node = DivNode::new(&mut ctx);
            // node.set_prefered_h(SizePreference::Default);
            node.set_id(Some("section".into()));
            node.set_prefered_w(SizePreference::FillAvailable);
            node.set_background_color(Colors::White().unwrap(), &mut ctx);
            node.set_border_width(1.0);
            node.set_border_color(Colors::LightGray().unwrap(), &mut ctx);
            node.set_corner_radius(16.0);
            node.set_margin(Margin::all(12.0));

            let input = {
                let mut node = DivNode::new(&mut ctx);
                node.set_id(Some("input".into()));
                node.set_prefered_w(SizePreference::FillAvailable);
                node.set_prefered_h(SizePreference::Fixed(24.0));
                node.set_background_color(Colors::LightGray().unwrap(), &mut ctx);
                node.set_corner_radius(8.0);
                node.set_border_width(1.0);
                node.set_border_color(Colors::Black().unwrap(), &mut ctx);
                node.set_margin(Margin::all(24.0));

                node
            };

            node.add_children(input);

            node
        };

        div.add_children(section);

        div
    };

    let (w, h) = window.size();
    let mut renderer = Renderer::new(composition_host, Box::new(root_node), (w - 16.0, h - 30.0)); // wtf
    renderer.update();

    window.show();

    window.run(move |hwnd, message, wparam, lparam| match message {
        WM_QUIT => {
            renderer.close();
            None
        }
        WM_SIZE => {
            let w = (lparam.0 & 0xffff) as u16 as f32;
            let h = (lparam.0 >> 16) as u16 as f32;
            renderer.resize(w, h);
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
