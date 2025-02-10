/// Manages an API Management Service Diagnostic.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleDiagnostic = diagnostic::create(
///         "exampleDiagnostic",
///         DiagnosticArgs::builder()
///             .always_log_errors(true)
///             .api_management_logger_id("${exampleLogger.id}")
///             .api_management_name("${exampleService.name}")
///             .backend_request(
///                 DiagnosticBackendRequest::builder()
///                     .bodyBytes(32)
///                     .headersToLogs(vec!["content-type", "accept", "origin",])
///                     .build_struct(),
///             )
///             .backend_response(
///                 DiagnosticBackendResponse::builder()
///                     .bodyBytes(32)
///                     .headersToLogs(vec!["content-type", "content-length", "origin",])
///                     .build_struct(),
///             )
///             .frontend_request(
///                 DiagnosticFrontendRequest::builder()
///                     .bodyBytes(32)
///                     .headersToLogs(vec!["content-type", "accept", "origin",])
///                     .build_struct(),
///             )
///             .frontend_response(
///                 DiagnosticFrontendResponse::builder()
///                     .bodyBytes(32)
///                     .headersToLogs(vec!["content-type", "content-length", "origin",])
///                     .build_struct(),
///             )
///             .http_correlation_protocol("W3C")
///             .identifier("applicationinsights")
///             .log_client_ip(true)
///             .resource_group_name("${example.name}")
///             .sampling_percentage(5)
///             .verbosity("verbose")
///             .build_struct(),
///     );
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("example-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLogger = logger::create(
///         "exampleLogger",
///         LoggerArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .application_insights(
///                 LoggerApplicationInsights::builder()
///                     .instrumentationKey("${exampleInsights.instrumentationKey}")
///                     .build_struct(),
///             )
///             .name("example-apimlogger")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@mycompany.io")
///             .publisher_name("My Company")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Diagnostics can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/diagnostic:Diagnostic example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/diagnostics/applicationinsights
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod diagnostic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiagnosticArgs {
        /// Always log errors. Send telemetry if there is an erroneous condition, regardless of sampling settings.
        #[builder(into, default)]
        pub always_log_errors: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The id of the target API Management Logger where the API Management Diagnostic should be saved.
        #[builder(into)]
        pub api_management_logger_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the API Management Service where this Diagnostic should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `backend_request` block as defined below.
        #[builder(into, default)]
        pub backend_request: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::DiagnosticBackendRequest>,
        >,
        /// A `backend_response` block as defined below.
        #[builder(into, default)]
        pub backend_response: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::DiagnosticBackendResponse>,
        >,
        /// A `frontend_request` block as defined below.
        #[builder(into, default)]
        pub frontend_request: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::DiagnosticFrontendRequest>,
        >,
        /// A `frontend_response` block as defined below.
        #[builder(into, default)]
        pub frontend_response: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::DiagnosticFrontendResponse>,
        >,
        /// The HTTP Correlation Protocol to use. Possible values are `None`, `Legacy` or `W3C`.
        #[builder(into, default)]
        pub http_correlation_protocol: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The diagnostic identifier for the API Management Service. At this time the supported values are `applicationinsights` and `azuremonitor`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Log client IP address.
        #[builder(into, default)]
        pub log_client_ip: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The format of the Operation Name for Application Insights telemetries. Possible values are `Name`, and `Url`.
        #[builder(into, default)]
        pub operation_name_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Sampling (%). For high traffic APIs, please read this [documentation](https://docs.microsoft.com/azure/api-management/api-management-howto-app-insights#performance-implications-and-log-sampling) to understand performance implications and log sampling. Valid values are between `0.0` and `100.0`.
        #[builder(into, default)]
        pub sampling_percentage: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// Logging verbosity. Possible values are `verbose`, `information` or `error`.
        #[builder(into, default)]
        pub verbosity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DiagnosticResult {
        /// Always log errors. Send telemetry if there is an erroneous condition, regardless of sampling settings.
        pub always_log_errors: pulumi_gestalt_rust::Output<bool>,
        /// The id of the target API Management Logger where the API Management Diagnostic should be saved.
        pub api_management_logger_id: pulumi_gestalt_rust::Output<String>,
        /// The Name of the API Management Service where this Diagnostic should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// A `backend_request` block as defined below.
        pub backend_request: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::DiagnosticBackendRequest,
        >,
        /// A `backend_response` block as defined below.
        pub backend_response: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::DiagnosticBackendResponse,
        >,
        /// A `frontend_request` block as defined below.
        pub frontend_request: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::DiagnosticFrontendRequest,
        >,
        /// A `frontend_response` block as defined below.
        pub frontend_response: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::DiagnosticFrontendResponse,
        >,
        /// The HTTP Correlation Protocol to use. Possible values are `None`, `Legacy` or `W3C`.
        pub http_correlation_protocol: pulumi_gestalt_rust::Output<String>,
        /// The diagnostic identifier for the API Management Service. At this time the supported values are `applicationinsights` and `azuremonitor`. Changing this forces a new resource to be created.
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// Log client IP address.
        pub log_client_ip: pulumi_gestalt_rust::Output<bool>,
        /// The format of the Operation Name for Application Insights telemetries. Possible values are `Name`, and `Url`.
        pub operation_name_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Sampling (%). For high traffic APIs, please read this [documentation](https://docs.microsoft.com/azure/api-management/api-management-howto-app-insights#performance-implications-and-log-sampling) to understand performance implications and log sampling. Valid values are between `0.0` and `100.0`.
        pub sampling_percentage: pulumi_gestalt_rust::Output<f64>,
        /// Logging verbosity. Possible values are `verbose`, `information` or `error`.
        pub verbosity: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DiagnosticArgs,
    ) -> DiagnosticResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let always_log_errors_binding = args.always_log_errors.get_output(context);
        let api_management_logger_id_binding = args
            .api_management_logger_id
            .get_output(context);
        let api_management_name_binding = args.api_management_name.get_output(context);
        let backend_request_binding = args.backend_request.get_output(context);
        let backend_response_binding = args.backend_response.get_output(context);
        let frontend_request_binding = args.frontend_request.get_output(context);
        let frontend_response_binding = args.frontend_response.get_output(context);
        let http_correlation_protocol_binding = args
            .http_correlation_protocol
            .get_output(context);
        let identifier_binding = args.identifier.get_output(context);
        let log_client_ip_binding = args.log_client_ip.get_output(context);
        let operation_name_format_binding = args
            .operation_name_format
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sampling_percentage_binding = args.sampling_percentage.get_output(context);
        let verbosity_binding = args.verbosity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/diagnostic:Diagnostic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alwaysLogErrors".into(),
                    value: always_log_errors_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementLoggerId".into(),
                    value: api_management_logger_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: api_management_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendRequest".into(),
                    value: backend_request_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendResponse".into(),
                    value: backend_response_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendRequest".into(),
                    value: frontend_request_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendResponse".into(),
                    value: frontend_response_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpCorrelationProtocol".into(),
                    value: http_correlation_protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logClientIp".into(),
                    value: log_client_ip_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operationNameFormat".into(),
                    value: operation_name_format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samplingPercentage".into(),
                    value: sampling_percentage_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "verbosity".into(),
                    value: verbosity_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DiagnosticResult {
            always_log_errors: o.get_field("alwaysLogErrors"),
            api_management_logger_id: o.get_field("apiManagementLoggerId"),
            api_management_name: o.get_field("apiManagementName"),
            backend_request: o.get_field("backendRequest"),
            backend_response: o.get_field("backendResponse"),
            frontend_request: o.get_field("frontendRequest"),
            frontend_response: o.get_field("frontendResponse"),
            http_correlation_protocol: o.get_field("httpCorrelationProtocol"),
            identifier: o.get_field("identifier"),
            log_client_ip: o.get_field("logClientIp"),
            operation_name_format: o.get_field("operationNameFormat"),
            resource_group_name: o.get_field("resourceGroupName"),
            sampling_percentage: o.get_field("samplingPercentage"),
            verbosity: o.get_field("verbosity"),
        }
    }
}
