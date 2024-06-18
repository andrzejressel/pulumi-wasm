use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hint::unreachable_unchecked;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::rc::Rc;

use log::error;
use rmpv::Value;
use uuid::Uuid;

use crate::model::MaybeNodeValue::Set;
use crate::model::NodeValue::Exists;
use crate::model::{FunctionName, NodeValue, OutputId};
use crate::nodes;
use crate::nodes::{Callback, DoneNode, NativeFunctionNode};
use crate::ref_utils::Mappable;

enum EngineNode {
    Done(DoneNode),
    NativeFunction(NativeFunctionNode),
}

pub(crate) struct Holder<A: ?Sized>(Rc<RefCell<A>>);

impl<A> From<A> for Holder<A> {
    fn from(value: A) -> Self {
        Holder(Rc::new(RefCell::new(value)))
    }
}

impl<A: ?Sized> Holder<A> {
    fn map<U, F>(&self, f: F) -> Ref<U>
    where
        F: FnOnce(&A) -> &U,
    {
        Ref::map(self.get(), f)
    }

    fn map_mut<U, F>(&self, f: F) -> RefMut<U>
    where
        F: FnOnce(&mut A) -> &mut U,
    {
        RefMut::map(self.get_mut(), f)
    }

    fn foreach_mut<F>(&self, f: F)
    where
        F: FnOnce(&mut A),
    {
        RefMut::map(self.get_mut(), |x| {
            f(x);
            x
        });
    }

    fn get(&self) -> Ref<A> {
        // Ref::map(self.0.borrow(), |x| x)
        self.0.borrow()
    }

    #[inline(always)]
    fn get_mut(&self) -> RefMut<A> {
        self.0.borrow_mut()
    }
}

pub trait Swappable {
    type Target: ?Sized;

    fn swap(self) -> Self::Target;
}

type NodesMap = HashMap<OutputId, Holder<EngineNode>>;

struct Engine {
    done_node_ids: VecDeque<OutputId>,
    unknown_foreign_function_ids: HashSet<OutputId>,

    nodes: NodesMap,
}

impl Engine {
    fn new() -> Self {
        Self {
            done_node_ids: VecDeque::new(),
            unknown_foreign_function_ids: HashSet::new(),
            nodes: HashMap::new(),
        }
    }

    fn run(&mut self, native_function_results: HashMap<OutputId, Value>) {

        // loop {
            self.loop_over_dones();
            self.loop_over_native_function_results(native_function_results);
            // break;
        // }

    }

    fn loop_over_dones(&mut self) {
        loop {
            let output_id = match self.done_node_ids.pop_back() {
                None => break,
                Some(output_id) => output_id,
            };

            // let node = self.get_done(output_id);
            let node = Self::get_done_free(&self.nodes, output_id);

            let value = Exists(node.get_value());
            let callbacks = node.get_callbacks();

            for callback in callbacks {
                Engine::handle_callback(&value, callback, &self.nodes, &mut self.unknown_foreign_function_ids);
            }
        }
    }

    fn loop_over_native_function_results(&mut self, native_function_results: HashMap<OutputId, Value>) {
        // self.unknown_foreign_function_ids
        let nodes = &self.nodes;

        for (output_id, value) in native_function_results {
            let mut node = Self::get_native_function_free_mut(nodes, output_id);
            let node_value: NodeValue = value.into();
            node.set_value(node_value.clone());
        }

    }

    fn run_callbacks(callbacks: &Vec<Callback>, node_value: &NodeValue, nodes: &NodesMap, unknown_foreign_function_ids: &mut HashSet<OutputId>) {
        callbacks
            .iter()
            .for_each(|callback| Self::handle_callback(node_value, callback, nodes, unknown_foreign_function_ids));
    }

    fn handle_callback(value: &NodeValue, callback: &Callback, nodes: &NodesMap, unknown_foreign_function_ids: &mut HashSet<OutputId>) {
        match callback {
            Callback::CreateResource(_, _) => panic!("TEST"),
            Callback::ExtractField(_) => panic!("TEST2"),
            Callback::NativeFunction(output_id) => {
                let mut node = Self::get_native_function_free_mut(nodes, *output_id);
                node.set_argument(value.clone());
                unknown_foreign_function_ids.insert(*output_id);
            }
        }
    }

    fn get_done(&self, output_id: OutputId) -> Ref<DoneNode> {
        Self::get_done_free(&self.nodes, output_id)
    }

