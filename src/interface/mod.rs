// mod composable;

// struct Signal<T> {
//     inner: T,
// }

// struct Props {}

// impl Default for Props {
//     fn default() -> Self {
//         Self {}
//     }
// }

// pub fn component(props: Props) {
//     // mutable state
//     //
// }

// struct Component {
//     count: Signal<u64>,
// }

// impl Component {
//     fn render(&mut self) {}

//     // provide default implementation that will warn you
//     // if you forgot to #[render_fn]
//     // we will `move` component state to this fn,
//     // insiede the runtime? boxed it?
//     // 
//     // but then || self.increment()
//     fn actually_render(&mut self, _runtime: ()) {}
// }
