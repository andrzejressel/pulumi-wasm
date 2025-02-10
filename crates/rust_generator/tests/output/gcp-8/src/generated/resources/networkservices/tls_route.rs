/// ## Example Usage
///
/// ### Network Services Tls Route Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .name("my-backend-service")
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("backend-service-health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let defaultTlsRoute = tls_route::create(
///         "defaultTlsRoute",
///         TlsRouteArgs::builder()
///             .description("my description")
///             .name("my-tls-route")
///             .rules(
///                 vec![
///                     TlsRouteRule::builder().action(TlsRouteRuleAction::builder()
///                     .destinations(vec![TlsRouteRuleActionDestination::builder()
///                     .serviceName("${default.id}").weight(1).build_struct(),])
///                     .build_struct()).matches(vec![TlsRouteRuleMatch::builder()
///                     .alpns(vec!["http/1.1",]).sniHosts(vec!["example.com",])
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Services Tls Route Mesh Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: my-backend-service
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: backend-service-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   defaultMesh:
///     type: gcp:networkservices:Mesh
///     name: default
///     properties:
///       name: my-tls-route
///       labels:
///         foo: bar
///       description: my description
///   defaultTlsRoute:
///     type: gcp:networkservices:TlsRoute
///     name: default
///     properties:
///       name: my-tls-route
///       description: my description
///       meshes:
///         - ${defaultMesh.id}
///       rules:
///         - matches:
///             - sniHosts:
///                 - example.com
///               alpns:
///                 - http/1.1
///           action:
///             destinations:
///               - serviceName: ${default.id}
///                 weight: 1
/// ```
/// ### Network Services Tls Route Gateway Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: my-backend-service
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: backend-service-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   defaultGateway:
///     type: gcp:networkservices:Gateway
///     name: default
///     properties:
///       name: my-tls-route
///       labels:
///         foo: bar
///       description: my description
///       scope: my-scope
///       type: OPEN_MESH
///       ports:
///         - 443
///   defaultTlsRoute:
///     type: gcp:networkservices:TlsRoute
///     name: default
///     properties:
///       name: my-tls-route
///       description: my description
///       gateways:
///         - ${defaultGateway.id}
///       rules:
///         - matches:
///             - sniHosts:
///                 - example.com
///               alpns:
///                 - http/1.1
///           action:
///             destinations:
///               - serviceName: ${default.id}
///                 weight: 1
/// ```
///
/// ## Import
///
/// TlsRoute can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/tlsRoutes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TlsRoute can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/tlsRoute:TlsRoute default projects/{{project}}/locations/global/tlsRoutes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/tlsRoute:TlsRoute default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/tlsRoute:TlsRoute default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tls_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TlsRouteArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Gateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests
        /// served by the gateway. Each gateway reference should match the pattern:
        /// projects/*/locations/global/gateways/<gateway_name>
        #[builder(into, default)]
        pub gateways: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Meshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served
        /// by the mesh. Each mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name> The attached
        /// Mesh should be of a type SIDECAR
        #[builder(into, default)]
        pub meshes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the TlsRoute resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rules that define how traffic is routed and handled.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::networkservices::TlsRouteRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct TlsRouteResult {
        /// Time the TlsRoute was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Gateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests
        /// served by the gateway. Each gateway reference should match the pattern:
        /// projects/*/locations/global/gateways/<gateway_name>
        pub gateways: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Meshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served
        /// by the mesh. Each mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name> The attached
        /// Mesh should be of a type SIDECAR
        pub meshes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the TlsRoute resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Rules that define how traffic is routed and handled.
        /// Structure is documented below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkservices::TlsRouteRule>,
        >,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Time the TlsRoute was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TlsRouteArgs,
    ) -> TlsRouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let gateways_binding = args.gateways.get_output(context);
        let meshes_binding = args.meshes.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networkservices/tlsRoute:TlsRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gateways".into(),
                    value: gateways_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "meshes".into(),
                    value: meshes_binding.get_id(),
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
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TlsRouteResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            gateways: o.get_field("gateways"),
            meshes: o.get_field("meshes"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            rules: o.get_field("rules"),
            self_link: o.get_field("selfLink"),
            update_time: o.get_field("updateTime"),
        }
    }
}
