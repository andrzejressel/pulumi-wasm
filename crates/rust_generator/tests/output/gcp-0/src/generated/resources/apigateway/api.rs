/// A consumable API that can be used by multiple Gateways.
///
/// To get more information about Api, see:
///
/// * [API documentation](https://cloud.google.com/api-gateway/docs/reference/rest/v1beta/projects.locations.apis)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/api-gateway/docs/quickstart)
///
/// ## Example Usage
///
/// ### Apigateway Api Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let api = api::create("api", ApiArgs::builder().api_id("my-api").build_struct());
/// }
/// ```
///
/// ## Import
///
/// Api can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/apis/{{api_id}}`
///
/// * `{{project}}/{{api_id}}`
///
/// * `{{api_id}}`
///
/// When using the `pulumi import` command, Api can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigateway/api:Api default projects/{{project}}/locations/global/apis/{{api_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigateway/api:Api default {{project}}/{{api_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigateway/api:Api default {{api_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiArgs {
        /// Identifier to assign to the API. Must be unique within scope of the parent resource(project)
        ///
        ///
        /// - - -
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A user-visible name for the API.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Immutable. The name of a Google Managed Service ( https://cloud.google.com/service-infrastructure/docs/glossary#managed).
        /// If not specified, a new Service will automatically be created in the same project as this API.
        #[builder(into, default)]
        pub managed_service: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiResult {
        /// Identifier to assign to the API. Must be unique within scope of the parent resource(project)
        ///
        ///
        /// - - -
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A user-visible name for the API.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Immutable. The name of a Google Managed Service ( https://cloud.google.com/service-infrastructure/docs/glossary#managed).
        /// If not specified, a new Service will automatically be created in the same project as this API.
        pub managed_service: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the API. Format `projects/{{project}}/locations/global/apis/{{apiId}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApiArgs,
    ) -> ApiResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let managed_service_binding = args
            .managed_service
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigateway/api:Api".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "managedService".into(),
                    value: &managed_service_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApiResult {
            api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiId"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            managed_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedService"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
        }
    }
}
