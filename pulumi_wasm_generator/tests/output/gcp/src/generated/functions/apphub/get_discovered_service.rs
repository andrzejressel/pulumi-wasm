pub mod get_discovered_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDiscoveredServiceArgs {
        /// The location of the discovered service.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The host project of the discovered service.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The uri of the service.
        #[builder(into)]
        pub service_uri: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDiscoveredServiceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The location that the underlying resource resides in.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Resource name of a Service. Format: "projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}".
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Properties of an underlying compute resource that can comprise a Service. Structure is documented below
        pub service_properties: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::apphub::GetDiscoveredServiceServiceProperty>,
        >,
        /// Reference to an underlying networking resource that can comprise a Service. Structure is documented below
        pub service_references: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::apphub::GetDiscoveredServiceServiceReference>,
        >,
        pub service_uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDiscoveredServiceArgs) -> GetDiscoveredServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let service_uri_binding = args.service_uri.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:apphub/getDiscoveredService:getDiscoveredService".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceUri".into(),
                    value: &service_uri_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "serviceProperties".into(),
                },
                register_interface::ResultField {
                    name: "serviceReferences".into(),
                },
                register_interface::ResultField {
                    name: "serviceUri".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDiscoveredServiceResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceProperties").unwrap(),
            ),
            service_references: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceReferences").unwrap(),
            ),
            service_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceUri").unwrap(),
            ),
        }
    }
}
