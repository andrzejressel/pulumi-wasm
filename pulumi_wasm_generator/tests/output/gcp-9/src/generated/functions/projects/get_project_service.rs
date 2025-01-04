pub mod get_project_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProjectServiceArgs {
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Google Platform project service.
        ///
        /// - - -
        #[builder(into)]
        pub service: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetProjectServiceResult {
        pub check_if_service_has_usage_on_destroy: pulumi_wasm_rust::Output<bool>,
        pub disable_dependent_services: pulumi_wasm_rust::Output<bool>,
        pub disable_on_destroy: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub service: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetProjectServiceArgs) -> GetProjectServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_inner();
        let service_binding = args.service.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:projects/getProjectService:getProjectService".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "checkIfServiceHasUsageOnDestroy".into(),
                },
                register_interface::ResultField {
                    name: "disableDependentServices".into(),
                },
                register_interface::ResultField {
                    name: "disableOnDestroy".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProjectServiceResult {
            check_if_service_has_usage_on_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checkIfServiceHasUsageOnDestroy").unwrap(),
            ),
            disable_dependent_services: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableDependentServices").unwrap(),
            ),
            disable_on_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableOnDestroy").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
        }
    }
}
