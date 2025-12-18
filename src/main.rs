use taffy::TaffyTree;
use windows::{
    UI::Colors,
    Win32::{Foundation::*, UI::WindowsAndMessaging::*},
    core::*,
};

use crate::{
    composition_host::CompositionHost,
    kit::{
        math::{Margin, SizePreference},
        renderer::{RenderContext, Renderer},
    },
    window::{Window, WindowOptions},
};

mod composition_host;
mod kit;
mod window;

fn main2() -> Result<()> {
    // unsafe {
    //     IsGUIThread(true).unwrap();
    // }

    let options = WindowOptions::builder().build();
    let mut window = Window::new(options).expect("Unable to create window");
    window.use_mica();

    let mut composition_host = CompositionHost::new(window.handle)?;
    composition_host.init_root().unwrap();

    let mut ctx = RenderContext::new(&composition_host.compositor);

    // let root_node = {
    //     let mut div = DivNode::new(&mut ctx);
    //     div.set_prefered_w(SizePreference::FillAvailable);
    //     div.set_prefered_h(SizePreference::FillAvailable);
    //     // div.set_background_color(Colors::Red().unwrap(), &mut ctx);

    //     let mut create_smth = || {
    //         let mut node = DivNode::new(&mut ctx);
    //         node.set_prefered_h(SizePreference::Fixed(64.0));
    //         node.set_prefered_w(SizePreference::Fixed(64.0));
    //         node.set_background_color(Colors::White().unwrap(), &mut ctx);
    //         node.set_border_width(1.0);
    //         node.set_border_color(Colors::LightGray().unwrap(), &mut ctx);
    //         node.set_corner_radius(8.0);
    //         node.set_margin(Margin::top(4.0));
    //         node
    //     };

    //     // div.add_children(create_smth());
    //     // div.add_children(create_smth());
    //     // div.add_children(create_smth());
    //     // div.add_children(create_smth());

    //     let section = {
    //         let mut node = DivNode::new(&mut ctx);
    //         // node.set_prefered_h(SizePreference::Default);
    //         node.set_id(Some("section".into()));
    //         node.set_prefered_w(SizePreference::FillAvailable);
    //         node.set_background_color(Colors::White().unwrap(), &mut ctx);
    //         node.set_border_width(1.0);
    //         node.set_border_color(Colors::LightGray().unwrap(), &mut ctx);
    //         node.set_corner_radius(16.0);
    //         node.set_margin(Margin::all(12.0));

    //         let input = {
    //             let mut node = DivNode::new(&mut ctx);
    //             node.set_id(Some("input".into()));
    //             node.set_prefered_w(SizePreference::FillAvailable);
    //             node.set_prefered_h(SizePreference::Fixed(36.0));
    //             node.set_background_color(Colors::LightGray().unwrap(), &mut ctx);
    //             node.set_corner_radius(12.0);
    //             node.set_border_width(1.0);
    //             node.set_border_color(Colors::Gray().unwrap(), &mut ctx);
    //             node.set_margin(Margin::all(24.0));

    //             node
    //         };

    //         node.add_children(input);

    //         node
    //     };

    //     div.add_children(section);

    //     div
    // };

    let (w, h) = window.size();
    // let mut renderer = Renderer::new(composition_host, Box::new(root_node), (w - 16.0, h - 30.0)); // wtf
    // renderer.update();

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

fn main() {
    use taffy::prelude::*;

    let mut tree: TaffyTree<()> = TaffyTree::new();

    // Create a tree of nodes using `TaffyTree.new_leaf` and `TaffyTree.new_with_children`.
    // These functions both return a node id which can be used to refer to that node
    // The Style struct is used to specify styling information
    let header_node = tree
        .new_leaf(Style {
            min_size: Size {
                width: percent(1.0),
                height: auto(),
            },
            flex_grow: 1.0,
            ..Default::default()
        })
        .unwrap();

    let mut style = Style {
        min_size: Size {
            width: percent(1.0),
            height: length(10.0),
        },
        flex_grow: 1.0,
        ..Default::default()
    };
    let body_node = tree.new_leaf(style.clone()).unwrap();

    let root_node = tree
        .new_with_children(
            Style {
                size: Size {
                    width: length(800.0),
                    height: length(600.0),
                },
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            &[header_node, body_node],
        )
        .unwrap();

    // Call compute_layout on the root of your tree to run the layout algorithm
    tree.compute_layout(root_node, Size::MAX_CONTENT).unwrap();
    tree.print_tree(root_node);

    style.flex_grow = 0.0;
    style.size.height = length(200.0);
    tree.set_style(body_node, style).unwrap();
    
    // tree.compute_layout(root_node, Size::MAX_CONTENT).unwrap();
    tree.print_tree(root_node);

    println!("root dirty? {}", tree.dirty(root_node).unwrap());
    println!("header dirty? {}", tree.dirty(header_node).unwrap());
}
