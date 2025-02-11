/// An API Configuration is an association of an API Controller Config and a Gateway Config
///
/// To get more information about ApiConfig, see:
///
/// * [API documentation](https://cloud.google.com/api-gateway/docs/reference/rest/v1beta/projects.locations.apis.configs)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/api-gateway/docs/creating-api-config)
///
/// ## Example Usage
///
/// ## Import
///
/// ApiConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config_id}}`
///
/// * `{{project}}/{{api}}/{{api_config_id}}`
///
/// * `{{api}}/{{api_config_id}}`
///
/// When using the `pulumi import` command, ApiConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigateway/apiConfig:ApiConfig default projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigateway/apiConfig:ApiConfig default {{project}}/{{api}}/{{api_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigateway/apiConfig:ApiConfig default {{api}}/{{api_config_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiConfigArgs {
        /// The API to attach the config to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub api: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier to assign to the API Config. Must be unique within scope of the parent resource(api).
        #[builder(into, default)]
        pub api_config_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the
        /// specified prefix. If this and api_config_id are unspecified, a random value is chosen for the name.
        #[builder(into, default)]
        pub api_config_id_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A user-visible name for the API.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Immutable. Gateway specific configuration.
        /// If not specified, backend authentication will be set to use OIDC authentication using the default compute service account
        /// Structure is documented below.
        #[builder(into, default)]
        pub gateway_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigateway::ApiConfigGatewayConfig>,
        >,
        /// gRPC service definition files. If specified, openapiDocuments must not be included.
        /// Structure is documented below.
        #[builder(into, default)]
        pub grpc_services: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apigateway::ApiConfigGrpcService>>,
        >,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Service Configuration files. At least one must be included when using gRPC service definitions. See https://cloud.google.com/endpoints/docs/grpc/grpc-service-config#service_configuration_overview for the expected file contents.
        /// If multiple files are specified, the files are merged with the following rules: * All singular scalar fields are merged using "last one wins" semantics in the order of the files uploaded. * Repeated fields are concatenated. * Singular embedded messages are merged using these rules for nested fields.
        /// Structure is documented below.
        #[builder(into, default)]
        pub managed_service_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apigateway::ApiConfigManagedServiceConfig>>,
        >,
        /// OpenAPI specification documents. If specified, grpcServices and managedServiceConfigs must not be included.
        /// Structure is documented below.
        #[builder(into, default)]
        pub openapi_documents: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apigateway::ApiConfigOpenapiDocument>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiConfigResult {
        /// The API to attach the config to.
        ///
        ///
        /// - - -
        pub api: pulumi_gestalt_rust::Output<String>,
        /// Identifier to assign to the API Config. Must be unique within scope of the parent resource(api).
        pub api_config_id: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the
        /// specified prefix. If this and api_config_id are unspecified, a random value is chosen for the name.
        pub api_config_id_prefix: pulumi_gestalt_rust::Output<String>,
        /// A user-visible name for the API.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Immutable. Gateway specific configuration.
        /// If not specified, backend authentication will be set to use OIDC authentication using the default compute service account
        /// Structure is documented below.
        pub gateway_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigateway::ApiConfigGatewayConfig>,
        >,
        /// gRPC service definition files. If specified, openapiDocuments must not be included.
        /// Structure is documented below.
        pub grpc_services: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apigateway::ApiConfigGrpcService>>,
        >,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Service Configuration files. At least one must be included when using gRPC service definitions. See https://cloud.google.com/endpoints/docs/grpc/grpc-service-config#service_configuration_overview for the expected file contents.
        /// If multiple files are specified, the files are merged with the following rules: * All singular scalar fields are merged using "last one wins" semantics in the order of the files uploaded. * Repeated fields are concatenated. * Singular embedded messages are merged using these rules for nested fields.
        /// Structure is documented below.
        pub managed_service_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apigateway::ApiConfigManagedServiceConfig>>,
        >,
        /// The resource name of the API Config.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// OpenAPI specification documents. If specified, grpcServices and managedServiceConfigs must not be included.
        /// Structure is documented below.
        pub openapi_documents: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apigateway::ApiConfigOpenapiDocument>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the associated Service Config (https://cloud.google.com/service-infrastructure/docs/glossary#config).
        pub service_config_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiConfigArgs,
    ) -> ApiConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_binding = args.api.get_output(context);
        let api_config_id_binding = args.api_config_id.get_output(context);
        let api_config_id_prefix_binding = args.api_config_id_prefix.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let gateway_config_binding = args.gateway_config.get_output(context);
        let grpc_services_binding = args.grpc_services.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let managed_service_configs_binding = args
            .managed_service_configs
            .get_output(context);
        let openapi_documents_binding = args.openapi_documents.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigateway/apiConfig:ApiConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "api".into(),
                    value: &api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiConfigId".into(),
                    value: &api_config_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiConfigIdPrefix".into(),
                    value: &api_config_id_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayConfig".into(),
                    value: &gateway_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grpcServices".into(),
                    value: &grpc_services_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedServiceConfigs".into(),
                    value: &managed_service_configs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "openapiDocuments".into(),
                    value: &openapi_documents_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiConfigResult {
            api: o.get_field("api"),
            api_config_id: o.get_field("apiConfigId"),
            api_config_id_prefix: o.get_field("apiConfigIdPrefix"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            gateway_config: o.get_field("gatewayConfig"),
            grpc_services: o.get_field("grpcServices"),
            labels: o.get_field("labels"),
            managed_service_configs: o.get_field("managedServiceConfigs"),
            name: o.get_field("name"),
            openapi_documents: o.get_field("openapiDocuments"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            service_config_id: o.get_field("serviceConfigId"),
        }
    }
}
