#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TagArgs {
    /// Name of the source image.
    #[builder(into)]
    pub source_image: pulumi_wasm_rust::Output<String>,
    /// Name of the target image.
    #[builder(into)]
    pub target_image: pulumi_wasm_rust::Output<String>,
}
pub struct TagResult {
    /// Name of the source image.
    pub source_image: pulumi_wasm_rust::Output<String>,
    /// ImageID of the source image in the format of `sha256:<<ID>>`
    pub source_image_id: pulumi_wasm_rust::Output<String>,
    /// Name of the target image.
    pub target_image: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TagArgs) -> TagResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let source_image_binding = args.source_image.get_inner();
    let target_image_binding = args.target_image.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "docker:index/tag:Tag".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "sourceImage".into(),
                value: &source_image_binding,
            },
            register_interface::ObjectField {
                name: "targetImage".into(),
                value: &target_image_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "sourceImage".into() },
            register_interface::ResultField { name : "sourceImageId".into() },
            register_interface::ResultField { name : "targetImage".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    TagResult {
        source_image: into_domain(hashmap.remove("sourceImage").unwrap()),
        source_image_id: into_domain(hashmap.remove("sourceImageId").unwrap()),
        target_image: into_domain(hashmap.remove("targetImage").unwrap()),
    }
}
