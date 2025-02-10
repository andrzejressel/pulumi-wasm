/// TargetServer configuration. TargetServers are used to decouple a proxy TargetEndpoint HTTPTargetConnections from concrete URLs for backend services.
///
///
/// To get more information about TargetServer, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.targetservers/create)
/// * How-to Guides
///     * [Load balancing across backend servers](https://cloud.google.com/apigee/docs/api-platform/deploy/load-balancing-across-backend-servers)
///
/// ## Example Usage
///
/// ### Apigee Target Server Test Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let apigee = service::create(
///         "apigee",
///         ServiceArgs::builder()
///             .project("${project.projectId}")
///             .service("apigee.googleapis.com")
///             .build_struct(),
///     );
///     let apigeeEnvironment = environment::create(
///         "apigeeEnvironment",
///         EnvironmentArgs::builder()
///             .description("Apigee Environment")
///             .display_name("environment-1")
///             .name("my-environment-name")
///             .org_id("${apigeeOrg.id}")
///             .build_struct(),
///     );
///     let apigeeNetwork = network::create(
///         "apigeeNetwork",
///         NetworkArgs::builder()
///             .name("apigee-network")
///             .project("${project.projectId}")
///             .build_struct(),
///     );
///     let apigeeOrg = organization::create(
///         "apigeeOrg",
///         OrganizationArgs::builder()
///             .analytics_region("us-central1")
///             .authorized_network("${apigeeNetwork.id}")
///             .project_id("${project.projectId}")
///             .build_struct(),
///     );
///     let apigeeRange = global_address::create(
///         "apigeeRange",
///         GlobalAddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("apigee-range")
///             .network("${apigeeNetwork.id}")
///             .prefix_length(16)
///             .project("${project.projectId}")
///             .purpose("VPC_PEERING")
///             .build_struct(),
///     );
///     let apigeeTargetServer = target_server::create(
///         "apigeeTargetServer",
///         TargetServerArgs::builder()
///             .description("Apigee Target Server")
///             .env_id("${apigeeEnvironment.id}")
///             .host("abc.foo.com")
///             .name("my-target-server")
///             .port(8080)
///             .protocol("HTTP")
///             .build_struct(),
///     );
///     let apigeeVpcConnection = connection::create(
///         "apigeeVpcConnection",
///         ConnectionArgs::builder()
///             .network("${apigeeNetwork.id}")
///             .reserved_peering_ranges(vec!["${apigeeRange.name}",])
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
///     let compute = service::create(
///         "compute",
///         ServiceArgs::builder()
///             .project("${project.projectId}")
///             .service("compute.googleapis.com")
///             .build_struct(),
///     );
///     let project = project::create(
///         "project",
///         ProjectArgs::builder()
///             .billing_account("000000-0000000-0000000-000000")
///             .deletion_policy("DELETE")
///             .name("my-project")
///             .org_id("123456789")
///             .project_id("my-project")
///             .build_struct(),
///     );
///     let servicenetworking = service::create(
///         "servicenetworking",
///         ServiceArgs::builder()
///             .project("${project.projectId}")
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TargetServer can be imported using any of these accepted formats:
///
/// * `{{env_id}}/targetservers/{{name}}`
///
/// * `{{env_id}}/{{name}}`
///
/// When using the `pulumi import` command, TargetServer can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/targetServer:TargetServer default {{env_id}}/targetservers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/targetServer:TargetServer default {{env_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetServerArgs {
        /// A human-readable description of this TargetServer.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub env_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The host name this target connects to. Value must be a valid hostname as described by RFC-1123.
        #[builder(into)]
        pub host: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true.
        #[builder(into, default)]
        pub is_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The resource id of this reference. Values must match the regular expression [\w\s-.]+.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive.
        #[builder(into)]
        pub port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Immutable. The protocol used by this TargetServer.
        /// Possible values are: `HTTP`, `HTTP2`, `GRPC_TARGET`, `GRPC`, `EXTERNAL_CALLOUT`.
        #[builder(into, default)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies TLS configuration info for this TargetServer. The JSON name is sSLInfo for legacy/backwards compatibility reasons -- Edge originally supported SSL, and the name is still used for TLS configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub s_sl_info: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigee::TargetServerSSlInfo>,
        >,
    }
    #[allow(dead_code)]
    pub struct TargetServerResult {
        /// A human-readable description of this TargetServer.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        pub env_id: pulumi_gestalt_rust::Output<String>,
        /// The host name this target connects to. Value must be a valid hostname as described by RFC-1123.
        pub host: pulumi_gestalt_rust::Output<String>,
        /// Enabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true.
        pub is_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The resource id of this reference. Values must match the regular expression [\w\s-.]+.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Immutable. The protocol used by this TargetServer.
        /// Possible values are: `HTTP`, `HTTP2`, `GRPC_TARGET`, `GRPC`, `EXTERNAL_CALLOUT`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// Specifies TLS configuration info for this TargetServer. The JSON name is sSLInfo for legacy/backwards compatibility reasons -- Edge originally supported SSL, and the name is still used for TLS configuration.
        /// Structure is documented below.
        pub s_sl_info: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigee::TargetServerSSlInfo>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetServerArgs,
    ) -> TargetServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let env_id_binding = args.env_id.get_output(context);
        let host_binding = args.host.get_output(context);
        let is_enabled_binding = args.is_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let port_binding = args.port.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let s_sl_info_binding = args.s_sl_info.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/targetServer:TargetServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envId".into(),
                    value: env_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "host".into(),
                    value: host_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isEnabled".into(),
                    value: is_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sSlInfo".into(),
                    value: s_sl_info_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetServerResult {
            description: o.get_field("description"),
            env_id: o.get_field("envId"),
            host: o.get_field("host"),
            is_enabled: o.get_field("isEnabled"),
            name: o.get_field("name"),
            port: o.get_field("port"),
            protocol: o.get_field("protocol"),
            s_sl_info: o.get_field("sSlInfo"),
        }
    }
}
