/// test new feature with resoruces
pub mod foo {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub struct FooResult {
        /// A test for plain types
        pub default_kube_client_settings: pulumi_wasm_rust::Output<
            Option<super::types::KubeClientSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FooArgs) -> FooResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultKubeClientSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FooResult {
            default_kube_client_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultKubeClientSettings").unwrap(),
            ),
        }
    }
}
