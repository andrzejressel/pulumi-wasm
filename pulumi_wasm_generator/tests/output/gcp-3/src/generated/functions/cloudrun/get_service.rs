pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The location of the cloud run instance. eg us-central1
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Cloud Run Service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub autogenerate_revision_name: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub metadatas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrun::GetServiceMetadata>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrun::GetServiceStatus>,
        >,
        pub templates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrun::GetServiceTemplate>,
        >,
        pub traffics: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrun::GetServiceTraffic>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServiceArgs) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudrun/getService:getService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autogenerateRevisionName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "metadatas".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "statuses".into(),
                },
                register_interface::ResultField {
                    name: "templates".into(),
                },
                register_interface::ResultField {
                    name: "traffics".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceResult {
            autogenerate_revision_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autogenerateRevisionName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            metadatas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadatas").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statuses").unwrap(),
            ),
            templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templates").unwrap(),
            ),
            traffics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("traffics").unwrap(),
            ),
        }
    }
}
