/// Represents a TargetTcpProxy resource, which is used by one or more
/// global forwarding rule to route incoming TCP requests to a Backend
/// service.
///
///
/// To get more information about TargetTcpProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/targetTcpProxies)
/// * How-to Guides
///     * [Setting Up TCP proxy for Google Cloud Load Balancing](https://cloud.google.com/compute/docs/load-balancing/tcp-ssl/tcp-proxy)
///
/// ## Example Usage
///
/// ### Target Tcp Proxy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:TargetTCPProxy
///     properties:
///       name: test-proxy
///       backendService: ${defaultBackendService.id}
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend-service
///       protocol: TCP
///       timeoutSec: 10
///       healthChecks: ${defaultHealthCheck.id}
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       tcpHealthCheck:
///         port: '443'
/// ```
///
/// ## Import
///
/// TargetTcpProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/targetTcpProxies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TargetTcpProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetTCPProxy:TargetTCPProxy default projects/{{project}}/global/targetTcpProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetTCPProxy:TargetTCPProxy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetTCPProxy:TargetTCPProxy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_tcp_proxy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetTCPProxyArgs {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backend_service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        #[builder(into, default)]
        pub proxy_bind: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        #[builder(into, default)]
        pub proxy_header: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TargetTCPProxyResult {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        pub backend_service: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        pub proxy_bind: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        pub proxy_header: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_gestalt_rust::Output<i32>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetTCPProxyArgs,
    ) -> TargetTCPProxyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backend_service_binding = args.backend_service.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let proxy_bind_binding = args.proxy_bind.get_output(context);
        let proxy_header_binding = args.proxy_header.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/targetTCPProxy:TargetTCPProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendService".into(),
                    value: backend_service_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxyBind".into(),
                    value: proxy_bind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxyHeader".into(),
                    value: proxy_header_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetTCPProxyResult {
            backend_service: o.get_field("backendService"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            proxy_bind: o.get_field("proxyBind"),
            proxy_header: o.get_field("proxyHeader"),
            proxy_id: o.get_field("proxyId"),
            self_link: o.get_field("selfLink"),
        }
    }
}
