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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let check_if_service_has_usage_on_destroy_binding = args
            .check_if_service_has_usage_on_destroy
            .get_output(context);
        let disable_dependent_services_binding = args
            .disable_dependent_services
            .get_output(context);
        let disable_on_destroy_binding = args.disable_on_destroy.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:projects/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "checkIfServiceHasUsageOnDestroy".into(),
                    value: check_if_service_has_usage_on_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableDependentServices".into(),
                    value: disable_dependent_services_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableOnDestroy".into(),
                    value: disable_on_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: service_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceResult {
            check_if_service_has_usage_on_destroy: o
                .get_field("checkIfServiceHasUsageOnDestroy"),
            disable_dependent_services: o.get_field("disableDependentServices"),
            disable_on_destroy: o.get_field("disableOnDestroy"),
            project: o.get_field("project"),
            service: o.get_field("service"),
        }
    }
}
