/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Beta
        /// If `true`, the usage of the service to be disabled will be checked and an error
        /// will be returned if the service to be disabled has usage in last 30 days.
        /// Defaults to `false`.
        #[builder(into, default)]
        pub check_if_service_has_usage_on_destroy: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// If `true`, services that are enabled
        /// and which depend on this service should also be disabled when this service is
        /// destroyed. If `false` or unset, an error will be generated if any enabled
        /// services depend on this service when destroying it.
        #[builder(into, default)]
        pub disable_dependent_services: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub disable_on_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The project ID. If not provided, the provider project
        /// is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The service to enable.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Beta
        /// If `true`, the usage of the service to be disabled will be checked and an error
        /// will be returned if the service to be disabled has usage in last 30 days.
        /// Defaults to `false`.
        pub check_if_service_has_usage_on_destroy: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// If `true`, services that are enabled
        /// and which depend on this service should also be disabled when this service is
        /// destroyed. If `false` or unset, an error will be generated if any enabled
        /// services depend on this service when destroying it.
        pub disable_dependent_services: pulumi_gestalt_rust::Output<Option<bool>>,
        pub disable_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The project ID. If not provided, the provider project
        /// is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The service to enable.
        pub service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let check_if_service_has_usage_on_destroy_binding_1 = args
            .check_if_service_has_usage_on_destroy
            .get_output(context);
        let check_if_service_has_usage_on_destroy_binding = check_if_service_has_usage_on_destroy_binding_1
            .get_inner();
        let disable_dependent_services_binding_1 = args
            .disable_dependent_services
            .get_output(context);
        let disable_dependent_services_binding = disable_dependent_services_binding_1
            .get_inner();
        let disable_on_destroy_binding_1 = args.disable_on_destroy.get_output(context);
        let disable_on_destroy_binding = disable_on_destroy_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let service_binding_1 = args.service.get_output(context);
        let service_binding = service_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceResult {
            check_if_service_has_usage_on_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("checkIfServiceHasUsageOnDestroy"),
            ),
            disable_dependent_services: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableDependentServices"),
            ),
            disable_on_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableOnDestroy"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("service"),
            ),
        }
    }
}
