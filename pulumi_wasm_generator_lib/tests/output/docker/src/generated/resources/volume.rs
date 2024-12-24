#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct VolumeArgs {
    /// Driver type for the volume. Defaults to `local`.
    #[builder(into, default)]
    pub driver: pulumi_wasm_rust::Output<Option<String>>,
    /// Options specific to the driver.
    #[builder(into, default)]
    pub driver_opts: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    /// User-defined key/value metadata
    #[builder(into, default)]
    pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::VolumeLabel>>>,
    /// The name of the Docker volume (will be generated if not provided).
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct VolumeResult {
    /// Driver type for the volume. Defaults to `local`.
    pub driver: pulumi_wasm_rust::Output<String>,
    /// Options specific to the driver.
    pub driver_opts: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    /// User-defined key/value metadata
    pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::VolumeLabel>>>,
    /// The mountpoint of the volume.
    pub mountpoint: pulumi_wasm_rust::Output<String>,
    /// The name of the Docker volume (will be generated if not provided).
    pub name: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: VolumeArgs) -> VolumeResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let driver_binding = args.driver.get_inner();
    let driver_opts_binding = args.driver_opts.get_inner();
    let labels_binding = args.labels.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "docker:index/volume:Volume".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "driver".into(),
                value: &driver_binding,
            },
            register_interface::ObjectField {
                name: "driverOpts".into(),
                value: &driver_opts_binding,
            },
            register_interface::ObjectField {
                name: "labels".into(),
                value: &labels_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "driver".into() },
            register_interface::ResultField { name : "driverOpts".into() },
            register_interface::ResultField { name : "labels".into() },
            register_interface::ResultField { name : "mountpoint".into() },
            register_interface::ResultField { name : "name".into() },
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
    VolumeResult {
        driver: into_domain(hashmap.remove("driver").unwrap()),
        driver_opts: into_domain(hashmap.remove("driverOpts").unwrap()),
        labels: into_domain(hashmap.remove("labels").unwrap()),
        mountpoint: into_domain(hashmap.remove("mountpoint").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
    }
}
