/// Manages an API Management Service Diagnostic.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod diagnostic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiagnosticArgs {
        /// Always log errors. Send telemetry if there is an erroneous condition, regardless of sampling settings.
        #[builder(into, default)]
        pub always_log_errors: pulumi_wasm_rust::Output<Option<bool>>,
        /// The id of the target API Management Logger where the API Management Diagnostic should be saved.
        #[builder(into)]
        pub api_management_logger_id: pulumi_wasm_rust::Output<String>,
        /// The Name of the API Management Service where this Diagnostic should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// A `backend_request` block as defined below.
        #[builder(into, default)]
        pub backend_request: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::DiagnosticBackendRequest>,
        >,
        /// A `backend_response` block as defined below.
        #[builder(into, default)]
        pub backend_response: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::DiagnosticBackendResponse>,
        >,
        /// A `frontend_request` block as defined below.
        #[builder(into, default)]
        pub frontend_request: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::DiagnosticFrontendRequest>,
        >,
        /// A `frontend_response` block as defined below.
        #[builder(into, default)]
        pub frontend_response: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::DiagnosticFrontendResponse>,
        >,
        /// The HTTP Correlation Protocol to use. Possible values are `None`, `Legacy` or `W3C`.
        #[builder(into, default)]
        pub http_correlation_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// The diagnostic identifier for the API Management Service. At this time the supported values are `applicationinsights` and `azuremonitor`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// Log client IP address.
        #[builder(into, default)]
        pub log_client_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// The format of the Operation Name for Application Insights telemetries. Possible values are `Name`, and `Url`.
        #[builder(into, default)]
        pub operation_name_format: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Sampling (%). For high traffic APIs, please read this [documentation](https://docs.microsoft.com/azure/api-management/api-management-howto-app-insights#performance-implications-and-log-sampling) to understand performance implications and log sampling. Valid values are between `0.0` and `100.0`.
        #[builder(into, default)]
        pub sampling_percentage: pulumi_wasm_rust::Output<Option<f64>>,
        /// Logging verbosity. Possible values are `verbose`, `information` or `error`.
        #[builder(into, default)]
        pub verbosity: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DiagnosticResult {
        /// Always log errors. Send telemetry if there is an erroneous condition, regardless of sampling settings.
        pub always_log_errors: pulumi_wasm_rust::Output<bool>,
        /// The id of the target API Management Logger where the API Management Diagnostic should be saved.
        pub api_management_logger_id: pulumi_wasm_rust::Output<String>,
        /// The Name of the API Management Service where this Diagnostic should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// A `backend_request` block as defined below.
        pub backend_request: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::DiagnosticBackendRequest,
        >,
        /// A `backend_response` block as defined below.
        pub backend_response: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::DiagnosticBackendResponse,
        >,
        /// A `frontend_request` block as defined below.
        pub frontend_request: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::DiagnosticFrontendRequest,
        >,
        /// A `frontend_response` block as defined below.
        pub frontend_response: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::DiagnosticFrontendResponse,
        >,
        /// The HTTP Correlation Protocol to use. Possible values are `None`, `Legacy` or `W3C`.
        pub http_correlation_protocol: pulumi_wasm_rust::Output<String>,
        /// The diagnostic identifier for the API Management Service. At this time the supported values are `applicationinsights` and `azuremonitor`. Changing this forces a new resource to be created.
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// Log client IP address.
        pub log_client_ip: pulumi_wasm_rust::Output<bool>,
        /// The format of the Operation Name for Application Insights telemetries. Possible values are `Name`, and `Url`.
        pub operation_name_format: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Sampling (%). For high traffic APIs, please read this [documentation](https://docs.microsoft.com/azure/api-management/api-management-howto-app-insights#performance-implications-and-log-sampling) to understand performance implications and log sampling. Valid values are between `0.0` and `100.0`.
        pub sampling_percentage: pulumi_wasm_rust::Output<f64>,
        /// Logging verbosity. Possible values are `verbose`, `information` or `error`.
        pub verbosity: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DiagnosticArgs) -> DiagnosticResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let always_log_errors_binding = args.always_log_errors.get_inner();
        let api_management_logger_id_binding = args.api_management_logger_id.get_inner();
        let api_management_name_binding = args.api_management_name.get_inner();
        let backend_request_binding = args.backend_request.get_inner();
        let backend_response_binding = args.backend_response.get_inner();
        let frontend_request_binding = args.frontend_request.get_inner();
        let frontend_response_binding = args.frontend_response.get_inner();
        let http_correlation_protocol_binding = args
            .http_correlation_protocol
            .get_inner();
        let identifier_binding = args.identifier.get_inner();
        let log_client_ip_binding = args.log_client_ip.get_inner();
        let operation_name_format_binding = args.operation_name_format.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sampling_percentage_binding = args.sampling_percentage.get_inner();
        let verbosity_binding = args.verbosity.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/diagnostic:Diagnostic".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alwaysLogErrors".into(),
                    value: &always_log_errors_binding,
                },
                register_interface::ObjectField {
                    name: "apiManagementLoggerId".into(),
                    value: &api_management_logger_id_binding,
                },
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "backendRequest".into(),
                    value: &backend_request_binding,
                },
                register_interface::ObjectField {
                    name: "backendResponse".into(),
                    value: &backend_response_binding,
                },
                register_interface::ObjectField {
                    name: "frontendRequest".into(),
                    value: &frontend_request_binding,
                },
                register_interface::ObjectField {
                    name: "frontendResponse".into(),
                    value: &frontend_response_binding,
                },
                register_interface::ObjectField {
                    name: "httpCorrelationProtocol".into(),
                    value: &http_correlation_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "logClientIp".into(),
                    value: &log_client_ip_binding,
                },
                register_interface::ObjectField {
                    name: "operationNameFormat".into(),
                    value: &operation_name_format_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "samplingPercentage".into(),
                    value: &sampling_percentage_binding,
                },
                register_interface::ObjectField {
                    name: "verbosity".into(),
                    value: &verbosity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alwaysLogErrors".into(),
                },
                register_interface::ResultField {
                    name: "apiManagementLoggerId".into(),
                },
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "backendRequest".into(),
                },
                register_interface::ResultField {
                    name: "backendResponse".into(),
                },
                register_interface::ResultField {
                    name: "frontendRequest".into(),
                },
                register_interface::ResultField {
                    name: "frontendResponse".into(),
                },
                register_interface::ResultField {
                    name: "httpCorrelationProtocol".into(),
                },
                register_interface::ResultField {
                    name: "identifier".into(),
                },
                register_interface::ResultField {
                    name: "logClientIp".into(),
                },
                register_interface::ResultField {
                    name: "operationNameFormat".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "samplingPercentage".into(),
                },
                register_interface::ResultField {
                    name: "verbosity".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DiagnosticResult {
            always_log_errors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alwaysLogErrors").unwrap(),
            ),
            api_management_logger_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementLoggerId").unwrap(),
            ),
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            backend_request: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendRequest").unwrap(),
            ),
            backend_response: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendResponse").unwrap(),
            ),
            frontend_request: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendRequest").unwrap(),
            ),
            frontend_response: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendResponse").unwrap(),
            ),
            http_correlation_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpCorrelationProtocol").unwrap(),
            ),
            identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identifier").unwrap(),
            ),
            log_client_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logClientIp").unwrap(),
            ),
            operation_name_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operationNameFormat").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sampling_percentage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samplingPercentage").unwrap(),
            ),
            verbosity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verbosity").unwrap(),
            ),
        }
    }
}
