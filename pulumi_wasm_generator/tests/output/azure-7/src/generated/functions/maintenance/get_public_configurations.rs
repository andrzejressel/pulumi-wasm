pub mod get_public_configurations {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicConfigurationsArgs {
        /// The Azure location to filter the list of Public Maintenance Configurations against.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The recurring window to filter the list of Public Maintenance Configurations against. Possible values are `Monday-Thursday` and `Friday-Sunday`
        #[builder(into, default)]
        pub recur_every: pulumi_wasm_rust::Output<Option<String>>,
        /// The scope to filter the list of Public Maintenance Configurations against. Possible values are `Extension`, `Host`, `InGuestPatch`, `OSImage`, `SQLDB` and `SQLManagedInstance`.
        #[builder(into, default)]
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPublicConfigurationsResult {
        /// A `configs` block as defined below.
        pub configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::maintenance::GetPublicConfigurationsConfig>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure location of the Public Maintenance Configuration.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The rate at which a maintenance window is expected to recur.
        pub recur_every: pulumi_wasm_rust::Output<Option<String>>,
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPublicConfigurationsArgs) -> GetPublicConfigurationsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let recur_every_binding = args.recur_every.get_inner();
        let scope_binding = args.scope.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:maintenance/getPublicConfigurations:getPublicConfigurations"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "recurEvery".into(),
                    value: &recur_every_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "recurEvery".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPublicConfigurationsResult {
            configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            recur_every: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recurEvery").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}
