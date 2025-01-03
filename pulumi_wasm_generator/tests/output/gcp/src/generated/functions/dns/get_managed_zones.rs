pub mod get_managed_zones {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedZonesArgs {
        /// The ID of the project containing Google Cloud DNS zones. If this is not provided the default project will be used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetManagedZonesResult {
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of managed zones.
        pub managed_zones: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dns::GetManagedZonesManagedZone>,
        >,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetManagedZonesArgs) -> GetManagedZonesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:dns/getManagedZones:getManagedZones".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "managedZones".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetManagedZonesResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            managed_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedZones").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
