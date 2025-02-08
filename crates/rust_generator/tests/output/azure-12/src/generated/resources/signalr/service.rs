/// Manages an Azure SignalR service.
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
///             .location("West US")
///             .name("my-signalr")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .connectivity_logs_enabled(true)
///             .cors(
///                 vec![
///                     ServiceCor::builder().allowedOrigins(vec!["http://www.example.com",])
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .messaging_logs_enabled(true)
///             .name("tfex-signalr")
///             .public_network_access_enabled(false)
///             .resource_group_name("${example.name}")
///             .service_mode("Default")
///             .sku(ServiceSku::builder().capacity(1).name("Free_F1").build_struct())
///             .upstream_endpoints(
///                 vec![
///                     ServiceUpstreamEndpoint::builder()
///                     .categoryPatterns(vec!["connections", "messages",])
///                     .eventPatterns(vec!["*",]).hubPatterns(vec!["hub1",])
///                     .urlTemplate("http://foo.com").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SignalR services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:signalr/service:Service example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/terraform-signalr/providers/Microsoft.SignalRService/signalR/tfex-signalr
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Whether to enable AAD auth? Defaults to `true`.
        #[builder(into, default)]
        pub aad_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies if Connectivity Logs are enabled or not. Defaults to `false`.
        #[builder(into, default)]
        pub connectivity_logs_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `cors` block as documented below.
        #[builder(into, default)]
        pub cors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::signalr::ServiceCor>>,
        >,
        /// Specifies if Http Request Logs are enabled or not. Defaults to `false`.
        #[builder(into, default)]
        pub http_request_logs_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::signalr::ServiceIdentity>,
        >,
        /// A `live_trace` block as defined below.
        #[builder(into, default)]
        pub live_trace: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::signalr::ServiceLiveTrace>,
        >,
        #[builder(into, default)]
        pub live_trace_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to enable local auth? Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the SignalR service exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if Messaging Logs are enabled or not. Defaults to `false`.
        #[builder(into, default)]
        pub messaging_logs_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the SignalR service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to enable public network access? Defaults to `true`.
        ///
        /// > **Note:** `public_network_access_enabled` cannot be set to `false` in `Free` sku tier.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the SignalR service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the client connection timeout. Defaults to `30`.
        #[builder(into, default)]
        pub serverless_connection_timeout_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Specifies the service mode. Possible values are `Classic`, `Default` and `Serverless`. Defaults to `Default`.
        #[builder(into, default)]
        pub service_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `sku` block as documented below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::signalr::ServiceSku,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to request client certificate during TLS handshake? Defaults to `false`.
        ///
        /// > **Note:** `tls_client_cert_enabled` cannot be set to `true` in `Free` sku tier.
        #[builder(into, default)]
        pub tls_client_cert_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `upstream_endpoint` block as documented below. Using this block requires the SignalR service to be Serverless. When creating multiple blocks they will be processed in the order they are defined in.
        #[builder(into, default)]
        pub upstream_endpoints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::signalr::ServiceUpstreamEndpoint>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Whether to enable AAD auth? Defaults to `true`.
        pub aad_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if Connectivity Logs are enabled or not. Defaults to `false`.
        pub connectivity_logs_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `cors` block as documented below.
        pub cors: pulumi_gestalt_rust::Output<
            Vec<super::super::types::signalr::ServiceCor>,
        >,
        /// The FQDN of the SignalR service.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// Specifies if Http Request Logs are enabled or not. Defaults to `false`.
        pub http_request_logs_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::signalr::ServiceIdentity>,
        >,
        /// The publicly accessible IP of the SignalR service.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// A `live_trace` block as defined below.
        pub live_trace: pulumi_gestalt_rust::Output<
            Option<super::super::types::signalr::ServiceLiveTrace>,
        >,
        pub live_trace_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to enable local auth? Defaults to `true`.
        pub local_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the SignalR service exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies if Messaging Logs are enabled or not. Defaults to `false`.
        pub messaging_logs_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the SignalR service. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary access key for the SignalR service.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string for the SignalR service.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable public network access? Defaults to `true`.
        ///
        /// > **Note:** `public_network_access_enabled` cannot be set to `false` in `Free` sku tier.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The publicly accessible port of the SignalR service which is designed for browser/client use.
        pub public_port: pulumi_gestalt_rust::Output<i32>,
        /// The name of the resource group in which to create the SignalR service. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key for the SignalR service.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string for the SignalR service.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The publicly accessible port of the SignalR service which is designed for customer server side use.
        pub server_port: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the client connection timeout. Defaults to `30`.
        pub serverless_connection_timeout_in_seconds: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// Specifies the service mode. Possible values are `Classic`, `Default` and `Serverless`. Defaults to `Default`.
        pub service_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `sku` block as documented below.
        pub sku: pulumi_gestalt_rust::Output<super::super::types::signalr::ServiceSku>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to request client certificate during TLS handshake? Defaults to `false`.
        ///
        /// > **Note:** `tls_client_cert_enabled` cannot be set to `true` in `Free` sku tier.
        pub tls_client_cert_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `upstream_endpoint` block as documented below. Using this block requires the SignalR service to be Serverless. When creating multiple blocks they will be processed in the order they are defined in.
        pub upstream_endpoints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::signalr::ServiceUpstreamEndpoint>>,
        >,
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
        let aad_auth_enabled_binding = args
            .aad_auth_enabled
            .get_output(context)
            .get_inner();
        let connectivity_logs_enabled_binding = args
            .connectivity_logs_enabled
            .get_output(context)
            .get_inner();
        let cors_binding = args.cors.get_output(context).get_inner();
        let http_request_logs_enabled_binding = args
            .http_request_logs_enabled
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let live_trace_binding = args.live_trace.get_output(context).get_inner();
        let live_trace_enabled_binding = args
            .live_trace_enabled
            .get_output(context)
            .get_inner();
        let local_auth_enabled_binding = args
            .local_auth_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let messaging_logs_enabled_binding = args
            .messaging_logs_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let serverless_connection_timeout_in_seconds_binding = args
            .serverless_connection_timeout_in_seconds
            .get_output(context)
            .get_inner();
        let service_mode_binding = args.service_mode.get_output(context).get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tls_client_cert_enabled_binding = args
            .tls_client_cert_enabled
            .get_output(context)
            .get_inner();
        let upstream_endpoints_binding = args
            .upstream_endpoints
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:signalr/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aadAuthEnabled".into(),
                    value: &aad_auth_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "connectivityLogsEnabled".into(),
                    value: &connectivity_logs_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "cors".into(),
                    value: &cors_binding,
                },
                register_interface::ObjectField {
                    name: "httpRequestLogsEnabled".into(),
                    value: &http_request_logs_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "liveTrace".into(),
                    value: &live_trace_binding,
                },
                register_interface::ObjectField {
                    name: "liveTraceEnabled".into(),
                    value: &live_trace_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthEnabled".into(),
                    value: &local_auth_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "messagingLogsEnabled".into(),
                    value: &messaging_logs_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serverlessConnectionTimeoutInSeconds".into(),
                    value: &serverless_connection_timeout_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "serviceMode".into(),
                    value: &service_mode_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tlsClientCertEnabled".into(),
                    value: &tls_client_cert_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "upstreamEndpoints".into(),
                    value: &upstream_endpoints_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceResult {
            aad_auth_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aadAuthEnabled"),
            ),
            connectivity_logs_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectivityLogsEnabled"),
            ),
            cors: pulumi_gestalt_rust::__private::into_domain(o.extract_field("cors")),
            hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            http_request_logs_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpRequestLogsEnabled"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            live_trace: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("liveTrace"),
            ),
            live_trace_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("liveTraceEnabled"),
            ),
            local_auth_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localAuthEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            messaging_logs_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("messagingLogsEnabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            primary_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            primary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            public_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicPort"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
            secondary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            server_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverPort"),
            ),
            serverless_connection_timeout_in_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverlessConnectionTimeoutInSeconds"),
            ),
            service_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceMode"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tls_client_cert_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsClientCertEnabled"),
            ),
            upstream_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("upstreamEndpoints"),
            ),
        }
    }
}
