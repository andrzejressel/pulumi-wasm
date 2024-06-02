use rmpv::Value;
use std::collections::HashMap;
use std::process::Output;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeFunctionId(String);
impl NativeFunctionId {
    pub(crate) const fn new(id: String) -> Self {
        Self(id)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct OutputId(pub(crate) Uuid);
impl OutputId {
    pub(crate) const fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FieldName(String);

impl FieldName {
    pub(crate) fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FieldName {
    pub(crate) const fn new(name: String) -> Self {
        Self(name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DoneOutput {
    pub(crate) value: Value,
    pub(crate) dependencies: Vec<String>,
}
impl DoneOutput {
    pub(crate) const fn new(value: Value, dependencies: Vec<String>) -> Self {
        Self {
            value,
            dependencies,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeFunctionOutput {
    pub(crate) output_id: OutputId,
    pub(crate) native_function_id: NativeFunctionId,
}
impl NativeFunctionOutput {
    pub(crate) const fn new(output_id: OutputId, native_function_id: NativeFunctionId) -> Self {
        Self {
            output_id,
            native_function_id,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExtractFieldOutput {
    pub(crate) output_id: OutputId,
    pub(crate) field_name: FieldName,
    pub(crate) dependencies: Vec<String>,
}
impl ExtractFieldOutput {
    pub(crate) const fn new(
        output_id: OutputId,
        field_name: FieldName,
        dependencies: Vec<String>,
    ) -> Self {
        Self {
            output_id,
            field_name,
            dependencies,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CreateResourceOptions {
    pub(crate) r#type: String,
    pub(crate) name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CreateResourceOutput {
    output_id: OutputId,
    options: CreateResourceOptions,
    inputs: HashMap<FieldName, OutputId>,
    expected_results: HashMap<FieldName, msgpack_protobuf_converter::Type>,
}
impl CreateResourceOutput {
    pub(crate) const fn new(
        output_id: OutputId,
        options: CreateResourceOptions,
        inputs: HashMap<FieldName, OutputId>,
        expected_results: HashMap<FieldName, msgpack_protobuf_converter::Type>,
    ) -> Self {
        Self {
            output_id,
            options,
            inputs,
            expected_results,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NothingOutput {
    pub(crate) dependencies: Vec<String>,
}

impl NothingOutput {
    pub(crate) const fn new(dependencies: Vec<String>) -> Self {
        NothingOutput { dependencies }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum OutputContent {
    Done(DoneOutput),
    Nothing(NothingOutput),

    NativeFunction(NativeFunctionOutput),
    ExtractField(ExtractFieldOutput),
    CreateResource(CreateResourceOutput),
}
#[derive(Debug, PartialEq)]
pub(crate) struct FunctionsToMap {
    id: OutputId,
    function_id: NativeFunctionId,
    input: Value,
}
impl FunctionsToMap {
    pub(crate) const fn new(id: OutputId, function_id: NativeFunctionId, input: Value) -> Self {
        Self {
            id,
            function_id,
            input,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct FieldsToExtract {
    pub(crate) id: OutputId,
    pub(crate) field_name: FieldName,
    pub(crate) output: Value,
}
impl FieldsToExtract {
    pub(crate) const fn new(id: OutputId, field_name: FieldName, output: Value) -> Self {
        Self {
            id,
            field_name,
            output,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct RegisteredResourceOutput {
    fields: HashMap<FieldName, (Option<Value>, Vec<String>)>,
}
