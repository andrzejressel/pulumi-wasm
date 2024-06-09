use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use rmpv::Value;

use crate::node::{MaybeNodeValue, Node, NodeValue};
use crate::pulumi::{FieldName, Pulumi, RegisterResourceRequest};
use crate::{ActionableNode, OutputId};
use crate::node::MaybeNodeValue::Set;

struct RegisterResourceNode {
    pulumi: Rc<dyn Pulumi>,
    name: String,
    r#type: String,
    required_inputs: HashSet<String>,
    inputs: HashMap<String, NodeValue>,
    outputs: HashMap<FieldName, msgpack_protobuf_converter::Type>,
    callbacks: Vec<Box<dyn Fn(NodeValue) -> Vec<ActionableNode>>>,
}

impl RegisterResourceNode {
    fn new(
        pulumi: Rc<dyn Pulumi>,
        r#type: String,
        name: String,
        input_names: Vec<String>,
        outputs: HashMap<FieldName, msgpack_protobuf_converter::Type>,
    ) -> Self {
        Self {
            pulumi,
            outputs,
            name,
            r#type,
            required_inputs: input_names.iter().cloned().collect(),
            inputs: HashMap::new(),
            callbacks: Vec::new(),
        }
    }

    fn set_input(&mut self, name: String, value: NodeValue) -> Vec<ActionableNode> {
        if !self.required_inputs.contains(&name) {
            panic!("Input not found: {}", name);
        }
        self.required_inputs.remove(&name);
        self.inputs.insert(name, value);

        if self.required_inputs.is_empty() {
            // send
        }
        vec![]
    }
    
    fn send_to_pulumi(&self) -> Vec<ActionableNode> {
        
        let request = RegisterResourceRequest {
            r#type: self.r#type.clone(),
            name: self.name.clone(),
            object: self.inputs.iter().map(|(name, nv)| match nv {
                NodeValue::Nothing => (FieldName::new(name.clone()), Value::Nil),
                NodeValue::Exists(v) => (FieldName::new(name.clone()), v.clone()),
            }).collect(),
            expected_results: self.outputs.clone(),
        };
        
        self.pulumi.register_resource(request);
        
        vec![]
    }
}

struct LazyNode {
    id: OutputId,
    callbacks: Vec<Box<dyn Fn(NodeValue) -> Vec<ActionableNode>>>,
    value: MaybeNodeValue,
}

impl LazyNode {
    fn new(id: OutputId) -> Self {
        Self {
            id,
            callbacks: Vec::new(),
            value: MaybeNodeValue::NotYetCalculated,
        }
    }

    pub(crate) fn set_value(&mut self, value: NodeValue) -> Vec<ActionableNode> {
        self.value = Set(value.clone());
        self.callbacks
            .iter()
            .flat_map(|callback| callback(value.clone()))
            .collect()
    }
}

impl Node for LazyNode {
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

struct ExtractFieldNode {
    id: OutputId,
    callbacks: Vec<Box<dyn Fn(NodeValue) -> Vec<ActionableNode>>>,
    output_id: OutputId,
    field_name: FieldName,
    value: MaybeNodeValue,
}

impl Node for ExtractFieldNode {
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

impl ExtractFieldNode {
    fn new(id: OutputId, output_id: OutputId, field_name: FieldName) -> Self {
        Self {
            id,
            callbacks: Vec::new(),
            output_id,
            field_name,
            value: MaybeNodeValue::NotYetCalculated,
        }
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use crate::OutputId;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    mod register_resource_node {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }
    
    mod lazy_node {
        use super::*;
        use std::cell::OnceCell;
        use std::rc::Rc;
        use rmpv::Value::Nil;
        use crate::ActionableNode;
        use crate::node::MaybeNodeValue::{NotYetCalculated, Set};
        use crate::node::{NativeFunctionNode, Node};
        use crate::node::NodeValue::Exists;
        use crate::register_resource::node::LazyNode;

        #[test]
        fn value_is_empty_by_default() {
            let nfn = LazyNode::new(OUTPUT_ID_1.clone());
            assert_eq!(nfn.get_value(), &NotYetCalculated);
        }

        #[test]
        fn set_value_passes_it_downwards() {
            let cell = Rc::new(OnceCell::new());
            let mut nfn = LazyNode::new(OUTPUT_ID_1.clone());
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
