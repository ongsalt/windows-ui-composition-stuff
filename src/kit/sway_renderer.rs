/*
this assume that host renderer has 
- comment node
- fragment

*/
pub trait SwayRenderer {
    type HostNode; // this is probably and opaque pointer or just an id
    type HostEvent;

    fn create_comment(&mut self, text: &str) -> Self::HostNode;
    fn create_element(&mut self, elemnet_type: &str) -> Self::HostNode;
    // i think static template parsing is not relavent in compiled language??

    fn append(&mut self, anchor: Self::HostNode, node: Self::HostNode);
    fn remove_node(&mut self, node: Self::HostNode);

    fn append_child(&mut self, parent: Self::HostNode, fragment: Self::HostNode);
    fn get_child(&mut self, parent: Self::HostNode, index: usize) -> Option<Self::HostNode>;
    // unused tho
    // fn getNextSibling(&mut self,node: Self::HostNode) -> Option<Self::HostNode>;

    // wtf am i doing
    fn set_attribute<T>(&mut self, element: Self::HostNode, key: &str, value: T);

    fn add_event_listener<Fn: FnMut(Self::HostEvent)>(
        &mut self,
        element: Self::HostNode,
        event_type: &str,
        callback: Fn,
    );
    fn remove_event_listener<Fn: FnMut(Self::HostEvent)>(
        &mut self,
        element: Self::HostNode,
        event_type: &str,
        callback: Fn,
    );
}

// TODO: fallback
// createStaticContent?(content: string) -> () => HostNode | HostNode[];
// very dom thing, should this be in setAttribute?

// TODO: think about this: the user should not mess with effect in this
// createBinding<T>(node: HostNode, key: string, valueProxy: ValueProxy<T>) -> void;
