use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hint::unreachable_unchecked;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::rc::Rc;

use log::error;
use rmpv::Value;
use uuid::Uuid;

use crate::model::NodeValue::Exists;
use crate::model::{FieldName, FunctionName, NodeValue, OutputId};
use crate::nodes::{
    Callback, DoneNode, ExtractFieldNode, NativeFunctionNode, RegisterResourceNode,
};

#[derive(Clone, Debug, PartialEq)]
struct ForeignFunctionToInvoke {
    output_id: OutputId,
    function_name: FunctionName,
    value: Value,
}

enum EngineNode {
    Done(DoneNode),
    NativeFunction(NativeFunctionNode),
    RegisterResource(RegisterResourceNode),
    ExtractField(ExtractFieldNode),
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
        self.0.borrow()
    }

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
    ready_foreign_function_ids: HashSet<OutputId>,
    register_resource_ids: HashSet<OutputId>,

    nodes: NodesMap,
}

impl Engine {
    fn new() -> Self {
        Self {
            done_node_ids: VecDeque::new(),
            ready_foreign_function_ids: HashSet::new(),
            register_resource_ids: HashSet::new(),
            nodes: HashMap::new(),
        }
    }

    fn run(
        &mut self,
        native_function_results: HashMap<OutputId, Value>,
    ) -> Option<Vec<ForeignFunctionToInvoke>> {
        self.loop_over_dones();
        self.loop_over_native_function_results(native_function_results);
        if !self.ready_foreign_function_ids.is_empty() {
            return Some(self.prepare_foreign_function_results());
        }

        None
    }

    fn prepare_foreign_function_results(&self) -> Vec<ForeignFunctionToInvoke> {
        self.ready_foreign_function_ids
            .iter()
            .map(|output_id| {
                let node = self.get_native_function(*output_id);
                ForeignFunctionToInvoke {
                    output_id: *output_id,
                    function_name: node.get_function_name().clone(),
                    value: node.get_argument_value().clone(),
                }
            })
            .collect()
    }

    fn loop_over_dones(&mut self) {
        loop {
            let output_id = match self.done_node_ids.pop_back() {
                None => break,
                Some(output_id) => output_id,
            };

            let node = Self::get_done_free(&self.nodes, output_id);

            let value = node.get_value();
            let callbacks = node.get_callbacks();

            for callback in callbacks {
                Engine::handle_callback(
                    value,
                    callback,
                    &self.nodes,
                    &mut self.ready_foreign_function_ids,
                );
            }
        }
    }

    fn loop_over_native_function_results(
        &mut self,
        native_function_results: HashMap<OutputId, Value>,
    ) {
        let nodes = &self.nodes;

        for (output_id, value) in native_function_results {
            let mut node = Self::get_native_function_free_mut(nodes, output_id);
            let node_value: NodeValue = value.into();
            node.set_value(node_value.clone());
            self.ready_foreign_function_ids.remove(&output_id);
            Self::run_callbacks(
                node.get_callbacks(),
                &node_value,
                nodes,
                &mut self.ready_foreign_function_ids,
            );
        }
    }

    fn run_callbacks(
        callbacks: &[Callback],
        node_value: &NodeValue,
        nodes: &NodesMap,
        unknown_foreign_function_ids: &mut HashSet<OutputId>,
    ) {
        callbacks.iter().for_each(|callback| {
            Self::handle_callback(node_value, callback, nodes, unknown_foreign_function_ids)
        });
    }

    fn handle_callback(
        value: &NodeValue,
        callback: &Callback,
        nodes: &NodesMap,
        unknown_foreign_function_ids: &mut HashSet<OutputId>,
    ) {
        match callback {
            Callback::CreateResource(_, _) => panic!("TEST"),
            Callback::ExtractField(output_id) => {
                Self::handle_extract_field_callback(
                    value,
                    nodes,
                    unknown_foreign_function_ids,
                    output_id,
                );
            }
            Callback::NativeFunction(output_id) => {
                Self::handle_native_function_callback(
                    value,
                    nodes,
                    unknown_foreign_function_ids,
                    output_id,
                );
            }
        }
    }

    fn handle_native_function_callback(
        value: &NodeValue,
        nodes: &NodesMap,
        unknown_foreign_function_ids: &mut HashSet<OutputId>,
        output_id: &OutputId,
    ) {
        let mut node = Self::get_native_function_free_mut(nodes, *output_id);
        node.set_argument(value.clone());
        match value {
            NodeValue::Nothing => {
                node.set_value(NodeValue::Nothing);
                Self::run_callbacks(
                    node.get_callbacks(),
                    value,
                    nodes,
                    unknown_foreign_function_ids,
                );
            }
            Exists(_) => {
                unknown_foreign_function_ids.insert(*output_id);
            }
        }
    }

