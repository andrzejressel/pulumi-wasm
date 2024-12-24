#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ObservatoryScheduledTestArgs {
    /// The frequency to run the test. Available values: `DAILY`, `WEEKLY`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub frequency: pulumi_wasm_rust::Output<String>,
    /// The region to run the test in. Available values: `us-central1`, `us-east1`, `us-east4`, `us-south1`, `us-west1`, `southamerica-east1`, `europe-north1`, `europe-southwest1`, `europe-west1`, `europe-west2`, `europe-west3`, `europe-west4`, `europe-west8`, `europe-west9`, `asia-east1`, `asia-south1`, `asia-southeast1`, `me-west1`, `australia-southeast1`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub region: pulumi_wasm_rust::Output<String>,
    /// The page to run the test on. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct ObservatoryScheduledTestResult {
    /// The frequency to run the test. Available values: `DAILY`, `WEEKLY`. **Modifying this attribute will force creation of a new resource.**
    pub frequency: pulumi_wasm_rust::Output<String>,
    /// The region to run the test in. Available values: `us-central1`, `us-east1`, `us-east4`, `us-south1`, `us-west1`, `southamerica-east1`, `europe-north1`, `europe-southwest1`, `europe-west1`, `europe-west2`, `europe-west3`, `europe-west4`, `europe-west8`, `europe-west9`, `asia-east1`, `asia-south1`, `asia-southeast1`, `me-west1`, `australia-southeast1`. **Modifying this attribute will force creation of a new resource.**
    pub region: pulumi_wasm_rust::Output<String>,
    /// The page to run the test on. **Modifying this attribute will force creation of a new resource.**
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ObservatoryScheduledTestArgs,
) -> ObservatoryScheduledTestResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let frequency_binding = args.frequency.get_inner();
    let region_binding = args.region.get_inner();
    let url_binding = args.url.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/observatoryScheduledTest:ObservatoryScheduledTest"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "frequency".into(),
                value: &frequency_binding,
            },
            register_interface::ObjectField {
                name: "region".into(),
                value: &region_binding,
            },
            register_interface::ObjectField {
                name: "url".into(),
                value: &url_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "frequency".into() },
            register_interface::ResultField { name : "region".into() },
            register_interface::ResultField { name : "url".into() },
            register_interface::ResultField { name : "zoneId".into() },
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
    ObservatoryScheduledTestResult {
        frequency: into_domain(hashmap.remove("frequency").unwrap()),
        region: into_domain(hashmap.remove("region").unwrap()),
        url: into_domain(hashmap.remove("url").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
