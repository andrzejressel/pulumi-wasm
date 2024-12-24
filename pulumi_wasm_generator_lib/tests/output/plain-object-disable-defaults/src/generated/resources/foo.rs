#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FooArgs {
    #[builder(into, default)]
    pub argument: pulumi_wasm_rust::Output<Option<String>>,
    /// Options for tuning the Kubernetes client used by a Provider.
    #[builder(into)]
    pub backup_kube_client_settings: pulumi_wasm_rust::Output<
        super::types::KubeClientSettings,
    >,
    /// Options for tuning the Kubernetes client used by a Provider.
    #[builder(into, default)]
    pub kube_client_settings: pulumi_wasm_rust::Output<
        Option<super::types::KubeClientSettings>,
    >,
    /// describing things
    #[builder(into, default)]
    pub settings: pulumi_wasm_rust::Output<Option<super::types::LayeredType>>,
}
pub struct FooResult {
    /// A test for plain types
    pub default_kube_client_settings: pulumi_wasm_rust::Output<
        Option<super::types::KubeClientSettings>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: FooArgs) -> FooResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let argument_binding = args.argument.get_inner();
    let backup_kube_client_settings_binding = args
        .backup_kube_client_settings
        .get_inner();
    let kube_client_settings_binding = args.kube_client_settings.get_inner();
    let settings_binding = args.settings.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "example:index:Foo".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "argument".into(),
                value: &argument_binding,
            },
            register_interface::ObjectField {
                name: "backupKubeClientSettings".into(),
                value: &backup_kube_client_settings_binding,
            },
            register_interface::ObjectField {
                name: "kubeClientSettings".into(),
                value: &kube_client_settings_binding,
            },
            register_interface::ObjectField {
                name: "settings".into(),
                value: &settings_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "defaultKubeClientSettings".into()
            },
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
    FooResult {
        default_kube_client_settings: into_domain(
            hashmap.remove("defaultKubeClientSettings").unwrap(),
        ),
    }
}
