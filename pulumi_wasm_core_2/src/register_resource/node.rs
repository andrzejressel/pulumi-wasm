use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use rmpv::Value;

use crate::node::MaybeNodeValue::Set;
use crate::node::{MaybeNodeValue, Node, NodeValue, RegisterResourceActionableNode};
use crate::pulumi::{FieldName, Pulumi, RegisterResourceRequest};
use crate::ActionableNode::RegisterResource;
use crate::{ActionableNode, OutputId};

pub(crate) struct RegisterResourceNode {
    output_id: OutputId,
    pulumi: Rc<dyn Pulumi>,
    name: String,
    r#type: String,
    required_inputs: HashSet<FieldName>,
    inputs: HashMap<FieldName, NodeValue>,
    outputs: HashMap<FieldName, msgpack_protobuf_converter::Type>,
    callbacks: Vec<Box<dyn Fn(NodeValue) -> Vec<ActionableNode>>>,
}

impl RegisterResourceNode {
    pub(crate) fn new(
        output_id: OutputId,
        pulumi: Rc<dyn Pulumi>,
        r#type: String,
        name: String,
        input_names: Vec<FieldName>,
        outputs: HashMap<FieldName, msgpack_protobuf_converter::Type>,
    ) -> Self {
        Self {
            output_id,
            pulumi,
            outputs,
            name,
            r#type,
            required_inputs: input_names.iter().cloned().collect(),
            inputs: HashMap::new(),
            callbacks: Vec::new(),
        }
    }

    pub(crate) fn set_input(&mut self, name: FieldName, value: NodeValue) -> Vec<ActionableNode> {
        if !self.required_inputs.contains(&name) {
            panic!("Input not found: {:?}", name);
        }
        self.required_inputs.remove(&name);
        self.inputs.insert(name, value);

        if self.required_inputs.is_empty() {
            self.send_to_pulumi()
        } else {
            vec![]
        }
    }

    fn send_to_pulumi(&self) -> Vec<ActionableNode> {
        let request = RegisterResourceRequest {
            r#type: self.r#type.clone(),
            name: self.name.clone(),
            object: self
                .inputs
                .iter()
                .map(|(name, nv)| match nv {
                    NodeValue::Nothing => (name.clone(), Value::Nil),
                    NodeValue::Exists(v) => (name.clone(), v.clone()),
                })
                .collect(),
            expected_results: self.outputs.clone(),
        };

        let register_id = self.pulumi.register_resource(request);

        vec![RegisterResource(RegisterResourceActionableNode::new(
            self.output_id.clone(),
            register_id,
        ))]
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

    mod lazy_node {
        use std::cell::OnceCell;
        use std::rc::Rc;

        use rmpv::Value::Nil;

        use crate::node::MaybeNodeValue::{NotYetCalculated, Set};
        use crate::node::Node;
        use crate::node::NodeValue::Exists;
        use crate::register_resource::node::LazyNode;
        use crate::ActionableNode;

        use super::*;

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

    mod register_resource_node {
        use std::collections::HashMap;
        use std::rc::Rc;

        use rmpv::Value::Nil;

        use crate::node::NodeValue::{Exists, Nothing};
        use crate::pulumi::MockPulumi;
        use crate::register_resource::node::RegisterResourceNode;
        use crate::{ActionableNode, RegisterId};

        use super::*;

        #[test]
        fn set_input_passes_it_to_pulumi() {
            let register_id = RegisterId::new("abc".to_string());
            let mut pulumi = MockPulumi::new();
            let register_id_2 = register_id.clone();
            pulumi
                .expect_register_resource()
                .once()
                .returning(move |_| register_id_2.clone());

            let mut node = RegisterResourceNode::new(
                OUTPUT_ID_1.clone(),
                Rc::new(pulumi),
                "type".into(),
                "name".into(),
                vec!["exists_nil".into(), "exists_int".into(), "not_exist".into()],
                HashMap::new(),
            );

            let result = node.set_input("exists_nil".into(), Exists(Nil));
            assert_eq!(result, vec![]);

            let result = node.set_input("exists_int".into(), Exists(2.into()));
            assert_eq!(result, vec![]);

            let result = node.set_input("not_exist".into(), Nothing);
            assert_eq!(
                result,
                vec![ActionableNode::new_register_crate(
                    OUTPUT_ID_1.clone(),
                    register_id
                )]
            );
        }
    }

    static OUTPUT_ID_1: Lazy<OutputId> = Lazy::new(|| OutputId::new("1".to_string()));
    static OUTPUT_ID_2: Lazy<OutputId> = Lazy::new(|| OutputId::new("2".to_string()));
    static OUTPUT_ID_3: Lazy<OutputId> = Lazy::new(|| OutputId::new("3".to_string()));
    static OUTPUT_ID_4: Lazy<OutputId> = Lazy::new(|| OutputId::new("4".to_string()));
    static OUTPUT_ID_5: Lazy<OutputId> = Lazy::new(|| OutputId::new("5".to_string()));
    static OUTPUT_ID_6: Lazy<OutputId> = Lazy::new(|| OutputId::new("6".to_string()));
}
