use crate::model::MaybeNodeValue::{NotYetCalculated, Set};
use crate::model::{FieldName, FunctionName, MaybeNodeValue, NodeValue, OutputId};
use crate::pulumi::RegisterResourceRequest;
use log::error;
use rmpv::Value;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Callback {
    CreateResource(OutputId, FieldName),
    ExtractField(OutputId),
    NativeFunction(OutputId),
}

impl Callback {
    pub(crate) fn create_resource(output_id: OutputId, field_name: FieldName) -> Self {
        Self::CreateResource(output_id, field_name)
    }

    pub(crate) fn extract_field(output_id: OutputId) -> Self {
        Self::ExtractField(output_id)
    }

    pub(crate) fn native_function(output_id: OutputId) -> Self {
        Self::NativeFunction(output_id)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct DoneNode {
    value: NodeValue, // In reality Done have only Value, but being able to set Nothing is useful for testing
    callbacks: Vec<Callback>,
}

impl DoneNode {
    pub(crate) fn new(value: Value) -> Self {
        DoneNode::create(value, Vec::new())
    }

    pub(crate) fn create(value: Value, callbacks: Vec<Callback>) -> Self {
        Self {
            value: value.into(),
            callbacks,
        }
    }

    pub(crate) fn create_nothing() -> Self {
        Self {
            value: NodeValue::Nothing,
            callbacks: Vec::new(),
        }
    }

    pub(crate) fn get_value(&self) -> &NodeValue {
        &self.value
    }

    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct NativeFunctionNode {
    argument: MaybeNodeValue,
    value: MaybeNodeValue,
    function_name: FunctionName,
    callbacks: Vec<Callback>,
}

impl NativeFunctionNode {
    pub(crate) fn new(function_name: FunctionName) -> Self {
        Self {
            argument: MaybeNodeValue::NotYetCalculated,
            value: MaybeNodeValue::NotYetCalculated,
            function_name,
            callbacks: Vec::new(),
        }
    }

    pub(crate) fn get_argument_value(&self) -> &Value {
        match &self.argument {
            MaybeNodeValue::NotYetCalculated => {
                error!("Argument is not yet calculated");
                panic!("Argument is not yet calculated");
            }
            Set(NodeValue::Nothing) => {
                error!("Argument is Nothing");
                panic!("Argument is Nothing");
            }
            Set(NodeValue::Exists(value)) => value,
        }
    }

    pub(crate) fn get_value(&self) -> &MaybeNodeValue {
        &self.value
    }

    pub(crate) fn get_function_name(&self) -> &FunctionName {
        &self.function_name
    }

    pub(crate) fn set_argument(&mut self, value: NodeValue) {
        self.argument = MaybeNodeValue::Set(value);
    }

    pub(crate) fn set_value(&mut self, value: NodeValue) {
        self.value = MaybeNodeValue::Set(value);
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct RegisterResourceNode {
    value: MaybeNodeValue,
    name: String,
    r#type: String,
    required_inputs: HashSet<FieldName>,
    inputs: HashMap<FieldName, NodeValue>,
    outputs: HashMap<FieldName, msgpack_protobuf_converter::Type>,
    callbacks: Vec<Callback>,
}

impl RegisterResourceNode {
    pub(crate) fn new(
        r#type: String,
        name: String,
        input_names: Vec<FieldName>,
        outputs: HashMap<FieldName, msgpack_protobuf_converter::Type>,
    ) -> Self {
        Self {
            value: NotYetCalculated,
            outputs,
            name,
            r#type,
            required_inputs: input_names.iter().cloned().collect(),
            inputs: HashMap::new(),
            callbacks: Vec::new(),
        }
    }

    pub(crate) fn set_input(
        &mut self,
        name: FieldName,
        value: NodeValue,
    ) -> Option<RegisterResourceRequest> {
        if !self.required_inputs.contains(&name) {
            panic!("Input not found: {:?}", name);
        }
        self.required_inputs.remove(&name);
        self.inputs.insert(name, value);

        if self.required_inputs.is_empty() {
            Some(self.generate_request())
        } else {
            None
        }
    }

    fn generate_request(&self) -> RegisterResourceRequest {
        let mut object = HashMap::new();

        for (name, value) in self.inputs.iter() {
            match value {
                NodeValue::Nothing => {}
                NodeValue::Exists(v) => {
                    object.insert(name.clone(), v.clone());
                }
            };
        }

        RegisterResourceRequest {
            r#type: self.r#type.clone(),
            name: self.name.clone(),
            object,
            expected_results: self.outputs.clone(),
        }
    }

    fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ExtractFieldNode {
    value: MaybeNodeValue,
    field_name: FieldName,
    callbacks: Vec<Callback>,
}

impl ExtractFieldNode {
    pub(crate) fn create(
        value: MaybeNodeValue,
        field_name: FieldName,
        callbacks: Vec<Callback>,
    ) -> Self {
        Self {
            value,
            field_name,
            callbacks,
        }
    }

    pub(crate) fn new(field_name: FieldName) -> ExtractFieldNode {
        Self {
            value: MaybeNodeValue::NotYetCalculated,
            field_name,
            callbacks: Vec::new(),
        }
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub(crate) fn get_value(&self) -> &MaybeNodeValue {
        &self.value
    }

    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }

    // TODO: Write tests
    pub(crate) fn extract_field(&mut self, node_value: &NodeValue) -> NodeValue {
        match node_value {
            NodeValue::Nothing => {
                error!("Cannot extract field from Nothing");
                panic!("Cannot extract field from Nothing");
            }

            NodeValue::Exists(Value::Map(map)) => {
                let key: Value = self.field_name.as_string().clone().into();
                let value = map.iter().find(|(k, _)| k == &key).map(|(_, v)| v.clone());
                let new_node_value = match value {
                    None => NodeValue::Nothing,
                    Some(v) => NodeValue::Exists(v),
                };
                self.value = Set(new_node_value.clone());
                new_node_value
            }
            NodeValue::Exists(_) => {
                error!("Cannot extract field from non-Map");
                panic!("Cannot extract field from non-Map");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    mod register_resource_node {
        use std::collections::HashMap;

        use crate::model::NodeValue::{Exists, Nothing};
        use crate::nodes::RegisterResourceNode;
        use crate::pulumi::RegisterResourceRequest;
        use rmpv::Value::Nil;

        use super::*;

        #[test]
        fn set_input_passes_it_to_pulumi() {
            let mut node = RegisterResourceNode::new(
                "type".into(),
                "name".into(),
                vec!["exists_nil".into(), "exists_int".into(), "not_exist".into()],
                HashMap::new(),
            );

            let result = node.set_input("exists_nil".into(), Exists(Nil));
            assert_eq!(result, None);

            let result = node.set_input("exists_int".into(), Exists(2.into()));
            assert_eq!(result, None);

            let result = node.set_input("not_exist".into(), Nothing);
            assert_eq!(
                result,
                Some(RegisterResourceRequest {
                    r#type: "type".into(),
                    name: "name".into(),
                    object: HashMap::from([
                        ("exists_nil".into(), Nil),
                        ("exists_int".into(), 2.into()),
                    ]),
                    expected_results: HashMap::new(),
                })
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