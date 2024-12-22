
#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ExampleServerArgs {
    #[builder(into, default)]
    pub map_array_enum: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, crate::types::AnnotationStoreSchemaValueType>>>>,
}

pub struct ExampleServerResult {
    pub map_array_enum: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, crate::types::AnnotationStoreSchemaValueType>>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ExampleServerArgs
) -> ExampleServerResult {

    let result = crate::bindings::pulumi::example::example_server::invoke(
        name,
        &crate::bindings::pulumi::example::example_server::Args {
                map_array_enum: &args.map_array_enum.get_inner(),
        }
    );

    ExampleServerResult {
        map_array_enum: crate::into_domain(result.map_array_enum),
    }
}
