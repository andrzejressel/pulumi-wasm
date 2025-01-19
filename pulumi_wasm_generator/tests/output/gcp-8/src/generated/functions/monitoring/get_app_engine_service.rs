pub mod get_app_engine_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppEngineServiceArgs {
        /// The ID of the App Engine module underlying this
        /// service. Corresponds to the moduleId resource label in the [gae_app](https://cloud.google.com/monitoring/api/resources#tag_gae_app) monitored resource, or the service/module name.
        ///
        /// - - -
        ///
        /// Other optional fields include:
        #[builder(into)]
        pub module_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAppEngineServiceResult {
        /// Name used for UI elements listing this (Monitoring) Service.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub module_id: pulumi_wasm_rust::Output<String>,
        /// The full REST resource name for this channel. The syntax is:
        /// `projects/[PROJECT_ID]/services/[SERVICE_ID]`.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub service_id: pulumi_wasm_rust::Output<String>,
        /// Configuration for how to query telemetry on the Service. Structure is documented below.
        pub telemetries: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetAppEngineServiceTelemetry>,
        >,
        pub user_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAppEngineServiceArgs) -> GetAppEngineServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let module_id_binding = args.module_id.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:monitoring/getAppEngineService:getAppEngineService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "moduleId".into(),
                    value: &module_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "moduleId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
                register_interface::ResultField {
                    name: "telemetries".into(),
                },
                register_interface::ResultField {
                    name: "userLabels".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAppEngineServiceResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            module_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("moduleId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
            telemetries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("telemetries").unwrap(),
            ),
            user_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userLabels").unwrap(),
            ),
        }
    }
}
