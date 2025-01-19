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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod target_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetServerArgs {
        /// A human-readable description of this TargetServer.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub env_id: pulumi_wasm_rust::Output<String>,
        /// The host name this target connects to. Value must be a valid hostname as described by RFC-1123.
        #[builder(into)]
        pub host: pulumi_wasm_rust::Output<String>,
        /// Enabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true.
        #[builder(into, default)]
        pub is_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The resource id of this reference. Values must match the regular expression [\w\s-.]+.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive.
        #[builder(into)]
        pub port: pulumi_wasm_rust::Output<i32>,
        /// Immutable. The protocol used by this TargetServer.
        /// Possible values are: `HTTP`, `HTTP2`, `GRPC_TARGET`, `GRPC`, `EXTERNAL_CALLOUT`.
        #[builder(into, default)]
        pub protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies TLS configuration info for this TargetServer. The JSON name is sSLInfo for legacy/backwards compatibility reasons -- Edge originally supported SSL, and the name is still used for TLS configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub s_sl_info: pulumi_wasm_rust::Output<
            Option<super::super::types::apigee::TargetServerSSlInfo>,
        >,
    }
    #[allow(dead_code)]
    pub struct TargetServerResult {
        /// A human-readable description of this TargetServer.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        pub env_id: pulumi_wasm_rust::Output<String>,
        /// The host name this target connects to. Value must be a valid hostname as described by RFC-1123.
        pub host: pulumi_wasm_rust::Output<String>,
        /// Enabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true.
        pub is_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The resource id of this reference. Values must match the regular expression [\w\s-.]+.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// Immutable. The protocol used by this TargetServer.
        /// Possible values are: `HTTP`, `HTTP2`, `GRPC_TARGET`, `GRPC`, `EXTERNAL_CALLOUT`.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Specifies TLS configuration info for this TargetServer. The JSON name is sSLInfo for legacy/backwards compatibility reasons -- Edge originally supported SSL, and the name is still used for TLS configuration.
        /// Structure is documented below.
        pub s_sl_info: pulumi_wasm_rust::Output<
            Option<super::super::types::apigee::TargetServerSSlInfo>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TargetServerArgs) -> TargetServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let env_id_binding = args.env_id.get_inner();
        let host_binding = args.host.get_inner();
        let is_enabled_binding = args.is_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let port_binding = args.port.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let s_sl_info_binding = args.s_sl_info.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/targetServer:TargetServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "envId".into(),
                    value: &env_id_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "isEnabled".into(),
                    value: &is_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "sSlInfo".into(),
                    value: &s_sl_info_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "envId".into(),
                },
                register_interface::ResultField {
                    name: "host".into(),
                },
                register_interface::ResultField {
                    name: "isEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "sSlInfo".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TargetServerResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            env_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("envId").unwrap(),
            ),
            host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("host").unwrap(),
            ),
            is_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            s_sl_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sSlInfo").unwrap(),
            ),
        }
    }
}