    fn get_done_free(nodes: &NodesMap, output_id: OutputId) -> Ref<DoneNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map(|t| match t {
                EngineNode::Done(node) => node,
                EngineNode::NativeFunction(_) => {
                    error!("Node with id [{}] is native function, not done", output_id);
                    panic!("Node with id [{}] is native function, not done", output_id)
                }
            }),
        }
    }

    fn get_done_mut(&self, output_id: OutputId) -> &mut DoneNode {
        panic!()
        //     match self.nodes.get(&output_id).map(|n| n.get_mut()) {
        //         Some(EngineNode::Done(node)) => node,
        //         None => {
        //             error!("Cannot find node with id {}", output_id);
        //             panic!("Cannot find node with id {}", output_id)
        //             // Maybe in the future?
        //             // unsafe { unreachable_unchecked() }
        //         }
        //         Some(EngineNode::NativeFunction(_)) => {
        //             error!("Node with id [{}] is native function, not done", output_id);
        //             panic!("Node with id [{}] is native function, not done", output_id)
        //         }
        //     }
    }

    fn get_native_function(&self, output_id: OutputId) -> Ref<NativeFunctionNode> {
        match self.nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map(|t| match t {
                EngineNode::NativeFunction(node) => node,
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not native function", output_id);
                    panic!("Node with id [{}] is done, not native function", output_id)
                }
            }),
        }
    }

    fn get_native_function_free(nodes: &NodesMap, output_id: OutputId) -> Ref<NativeFunctionNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map(|t| match t {
                EngineNode::NativeFunction(node) => node,
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not native function", output_id);
                    panic!("Node with id [{}] is done, not native function", output_id)
                }
            }),
        }
    }

    fn get_native_function_free_mut(nodes: &NodesMap, output_id: OutputId) -> RefMut<NativeFunctionNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map_mut(|t| match t {
                EngineNode::NativeFunction(node) => node,
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not native function", output_id);
                    panic!("Node with id [{}] is done, not native function", output_id)
                }
            }),
        }
    }

    fn add_callback(&self, output_id: OutputId, callback: Callback) {
        match self.nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => {
                r.foreach_mut(|t| {
                    match t {
                        EngineNode::Done(node) => node.add_callback(callback),
                        EngineNode::NativeFunction(node) => node.add_callback(callback),
                    };
                });
            }
        }
    }

    fn create_done_node(&mut self, value: Value) -> OutputId {
        let output_id = Uuid::now_v7().into();
        let node = DoneNode::new(value);
        self.done_node_ids.push_back(output_id);
        self.nodes.insert(output_id, EngineNode::Done(node).into());

        output_id
    }

    fn create_native_function_node(
        &mut self,
        function_name: FunctionName,
        source: OutputId,
    ) -> OutputId {
        let output_id = Uuid::now_v7().into();
        let node = NativeFunctionNode::new(function_name);
        let callback = Callback::native_function(output_id);
        self.unknown_foreign_function_ids.insert(output_id);
        self.nodes
            .insert(output_id, EngineNode::NativeFunction(node).into());
        self.add_callback(source, callback);
        output_id
    }
}

#[cfg(test)]
mod tests {
    use rmpv::Value;
    use std::collections::{HashMap, HashSet};
    use std::ops::Deref;
    use uuid::Uuid;

    use crate::engine::Engine;
    use crate::nodes::{Callback, DoneNode, NativeFunctionNode};

    #[test]
    fn create_done_node_create_node_in_map() {
        let mut engine = Engine::new();
        let value: Value = 1.into();
        let output_id = engine.create_done_node(value.clone());

        assert_eq!(engine.done_node_ids, vec![output_id]);
        assert_eq!(
            engine.get_done(output_id).deref(),
            &DoneNode::create(value, Vec::new())
        );
    }

    #[test]
    fn create_native_function_node_create_node_in_map() {
        let mut engine = Engine::new();
        let value: Value = 1.into();
        let done_node_output_id = engine.create_done_node(value.clone());
        let native_node_output_id =
            engine.create_native_function_node("func".into(), done_node_output_id);

        assert_eq!(engine.done_node_ids, vec![done_node_output_id]);
        assert_eq!(
            engine.unknown_foreign_function_ids,
            [native_node_output_id].into()
        );

        assert_eq!(
            engine.get_done(done_node_output_id).deref(),
            &DoneNode::create(
                value,
                vec![Callback::NativeFunction(native_node_output_id)],
            )
        );
        assert_eq!(
            engine.get_native_function(native_node_output_id).deref(),
            &NativeFunctionNode::new("func".into())
        );
    }

    #[test]
    fn run_return_native_functions() {
        let mut engine = Engine::new();
        let value: Value = 1.into();
        let done_node_output_id = engine.create_done_node(value.clone());
        let native_node_output_id =
            engine.create_native_function_node("func".into(), done_node_output_id);

        let result = engine.run(HashMap::new());
        assert_eq!(engine.unknown_foreign_function_ids, [native_node_output_id].into());
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
    static UUID_2: Uuid = Uuid::from_bytes([2; 16]);
    static UUID_3: Uuid = Uuid::from_bytes([3; 16]);
    static UUID_4: Uuid = Uuid::from_bytes([4; 16]);
    static UUID_5: Uuid = Uuid::from_bytes([5; 16]);
    static UUID_6: Uuid = Uuid::from_bytes([6; 16]);
}