    fn handle_extract_field_callback(
        value: &NodeValue,
        nodes: &NodesMap,
        unknown_foreign_function_ids: &mut HashSet<OutputId>,
        output_id: &OutputId,
    ) {
        let mut node = Self::get_extract_field_free_mut(nodes, *output_id);
        let new_value = node.extract_field(value);
        Self::run_callbacks(
            node.get_callbacks(),
            &new_value,
            nodes,
            unknown_foreign_function_ids,
        )
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
                EngineNode::RegisterResource(_) => {
                    error!(
                        "Node with id [{}] is register resource, not done",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is register resource, not done",
                        output_id
                    )
                }
                EngineNode::ExtractField(_) => {
                    error!("Node with id [{}] is extract field, not done", output_id);
                    panic!("Node with id [{}] is extract field, not done", output_id)
                }
            }),
        }
    }

    fn get_native_function(&self, output_id: OutputId) -> Ref<NativeFunctionNode> {
        Self::get_native_function_free(&self.nodes, output_id)
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
                EngineNode::RegisterResource(_) => {
                    error!(
                        "Node with id [{}] is register resource, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is register resource, not native function",
                        output_id
                    )
                }
                EngineNode::ExtractField(_) => {
                    error!(
                        "Node with id [{}] is extract field, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is extract field, not native function",
                        output_id
                    )
                }
            }),
        }
    }

    fn get_native_function_free_mut(
        nodes: &NodesMap,
        output_id: OutputId,
    ) -> RefMut<NativeFunctionNode> {
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
                EngineNode::RegisterResource(_) => {
                    error!(
                        "Node with id [{}] is register resource, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is register resource, not native function",
                        output_id
                    )
                }
                EngineNode::ExtractField(_) => {
                    error!(
                        "Node with id [{}] is extract field, not native function",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is extract field, not native function",
                        output_id
                    )
                }
            }),
        }
    }

    fn get_extract_field_mut(&self, output_id: OutputId) -> RefMut<ExtractFieldNode> {
        Self::get_extract_field_free_mut(&self.nodes, output_id)
    }

    fn get_extract_field_free_mut(
        nodes: &NodesMap,
        output_id: OutputId,
    ) -> RefMut<ExtractFieldNode> {
        match nodes.get(&output_id) {
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(r) => r.map_mut(|t| match t {
                EngineNode::ExtractField(node) => node,
                EngineNode::NativeFunction(_) => {
                    error!(
                        "Node with id [{}] is native function, not extract field",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is native function, not extract field",
                        output_id
                    )
                }
                EngineNode::Done(_) => {
                    error!("Node with id [{}] is done, not extract field", output_id);
                    panic!("Node with id [{}] is done, not extract field", output_id)
                }
                EngineNode::RegisterResource(_) => {
                    error!(
                        "Node with id [{}] is register resource, not extract field",
                        output_id
                    );
                    panic!(
                        "Node with id [{}] is register resource, not extract field",
                        output_id
                    )
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
                        EngineNode::RegisterResource(_) => {
                            panic!("TEST")
                        }
                        EngineNode::ExtractField(node) => node.add_callback(callback),
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
        self.nodes
            .insert(output_id, EngineNode::NativeFunction(node).into());
        self.add_callback(source, callback);
        output_id
    }

    fn create_extract_field(
        &mut self,
        field_name: FieldName,
        source_output_id: OutputId,
    ) -> OutputId {
        let output_id = Uuid::now_v7().into();
        let node = ExtractFieldNode::new(field_name);
        let callback = Callback::extract_field(output_id);
        self.nodes
            .insert(output_id, EngineNode::ExtractField(node).into());
        self.add_callback(source_output_id, callback);
        output_id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Deref;

    use rmpv::Value;
    use uuid::Uuid;

    use crate::engine::{Engine, ForeignFunctionToInvoke};
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
        assert_eq!(engine.ready_foreign_function_ids, [].into());

        assert_eq!(
            engine.get_done(done_node_output_id).deref(),
            &DoneNode::create(value, vec![Callback::NativeFunction(native_node_output_id)])
        );
        assert_eq!(
            engine.get_native_function(native_node_output_id).deref(),
            &NativeFunctionNode::new("func".into())
        );
    }

    mod native_function {
        use super::*;

        #[test]
        fn run_return_native_functions() {
            let mut engine = Engine::new();
            let value: Value = 1.into();
            let done_node_output_id = engine.create_done_node(value.clone());
            let native_node_output_id =
                engine.create_native_function_node("func".into(), done_node_output_id);

            let result = engine.run(HashMap::new());
            assert_eq!(
                engine.ready_foreign_function_ids,
                [native_node_output_id].into()
            );
            assert_eq!(
                result,
                Some(vec![ForeignFunctionToInvoke {
                    output_id: native_node_output_id,
                    function_name: "func".into(),
                    value: 1.into(),
                }])
            );
        }

        #[test]
        fn sets_native_function_results() {
            let mut engine = Engine::new();
            let value: Value = 1.into();
            let done_node_output_id = engine.create_done_node(value.clone());
            let native_node_output_id =
                engine.create_native_function_node("func".into(), done_node_output_id);
            let result_value: Value = 2.into();

            let result = engine.run(HashMap::from([(
                native_node_output_id,
                result_value.clone(),
            )]));

            assert_eq!(engine.ready_foreign_function_ids, [].into());
            assert_eq!(
                engine
                    .get_native_function(native_node_output_id)
                    .get_value(),
                &result_value.into()
            );
            assert_eq!(result, None);
        }

        #[test]
        fn native_function_passes_unknown_value_downstream() {
            let mut engine = Engine::new();
            let value: Value = 1.into();
            let done_node_output_id = engine.create_done_node(value.clone());
            let native_node_output_id =
                engine.create_native_function_node("func".into(), done_node_output_id);
            let result_value: Value = 2.into();

            let result = engine.run(HashMap::from([(
                native_node_output_id,
                result_value.clone(),
            )]));

            assert_eq!(engine.ready_foreign_function_ids, [].into());
            assert_eq!(
                engine
                    .get_native_function(native_node_output_id)
                    .get_value(),
                &result_value.into()
            );
            assert_eq!(result, None);
        }

        #[test]
        fn native_function_can_be_run_from_another_native_function() {
            let mut engine = Engine::new();
            let value: Value = 1.into();
            let done_node_output_id = engine.create_done_node(value.clone());
            let native_node_output_id_1 =
                engine.create_native_function_node("func".into(), done_node_output_id);
            let native_node_output_id_2 =
                engine.create_native_function_node("func2".into(), native_node_output_id_1);

            let result = engine.run(HashMap::from([(native_node_output_id_1, 2.into())]));

            assert_eq!(
                engine.ready_foreign_function_ids,
                [native_node_output_id_2].into()
            );
            assert_eq!(
                result,
                Some(vec![ForeignFunctionToInvoke {
                    output_id: native_node_output_id_2,
                    function_name: "func2".into(),
                    value: 2.into(),
                }])
            );
        }
    }

    mod extract_field {
        use super::*;
        use crate::model::MaybeNodeValue::Set;
        use crate::model::NodeValue::Exists;
        use crate::nodes::Callback::NativeFunction;
        use crate::nodes::ExtractFieldNode;

        #[test]
        fn extract_field_extract_field_from_map() {
            let mut engine = Engine::new();
            let value = Value::Map(vec![("key".into(), 1.into())]);
            let done_node_output_id = engine.create_done_node(value.clone());
            let extract_field_node_output_id =
                engine.create_extract_field("key".into(), done_node_output_id);
            let native_function_node_output_id =
                engine.create_native_function_node("func".into(), extract_field_node_output_id);

            let result = engine.run(HashMap::new());

            assert_eq!(
                engine.ready_foreign_function_ids,
                [native_function_node_output_id].into()
            );
            assert_eq!(
                result,
                Some(vec![ForeignFunctionToInvoke {
                    output_id: native_function_node_output_id,
                    function_name: "func".into(),
                    value: 1.into(),
                }])
            );
            assert_eq!(
                engine
                    .get_extract_field_mut(extract_field_node_output_id)
                    .deref(),
                &ExtractFieldNode::create(
                    Set(Exists(1.into())),
                    "key".into(),
                    vec![NativeFunction(native_function_node_output_id)]
                )
            );
        }
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
    static UUID_2: Uuid = Uuid::from_bytes([2; 16]);
    static UUID_3: Uuid = Uuid::from_bytes([3; 16]);
    static UUID_4: Uuid = Uuid::from_bytes([4; 16]);
    static UUID_5: Uuid = Uuid::from_bytes([5; 16]);
    static UUID_6: Uuid = Uuid::from_bytes([6; 16]);
}