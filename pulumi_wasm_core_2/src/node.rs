use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use rmpv::Value;

use crate::node::MaybeNodeValue::{NotYetCalculated, Set};
use crate::node::NodeValue::Exists;
use crate::pulumi::FieldName;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct OutputId(pub(crate) String);
impl OutputId {
    pub(crate) const fn new(s: String) -> Self {
        Self(s)
    }
    pub(crate) fn generate_uuid() -> Self {
        let uuid = uuid::Uuid::new_v4();
        Self(uuid.to_string())
    }
}

enum InitiallyRunnableNode {
    Done(DoneNode),
}

#[derive(Debug, PartialEq)]
pub struct NativeFunctionActionableNode {
    output_id: OutputId,
    function_name: String,
    argument: Value,
}

impl NativeFunctionActionableNode {
    pub fn get_output_id(&self) -> &OutputId {
        &self.output_id
    }

    pub fn get_function_name(&self) -> &String {
        &self.function_name
    }

    pub fn get_argument(&self) -> &Value {
        &self.argument
    }
}

#[derive(Debug, PartialEq)]
pub enum ActionableNode {
    NativeFunction(NativeFunctionActionableNode),
}

impl ActionableNode {
    pub(crate) fn new_native_function(
        output_id: OutputId,
        function_name: String,
        argument: Value,
    ) -> ActionableNode {
        ActionableNode::NativeFunction(NativeFunctionActionableNode {
            output_id,
            function_name,
            argument,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MaybeNodeValue {
    NotYetCalculated,
    Set(NodeValue),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum NodeValue {
    Nothing, // preview
    Exists(Value),
}

pub(crate) trait Node {
    fn get_id(&self) -> &OutputId;
    fn add_callback(&mut self, callback: Box<dyn Fn(NodeValue) -> Vec<ActionableNode>>);
    fn get_value(&self) -> &MaybeNodeValue;
}

pub(crate) struct NativeFunctionNode {
    id: OutputId,
    callbacks: Vec<Box<dyn Fn(NodeValue) -> Vec<ActionableNode>>>,
    function_name: String,
    value: MaybeNodeValue,
}

impl Node for NativeFunctionNode {
    fn get_id(&self) -> &OutputId {
        &self.id
    }
    fn add_callback(&mut self, callback: Box<dyn Fn(NodeValue) -> Vec<ActionableNode>>) {
        self.callbacks.push(callback);
    }

    fn get_value(&self) -> &MaybeNodeValue {
        &self.value
    }
}

impl NativeFunctionNode {
    pub(crate) fn new(id: OutputId, function_name: String) -> Self {
        Self {
            id,
            callbacks: Vec::new(),
            function_name,
            value: NotYetCalculated,
        }
    }

    pub(crate) fn set_value(&mut self, value: NodeValue) -> Vec<ActionableNode> {
        self.value = Set(value.clone());
        self.callbacks
            .iter()
            .flat_map(|callback| callback(value.clone()))
            .collect()
    }

    pub(crate) fn set_argument_value(&mut self, value: NodeValue) -> Vec<ActionableNode> {
        match value {
            NodeValue::Nothing => {
                self.value = Set(NodeValue::Nothing);
                self.callbacks
                    .iter()
                    .flat_map(|callback| callback(value.clone()))
                    .collect()
            }
            Exists(v) => {
                vec![ActionableNode::new_native_function(
                    self.id.clone(),
                    self.function_name.clone(),
                    v.clone(),
                )]
            }
        }
    }
}

// struct CombinerNode {
//     set_elements: i8,
//     values: Vec<Option<Option<Value>>>,
//     callbacks: Vec<Box<dyn Fn(Value) -> Vec<ActionableNode>>>,
//     final_value: Option<Value>,
// }
//
// impl CombinerNode {
//     fn new(number_of_elements: i8) -> Self {
//         Self {
//             set_elements: 0,
//             values: vec![None; number_of_elements as usize],
//             callbacks: Vec::new(),
//             final_value: None
//         }
//     }
//
//     fn set_value(&mut self, itx: i8, value: Option<Value>) {
//         self.values[itx as usize] = Some(value);
//         self.set_elements += 1;
//     }
// }

// impl Node for CombinerNode {
//     fn add_callback(&mut self, callback: Box<dyn Fn(Value) -> Vec<ActionableNode>>) {
//         self.callbacks.push(callback);
//     }
//
//     fn get_value(&self) -> Option<&Value> {
//         self.final_value.as_ref()
//     }
// }
//
// impl CombinerNode {
//     fn add_value(&mut self, idx: i8, value: Option<Value>) {
//         self.values[idx as usize] = Some(value);
//         self.set_elements += 1;
//         // self.values.push(value);
//     }
// }

pub(crate) struct DoneNode {
    pub(crate) output_id: OutputId,
    callbacks: Vec<Box<dyn Fn(NodeValue) -> Vec<ActionableNode>>>,
    pub(crate) value: Value,
    maybe_value: MaybeNodeValue,
}

impl DoneNode {
    pub(crate) fn new(output_id: OutputId, value: Value) -> Self {
        Self {
            output_id,
            callbacks: Vec::new(),
            value: value.clone(),
            maybe_value: Set(Exists(value.clone())),
        }
    }
    pub(crate) fn run(&self) -> Vec<ActionableNode> {
        let mut result = Vec::new();
        for callback in self.callbacks.iter() {
            result.extend(callback(Exists(self.value.clone())));
        }
        result
    }
}

impl Node for DoneNode {
    fn get_id(&self) -> &OutputId {
        &self.output_id
    }

    fn add_callback(&mut self, callback: Box<dyn Fn(NodeValue) -> Vec<ActionableNode>>) {
        self.callbacks.push(callback);
    }

    fn get_value(&self) -> &MaybeNodeValue {
        &self.maybe_value
    }
}

// fn convert_option_ref<T>(opt: Option<&T>) -> &Option<T> {
//     let opt = So
// }

// static mut aaaa: Lazy<Arc<Mutex<HashMap<OutputId, Rc<Mutex<dyn Node>>>>>> = Lazy::new(|| {
//     Arc::new(Mutex::new(HashMap::<OutputId, Rc<Mutex<dyn Node>>>::new()))
// });

static mut aaaa: Lazy<Rc<Mutex<HashMap<OutputId, Rc<Mutex<dyn Node>>>>>> =
    Lazy::new(|| Rc::new(Mutex::new(HashMap::<OutputId, Rc<Mutex<dyn Node>>>::new())));

fn test() {

    // let mut h = HashMap::<OutputId, Rc<Mutex<dyn Node>>>::new();
    //
    // let mut deps = Graph::<&str, &str>::new();
    // let pg = deps.add_node("petgraph");
    // let fb = deps.add_node("fixedbitset");
    // let qc = deps.add_node("quickcheck");
    // let rand = deps.add_node("rand");
    // let libc = deps.add_node("libc");
    //
    // deps.add_edge(pg, fb, Default::default());
    //
    // let mut node = NativeFunctionNode::new("test_function".into());
    //
    // let combiner = CombinerNode {
    //     set_elements: 0,
    //     values: vec![],
    //     callbacks: vec![],
    //     final_value: None,
    // };
    //
    // let arc_lock = Rc::new(Mutex::new(combiner));
    // let tabc: Rc<Mutex<dyn Node>> = arc_lock.clone();
    //
    // let mut binding: MutexGuard<HashMap<OutputId, Rc<Mutex<dyn Node>>>> = unsafe {
    //     let raw = addr_of!(aaaa);
    //     let a =  &raw.as_ref().unwrap();
    //     let b = a.lock().unwrap();
    //     b
    // };
    // // let mut e = binding;
    // // e.insert()
    //
    //
    // let a1 = arc_lock.clone();
    // node.add_callback(Box::new(move |value| {
    //     let binding = a1.clone();
    //     let mut a = binding.lock().unwrap();
    //     a.add_value(0, Some(value));
    //     vec![]
    // }));
    //
    // let a2 = arc_lock.clone();
    // node.add_callback(Box::new(move |value| {
    //     let binding = a2.clone();
    //     let mut a = binding.lock().unwrap();
    //     a.add_value(1, Some(value));
    //     vec![]
    // }));
    //
    // binding.insert(OutputId("test".into()), arc_lock.clone());
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use crate::OutputId;

    mod native_function_node {
        use std::cell::OnceCell;
        use std::rc::Rc;

        use rmpv::Value::Nil;

        use crate::node::MaybeNodeValue::{NotYetCalculated, Set};
        use crate::node::NodeValue::{Exists, Nothing};
        use crate::node::{ActionableNode, MaybeNodeValue, NativeFunctionNode, Node};

        use super::*;

        #[test]
        fn value_is_empty_by_default() {
            let nfn = NativeFunctionNode::new(OUTPUT_ID_1.clone(), "function_name".into());
            assert_eq!(nfn.get_value(), &NotYetCalculated);
        }

        #[test]
        fn set_real_argument_value_return_self_output_id() {
            let nfn = NativeFunctionNode::new(OUTPUT_ID_1.clone(), "function_name".into());
            assert_eq!(nfn.get_value(), &NotYetCalculated);
        }

        #[test]
        fn set_nothing_argument_passes_it_downwards() {
            let cell = Rc::new(OnceCell::new());
            let mut nfn = NativeFunctionNode::new(OUTPUT_ID_1.clone(), "function_name".into());
            let cell_2 = cell.clone();
            nfn.add_callback(Box::new(move |node_value| {
                cell_2.set(Set(node_value.clone())).unwrap();
                vec![ActionableNode::new_native_function(
                    OUTPUT_ID_2.clone(),
                    "function".into(),
                    Nil,
                )]
            }));

            let result = nfn.set_argument_value(Nothing);
            let node_value: &MaybeNodeValue = cell.as_ref().get().unwrap();

            assert_eq!(
                result,
                vec![ActionableNode::new_native_function(
                    OUTPUT_ID_2.clone(),
                    "function".into(),
                    Nil
                )]
            );
            assert_eq!(node_value, &Set(Nothing));
            assert_eq!(nfn.get_value(), &Set(Nothing));
        }

        #[test]
        fn set_existing_argument_returns_id() {
            let cell = Rc::new(OnceCell::new());
            let mut nfn = NativeFunctionNode::new(OUTPUT_ID_1.clone(), "function_name".into());
            let cell_2 = cell.clone();
            nfn.add_callback(Box::new(move |node_value| {
                cell_2.set(Set(node_value.clone())).unwrap();
                vec![ActionableNode::new_native_function(
                    OUTPUT_ID_2.clone(),
                    "function".into(),
                    Nil,
                )]
            }));

            let result = nfn.set_argument_value(Exists(Nil));

            assert_eq!(
                result,
                vec![ActionableNode::new_native_function(
                    OUTPUT_ID_1.clone(),
                    "function_name".into(),
                    Nil
                )]
            );
            assert_eq!(cell.as_ref().get(), None);
            assert_eq!(nfn.get_value(), &NotYetCalculated);
        }

        #[test]
        fn set_value_passes_it_downwards() {
            let cell = Rc::new(OnceCell::new());
            let mut nfn = NativeFunctionNode::new(OUTPUT_ID_1.clone(), "function_name".into());
            let cell_2 = cell.clone();
            nfn.add_callback(Box::new(move |node_value| {
                cell_2.set(Set(node_value.clone())).unwrap();
                vec![ActionableNode::new_native_function(
                    OUTPUT_ID_2.clone(),
                    "function".into(),
                    Nil,
                )]
            }));

            let result = nfn.set_value(Exists(Nil));

            assert_eq!(
                result,
                vec![ActionableNode::new_native_function(
                    OUTPUT_ID_2.clone(),
                    "function".into(),
                    Nil
                )]
            );
            assert_eq!(cell.as_ref().get(), Some(Set(Exists(Nil))).as_ref());
            assert_eq!(nfn.get_value(), &Set(Exists(Nil)));
        }
    }

    static OUTPUT_ID_1: Lazy<OutputId> = Lazy::new(|| OutputId::new("1".to_string()));
    static OUTPUT_ID_2: Lazy<OutputId> = Lazy::new(|| OutputId::new("2".to_string()));
}
