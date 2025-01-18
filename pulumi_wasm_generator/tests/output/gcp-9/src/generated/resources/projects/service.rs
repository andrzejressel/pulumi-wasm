/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let project = service::create(
///         "project",
///         ServiceArgs::builder()
///             .disable_on_destroy(false)
///             .project("your-project-id")
///             .service("iam.googleapis.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Project services can be imported using the `project_id` and `service`, e.g.
///
/// * `{{project_id}}/{{service}}`
///
/// When using the `pulumi import` command, project services can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:projects/service:Service default {{project_id}}/{{service}}
/// ```
///
/// Note that unlike other resources that fail if they already exist,
///
/// `pulumi up` can be successfully used to verify already enabled services.
///
/// This means that when importing existing resources into Terraform, you can either
///
/// import the `google_project_service` resources or treat them as new
///
/// infrastructure and run `pulumi up` to add them to state.
///
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Beta
        /// If `true`, the usage of the service to be disabled will be checked and an error
        /// will be returned if the service to be disabled has usage in last 30 days.
        /// Defaults to `false`.
        #[builder(into, default)]
        pub check_if_service_has_usage_on_destroy: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// If `true`, services that are enabled
        /// and which depend on this service should also be disabled when this service is
        /// destroyed. If `false` or unset, an error will be generated if any enabled
        /// services depend on this service when destroying it.
        #[builder(into, default)]
        pub disable_dependent_services: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub disable_on_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The project ID. If not provided, the provider project
        /// is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The service to enable.
        #[builder(into)]
        pub service: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Beta
        /// If `true`, the usage of the service to be disabled will be checked and an error
        /// will be returned if the service to be disabled has usage in last 30 days.
        /// Defaults to `false`.
        pub check_if_service_has_usage_on_destroy: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// If `true`, services that are enabled
        /// and which depend on this service should also be disabled when this service is
        /// destroyed. If `false` or unset, an error will be generated if any enabled
        /// services depend on this service when destroying it.
        pub disable_dependent_services: pulumi_wasm_rust::Output<Option<bool>>,
        pub disable_on_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The project ID. If not provided, the provider project
        /// is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The service to enable.
        pub service: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let check_if_service_has_usage_on_destroy_binding = args
            .check_if_service_has_usage_on_destroy
            .get_inner();
        let disable_dependent_services_binding = args
            .disable_dependent_services
            .get_inner();
        let disable_on_destroy_binding = args.disable_on_destroy.get_inner();
        let project_binding = args.project.get_inner();
        let service_binding = args.service.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:projects/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "checkIfServiceHasUsageOnDestroy".into(),
                    value: &check_if_service_has_usage_on_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "disableDependentServices".into(),
                    value: &disable_dependent_services_binding,
                },
                register_interface::ObjectField {
                    name: "disableOnDestroy".into(),
                    value: &disable_on_destroy_binding,
                },
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
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceResult {
            check_if_service_has_usage_on_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checkIfServiceHasUsageOnDestroy").unwrap(),
            ),
            disable_dependent_services: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableDependentServices").unwrap(),
            ),
            disable_on_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableOnDestroy").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
        }
    }
}
