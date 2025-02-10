/// A consumable API that can be used by multiple Gateways.
///
/// To get more information about Gateway, see:
///
/// * [API documentation](https://cloud.google.com/api-gateway/docs/reference/rest/v1beta/projects.locations.apis)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/api-gateway/docs/quickstart)
///
/// ## Example Usage
///
/// ## Import
///
/// Gateway can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/gateways/{{gateway_id}}`
///
/// * `{{project}}/{{region}}/{{gateway_id}}`
///
/// * `{{region}}/{{gateway_id}}`
///
/// * `{{gateway_id}}`
///
/// When using the `pulumi import` command, Gateway can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigateway/gateway:Gateway default projects/{{project}}/locations/{{region}}/gateways/{{gateway_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigateway/gateway:Gateway default {{project}}/{{region}}/{{gateway_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigateway/gateway:Gateway default {{region}}/{{gateway_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigateway/gateway:Gateway default {{gateway_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayArgs {
        /// Resource name of the API Config for this Gateway. Format: projects/{project}/locations/global/apis/{api}/configs/{apiConfig}.
        /// When changing api configs please ensure the new config is a new resource and the
        /// lifecycle rule `create_before_destroy` is set.
        #[builder(into)]
        pub api_config: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A user-visible name for the API.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier to assign to the Gateway. Must be unique within scope of the parent resource(project).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the gateway for the API.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GatewayResult {
        /// Resource name of the API Config for this Gateway. Format: projects/{project}/locations/global/apis/{api}/configs/{apiConfig}.
        /// When changing api configs please ensure the new config is a new resource and the
        /// lifecycle rule `create_before_destroy` is set.
        pub api_config: pulumi_gestalt_rust::Output<String>,
        /// The default API Gateway host name of the form {gatewayId}-{hash}.{region_code}.gateway.dev.
        pub default_hostname: pulumi_gestalt_rust::Output<String>,
        /// A user-visible name for the API.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier to assign to the Gateway. Must be unique within scope of the parent resource(project).
        ///
        ///
        /// - - -
        pub gateway_id: pulumi_gestalt_rust::Output<String>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource name of the Gateway. Format: projects/{project}/locations/{region}/gateways/{gateway}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the gateway for the API.
        pub region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GatewayArgs,
    ) -> GatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_config_binding = args.api_config.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let gateway_id_binding = args.gateway_id.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigateway/gateway:Gateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiConfig".into(),
                    value: api_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayId".into(),
                    value: gateway_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GatewayResult {
            api_config: o.get_field("apiConfig"),
            default_hostname: o.get_field("defaultHostname"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            gateway_id: o.get_field("gatewayId"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
        }
    }
}
