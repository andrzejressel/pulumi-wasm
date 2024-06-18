use crate::model::MaybeNodeValue::Set;
use crate::model::{FieldName, FunctionName, MaybeNodeValue, NodeValue, OutputId};
use rmpv::Value;

pub(crate) enum RunState {
    // Ready(MaybeNodeValue),
    // NeedMoreData,
    // NativeFunctionReady,
    // CreateResourceReady
}

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
    value: Value,
    callbacks: Vec<Callback>,
}

impl DoneNode {
    pub(crate) fn new(value: Value) -> Self {
        DoneNode::create(value, Vec::new())
    }

    pub(crate) fn create(value: Value, callbacks: Vec<Callback>) -> Self {
        Self { value, callbacks }
    }

    pub(crate) fn get_value(&self) -> Value {
        self.value.clone()
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

    pub(crate) fn set_argument(&mut self, value: NodeValue) {
        self.argument = MaybeNodeValue::Set(value);
    }
    
    pub(crate) fn set_value(&mut self, value: NodeValue) {
        self.value = MaybeNodeValue::Set(value);
    }
}

impl NativeFunctionNode {
    fn set_data(&mut self, node_value: NodeValue) {
        self.value = MaybeNodeValue::Set(node_value);
    }
    pub(crate) fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct RegisterResourceNode {
    value: MaybeNodeValue,
    callbacks: Vec<Callback>,
}

impl RegisterResourceNode {
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
    fn add_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    mod done {
        use super::*;
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
    static UUID_2: Uuid = Uuid::from_bytes([2; 16]);
    static UUID_3: Uuid = Uuid::from_bytes([3; 16]);
    static UUID_4: Uuid = Uuid::from_bytes([4; 16]);
    static UUID_5: Uuid = Uuid::from_bytes([5; 16]);
    static UUID_6: Uuid = Uuid::from_bytes([6; 16]);
}
