#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RubberTreeArgs {
    #[builder(into, default)]
    pub container: pulumi_wasm_rust::Output<
        Option<super::super::super::types::Container>,
    >,
    #[builder(into)]
    pub diameter: pulumi_wasm_rust::Output<
        super::super::super::types::tree::v1::Diameter,
    >,
    #[builder(into, default)]
    pub farm: pulumi_wasm_rust::Output<
        Option<
            pulumi_wasm_provider_common::OneOf2<
                super::super::super::types::tree::v1::Farm,
                String,
            >,
        >,
    >,
    #[builder(into, default)]
    pub size: pulumi_wasm_rust::Output<
        Option<super::super::super::types::tree::v1::TreeSize>,
    >,
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<
        super::super::super::types::tree::v1::RubberTreeVariety,
    >,
}
pub struct RubberTreeResult {
    pub container: pulumi_wasm_rust::Output<
        Option<super::super::super::types::Container>,
    >,
    pub diameter: pulumi_wasm_rust::Output<
        super::super::super::types::tree::v1::Diameter,
    >,
    pub farm: pulumi_wasm_rust::Output<
        Option<
            pulumi_wasm_provider_common::OneOf2<
                super::super::super::types::tree::v1::Farm,
                String,
            >,
        >,
    >,
    pub size: pulumi_wasm_rust::Output<
        Option<super::super::super::types::tree::v1::TreeSize>,
    >,
    pub type_: pulumi_wasm_rust::Output<
        super::super::super::types::tree::v1::RubberTreeVariety,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RubberTreeArgs) -> RubberTreeResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let container_binding = args.container.get_inner();
    let diameter_binding = args.diameter.get_inner();
    let farm_binding = args.farm.get_inner();
    let size_binding = args.size.get_inner();
    let type__binding = args.type_.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "plant:tree/v1:RubberTree".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "container".into(),
                value: &container_binding,
            },
            register_interface::ObjectField {
                name: "diameter".into(),
                value: &diameter_binding,
            },
            register_interface::ObjectField {
                name: "farm".into(),
                value: &farm_binding,
            },
            register_interface::ObjectField {
                name: "size".into(),
                value: &size_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "container".into() },
            register_interface::ResultField { name : "diameter".into() },
            register_interface::ResultField { name : "farm".into() },
            register_interface::ResultField { name : "size".into() },
            register_interface::ResultField { name : "type".into() },
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
    RubberTreeResult {
        container: into_domain(hashmap.remove("container").unwrap()),
        diameter: into_domain(hashmap.remove("diameter").unwrap()),
        farm: into_domain(hashmap.remove("farm").unwrap()),
        size: into_domain(hashmap.remove("size").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
    }
}
