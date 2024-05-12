use rmpv::Value;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeFunctionId(String);
impl NativeFunctionId {
    pub(crate) const fn new(id: String) -> NativeFunctionId {
        NativeFunctionId(id)
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub(crate) struct OutputId(Uuid);
impl OutputId {
    pub(crate) const fn new(uuid: Uuid) -> OutputId {
        OutputId(uuid)
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FieldName(String);

impl FieldName {
    pub(crate) const fn new(name: String) -> FieldName {
        FieldName(name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DoneOutput {
    pub(crate) value: Value,
    pub(crate) dependencies: Vec<String>,
}
impl DoneOutput {
    pub(crate) const fn new(value: Value, dependencies: Vec<String>) -> Self {
        DoneOutput {
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
        NativeFunctionOutput {
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
        ExtractFieldOutput {
            output_id,
            field_name,
            dependencies,
        }
    }
}

pub(crate) struct FuncOutput {
    dependencies: Vec<String>,
    function: Box<dyn Fn(Vec<Value>) -> Option<Value>>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NothingOutput {
    dependencies: Vec<String>,
}

impl NothingOutput {
    pub(crate) const fn new(dependencies: Vec<String>) -> Self {
        NothingOutput { dependencies }
    }
}

pub(crate) enum OutputContent {
    Done(DoneOutput),
    NativeFunction(NativeFunctionOutput),
    ExtractField(ExtractFieldOutput),
    Func(FuncOutput),
    Nothing(NothingOutput),
}
#[derive(Debug, PartialEq)]
pub(crate) struct FunctionsToMap {
    id: OutputId,
    function_id: NativeFunctionId,
    output: Value,
}
impl FunctionsToMap {
    pub(crate) const fn new(id: OutputId, function_id: NativeFunctionId, output: Value) -> Self {
        FunctionsToMap {
            id,
            function_id,
            output,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct FieldsToExtract {
    id: OutputId,
    field_name: FieldName,
    output: Value,
}
impl FieldsToExtract {
    pub(crate) const fn new(id: OutputId, field_name: FieldName, output: Value) -> Self {
        FieldsToExtract {
            id,
            field_name,
            output,
        }
    }
}
