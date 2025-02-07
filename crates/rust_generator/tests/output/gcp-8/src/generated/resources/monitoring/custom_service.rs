/// A Service is a discrete, autonomous, and network-accessible unit,
/// designed to solve an individual concern. In Cloud Monitoring,
/// a Service acts as the root resource under which operational aspects of
/// the service are accessible
///
///
/// To get more information about Service, see:
///
/// * [API documentation](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/services)
/// * How-to Guides
///     * [Monitoring API Documentation](https://cloud.google.com/monitoring/api/v3/)
///     * [Service Monitoring](https://cloud.google.com/monitoring/service-monitoring)
///     * [Service-orientation on Wikipedia](https://en.wikipedia.org/wiki/Service-orientation)
///
/// ## Example Usage
///
/// ### Monitoring Service Custom
///
///
/// ```yaml
/// resources:
///   custom:
///     type: gcp:monitoring:CustomService
///     properties:
///       serviceId: custom-srv
///       displayName: My Custom Service custom-srv
///       telemetry:
///         resourceName: //product.googleapis.com/foo/foo/services/test
///       userLabels:
///         my_key: my_value
///         my_other_key: my_other_value
/// ```
///
/// ## Import
///
/// Service can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Service can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:monitoring/customService:CustomService default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/customService:CustomService default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/customService:CustomService default {{name}}
/// ```
///
pub mod custom_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomServiceArgs {
        /// Name used for UI elements listing this Service.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional service ID to use. If not given, the server will generate a
        /// service ID.
        #[builder(into, default)]
        pub service_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for how to query telemetry on a Service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub telemetry: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::CustomServiceTelemetry>,
        >,
        /// Labels which have been used to annotate the service. Label keys must start
        /// with a letter. Label keys and values may contain lowercase letters,
        /// numbers, underscores, and dashes. Label keys and values have a maximum
        /// length of 63 characters, and must be less than 128 bytes in size. Up to 64
        /// label entries may be stored. For labels which do not have a semantic value,
        /// the empty string may be supplied for the label value.
        #[builder(into, default)]
        pub user_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomServiceResult {
        /// Name used for UI elements listing this Service.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The full resource name for this service. The syntax is:
        /// projects/[PROJECT_ID]/services/[SERVICE_ID].
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// An optional service ID to use. If not given, the server will generate a
        /// service ID.
        pub service_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration for how to query telemetry on a Service.
        /// Structure is documented below.
        pub telemetry: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::CustomServiceTelemetry>,
        >,
        /// Labels which have been used to annotate the service. Label keys must start
        /// with a letter. Label keys and values may contain lowercase letters,
        /// numbers, underscores, and dashes. Label keys and values have a maximum
        /// length of 63 characters, and must be less than 128 bytes in size. Up to 64
        /// label entries may be stored. For labels which do not have a semantic value,
        /// the empty string may be supplied for the label value.
        pub user_labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomServiceArgs,
    ) -> CustomServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let service_id_binding = args.service_id.get_output(context).get_inner();
        let telemetry_binding = args.telemetry.get_output(context).get_inner();
        let user_labels_binding = args.user_labels.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:monitoring/customService:CustomService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
                register_interface::ObjectField {
                    name: "telemetry".into(),
                    value: &telemetry_binding,
                },
                register_interface::ObjectField {
                    name: "userLabels".into(),
                    value: &user_labels_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomServiceResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
            telemetry: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("telemetry"),
            ),
            user_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userLabels"),
            ),
        }
    }
}
