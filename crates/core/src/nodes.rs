use crate::model::MaybeNodeValue::{NotYetCalculated, Set};
use crate::model::NodeValue::Nothing;
use crate::model::{FieldName, FunctionName, MaybeNodeValue, NodeValue, OutputId};
use crate::pulumi::service::{PerformResourceRequest, RegisterResourceResponse};
use log::error;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Callback {
    CreateResource(OutputId, FieldName),
    ExtractField(OutputId),
    NativeFunction(OutputId),
    CombineOutputs(OutputId, u32),
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
    pub(crate) fn combine_outputs(output_id: OutputId, count: u32) -> Self {
        Self::CombineOutputs(output_id, count)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct DoneNode {
    value: NodeValue, // In reality Done have only Value, but being able to set Nothing is useful for testing
    callbacks: Vec<Callback>,
}

impl DoneNode {
    pub(crate) fn create(value: Value, secret: bool, callbacks: Vec<Callback>) -> Self {
        Self {
            value: NodeValue::exists(value, secret),
            callbacks,
        }
    }
    pub(crate) fn new(value: Value, secret: bool) -> Self {
        DoneNode::create(value, secret, Vec::new())
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
            argument: NotYetCalculated,
            value: NotYetCalculated,
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
            Set(NodeValue::Exists { value, secret: _ }) => value,
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

    // Add test that checks if output is secret
    pub(crate) fn set_value(&mut self, value: Value) -> NodeValue {
        let node_value = NodeValue::Exists {
            value,
            secret: self.is_secret(),
        };
        self.value = MaybeNodeValue::Set(node_value.clone());
        node_value
    }

    pub(crate) fn set_nothing(&mut self) {
        self.value = MaybeNodeValue::Set(NodeValue::Nothing);
    }

    fn is_secret(&self) -> bool {
        match &self.argument {
            MaybeNodeValue::NotYetCalculated => {
                error!("Argument is not yet calculated");
                panic!("Argument is not yet calculated");
            }
            Set(NodeValue::Nothing) => false,
            Set(NodeValue::Exists { value: _, secret }) => *secret,
        }
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RegisterResourceRequestOperation {
    pub(crate) name: String,
    pub(crate) r#type: String,
}

impl RegisterResourceRequestOperation {
    pub(crate) fn new(r#type: String, name: String) -> Self {
        Self { name, r#type }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ResourceInvokeRequestOperation {
    pub(crate) token: String,
}
impl ResourceInvokeRequestOperation {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ResourceRequestOperation {
    Register(RegisterResourceRequestOperation),
    Invoke(ResourceInvokeRequestOperation),
}

#[derive(Debug, PartialEq)]
pub(crate) struct AbstractResourceNode {
    value: MaybeNodeValue,
    required_inputs: HashSet<FieldName>,
    inputs: HashMap<FieldName, NodeValue>,
    callbacks: Vec<Callback>,
    operation: ResourceRequestOperation,
    version: String,
}

impl AbstractResourceNode {
    pub(crate) fn create(
        value: MaybeNodeValue,
        operation: ResourceRequestOperation,
        required_inputs: HashSet<FieldName>,
        inputs: HashMap<FieldName, NodeValue>,
        callbacks: Vec<Callback>,
        version: String,
    ) -> Self {
        Self {
            value,
            operation,
            required_inputs,
            inputs,
            callbacks,
            version,
        }
    }

    pub(crate) fn new(
        operation: ResourceRequestOperation,
        input_names: HashSet<FieldName>,
        version: String,
    ) -> Self {
        Self::create(
            NotYetCalculated,
            operation,
            input_names,
            HashMap::new(),
            Vec::new(),
            version,
        )
    }

    pub(crate) fn set_input(
        &mut self,
        name: FieldName,
        value: NodeValue,
    ) -> Option<PerformResourceRequest> {
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

    pub(crate) fn set_value(&mut self, value: &RegisterResourceResponse) -> NodeValue {
        let map: HashMap<String, Value> = value
            .outputs
            .iter()
            .map(|(k, v)| (k.as_string().clone(), v.clone()))
            .collect();
        let val = Value::Object(map.into_iter().collect());
        // Secret value does not matter - true value will be resolved in [ExtractFieldNode]
        let node_value = NodeValue::exists(val, false);

        self.value = Set(node_value.clone());
        node_value
    }

    pub(crate) fn get_value(&self) -> &MaybeNodeValue {
        &self.value
    }

    fn generate_request(&self) -> PerformResourceRequest {
        let mut object = HashMap::new();

        for (name, value) in self.inputs.iter() {
            match value {
                NodeValue::Nothing => {
                    object.insert(name.clone(), None);
                }
                NodeValue::Exists { value, secret } => {
                    let mut value = value.clone();
                    if *secret {
                        value = json!({
                            crate::constants::SPECIAL_SIG_KEY: crate::constants::SPECIAL_SECRET_SIG,
                            crate::constants::SECRET_VALUE_NAME: value,
                        });
                    }
                    object.insert(name.clone(), Some(value.clone()));
                }
            };
        }

        PerformResourceRequest {
            operation: self.operation.clone(),
            object,
            version: self.version.clone(),
        }
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ExtractFieldNode {
    value: MaybeNodeValue,
    field_name: FieldName,
    callbacks: Vec<Callback>,
    in_preview: bool,
}

impl ExtractFieldNode {
    pub(crate) fn create(
        value: MaybeNodeValue,
        field_name: FieldName,
        callbacks: Vec<Callback>,
        in_preview: bool,
    ) -> Self {
        Self {
            value,
            field_name,
            callbacks,
            in_preview,
        }
    }

    pub(crate) fn new(field_name: FieldName, in_preview: bool) -> ExtractFieldNode {
        Self::create(NotYetCalculated, field_name, Vec::new(), in_preview)
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

            NodeValue::Exists {
                value: Value::Object(map),
                secret: _,
            } => {
                log::info!(
                    "Extracting field: [{:?}] from map [{:?}]",
                    self.field_name,
                    map
                );
                let key: Value = self.field_name.as_string().clone().into();
                let value = map.iter().find(|(k, _)| *k == &key).map(|(_, v)| v.clone());
                let in_preview = self.in_preview;
                let new_node_value = match value {
                    None if in_preview => NodeValue::Nothing,
                    None => NodeValue::exists(Value::Null, false),
                    Some(v) => {
                        if v.is_object() && v.get(crate::constants::SPECIAL_SIG_KEY).is_some() {
                            let secret = v
                                .get(crate::constants::SPECIAL_SIG_KEY)
                                .unwrap()
                                .as_str()
                                .unwrap()
                                == crate::constants::SPECIAL_SECRET_SIG;
                            let value = v.get(crate::constants::SECRET_VALUE_NAME).unwrap().clone();
                            NodeValue::exists(value, secret)
                        } else {
                            NodeValue::exists(v.clone(), false)
                        }
                    }
                };
                self.value = Set(new_node_value.clone());
                new_node_value
            }
            NodeValue::Exists {
                value: _,
                secret: _,
            } => {
                error!("Cannot extract field from non-Map");
                panic!("Cannot extract field from non-Map");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CombineOutputsNode {
    value: MaybeNodeValue,
    inputs: Vec<Option<(Value, bool)>>,
    inputs_set: u32,
    callbacks: Vec<Callback>,
}

impl CombineOutputsNode {
    pub(crate) fn new(number_of_inputs: u32) -> Self {
        Self {
            value: NotYetCalculated,
            inputs: vec![None; number_of_inputs as usize],
            inputs_set: 0,
            callbacks: Vec::new(),
        }
    }

    pub(crate) fn set_node_value(&mut self, index: u32, value: NodeValue) -> Option<NodeValue> {
        self.inputs[index as usize] = match value {
            Nothing => None,
            NodeValue::Exists { value, secret } => Some((value, secret)),
        };
        self.inputs_set += 1;
        if self.inputs_set == self.inputs.len() as u32 {
            let set_inputs: Vec<_> = self.inputs.iter().filter_map(|v| v.clone()).collect();
            let value: NodeValue = if set_inputs.len() != self.inputs.len() {
                Nothing
            } else {
                let list: Value = set_inputs
                    .iter()
                    .map(|(value, _)| value.clone())
                    .collect::<Vec<_>>()
                    .into();
                let secret = set_inputs.iter().any(|(_, secret)| *secret);
                NodeValue::exists(list, secret)
            };
            self.value = Set(value.clone());
            Some(value)
        } else {
            None
        }
    }

    pub(crate) fn get_value(&self) -> &MaybeNodeValue {
        &self.value
    }

    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }
    pub(crate) fn get_callbacks(&self) -> &Vec<Callback> {
        &self.callbacks
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::model::NodeValue::Nothing;
    use crate::nodes::AbstractResourceNode;
    use serde_json::Value::Null;

    mod register_resource_node {
        use super::*;
        use crate::model::NodeValue;
        use crate::nodes::ResourceRequestOperation::Register;
        use crate::nodes::{RegisterResourceRequestOperation, ResourceRequestOperation};
        use crate::pulumi::service::PerformResourceRequest;
        use serde_json::json;

        #[test]
        fn set_input_passes_it_to_pulumi() {
            let mut node = AbstractResourceNode::new(
                Register(RegisterResourceRequestOperation::new(
                    "type".into(),
                    "name".into(),
                )),
                ["exists_nil".into(), "exists_int".into(), "not_exist".into()].into(),
                "1.0.0".into(),
            );

            let result = node.set_input("exists_nil".into(), NodeValue::exists(Null, false));
            assert_eq!(result, None);

            let result = node.set_input("exists_int".into(), NodeValue::exists(2.into(), false));
            assert_eq!(result, None);

            let result = node.set_input("not_exist".into(), Nothing);
            assert_eq!(
                result,
                Some(PerformResourceRequest {
                    object: HashMap::from([
                        ("exists_nil".into(), Some(Null)),
                        ("exists_int".into(), Some(2.into())),
                        ("not_exist".into(), None),
                    ]),
                    operation: ResourceRequestOperation::Register(
                        RegisterResourceRequestOperation::new("type".into(), "name".into())
                    ),
                    version: "1.0.0".into()
                })
            );
        }

        #[test]
        fn should_serialize_secrets() {
            let mut node = AbstractResourceNode::new(
                Register(RegisterResourceRequestOperation::new(
                    "type".into(),
                    "name".into(),
                )),
                ["secret".into()].into(),
                "1.0.0".into(),
            );

            let result = node.set_input("secret".into(), NodeValue::exists(2.into(), true));
            assert_eq!(
                result,
                Some(PerformResourceRequest {
                    object: HashMap::from([(
                        "secret".into(),
                        Some(json!({
                            "4dabf18193072939515e22adb298388d": "1b47061264138c4ac30d75fd1eb44270",
                            "value": 2,
                        }))
                    )]),
                    operation: ResourceRequestOperation::Register(
                        RegisterResourceRequestOperation::new("type".into(), "name".into())
                    ),
                    version: "1.0.0".into()
                })
            );
        }
    }

    mod combine_outputs_node {
        use super::*;
        use crate::model::MaybeNodeValue::{NotYetCalculated, Set};
        use crate::model::{MaybeNodeValue, NodeValue};
        use crate::nodes::CombineOutputsNode;
        use serde_json::json;

        #[test]
        fn set_inputs() {
            let mut node = CombineOutputsNode::new(2);

            let result = node.set_node_value(0, NodeValue::exists(0.into(), false));
            assert_eq!(result, None);
            assert_eq!(node.value, NotYetCalculated);

            let result = node.set_node_value(1, NodeValue::exists("123".into(), false));
            assert_eq!(result, Some(NodeValue::exists(json!([0, "123"]), false)));
            assert_eq!(
                node.value,
                MaybeNodeValue::set_value(json!([0, "123"]), false)
            );
        }

        #[test]
        fn set_unknown_inputs() {
            let mut node = CombineOutputsNode::new(2);

            let result = node.set_node_value(0, NodeValue::exists(0.into(), false));
            assert_eq!(result, None);
            assert_eq!(node.get_value(), &NotYetCalculated);

            let result = node.set_node_value(1, Nothing);
            assert_eq!(result, Some(Nothing));
            assert_eq!(node.get_value(), &Set(Nothing));
        }

        #[test]
        fn single_secret_causes_whole_result_to_be_false() {
            let mut node = CombineOutputsNode::new(2);

            let result = node.set_node_value(0, NodeValue::exists(0.into(), false));
            assert_eq!(result, None);
            assert_eq!(node.get_value(), &NotYetCalculated);

            let result = node.set_node_value(1, NodeValue::exists("123".into(), true));
            assert_eq!(result, Some(NodeValue::exists(json!([0, "123"]), true)));
            assert_eq!(
                node.get_value(),
                &Set(NodeValue::exists(json!([0, "123"]), true))
            );
        }
    }

    mod extract_field_node {
        use crate::model::{MaybeNodeValue, NodeValue};
        use crate::nodes::ExtractFieldNode;
        use serde_json::json;

        #[test]
        fn should_extract_field() {
            let mut node = ExtractFieldNode::new("field".into(), false);
            let value = NodeValue::exists(json!({"field": "value"}), false);
            let result = node.extract_field(&value);
            assert_eq!(result, NodeValue::exists("value".into(), false));
            assert_eq!(
                node.get_value(),
                &MaybeNodeValue::Set(NodeValue::exists("value".into(), false))
            );
        }

        #[test]
        fn should_extract_secret_value() {
            let mut node = ExtractFieldNode::new("field".into(), false);
            let value = NodeValue::exists(
                json!({
                    "field": {
                                "4dabf18193072939515e22adb298388d": "1b47061264138c4ac30d75fd1eb44270",
                                "value": "value",
                            }
                }),
                false,
            );
            let result = node.extract_field(&value);
            assert_eq!(result, NodeValue::exists("value".into(), true));
            assert_eq!(
                node.get_value(),
                &MaybeNodeValue::Set(NodeValue::exists("value".into(), true))
            );
        }
    }
}
