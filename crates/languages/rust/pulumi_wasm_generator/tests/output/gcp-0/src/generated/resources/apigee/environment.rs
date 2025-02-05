/// An `Environment` in Apigee.
///
///
/// To get more information about Environment, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Example Usage
///
/// ### Apigee Environment Basic
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   env:
///     type: gcp:apigee:Environment
///     properties:
///       name: my-environment
///       description: Apigee Environment
///       displayName: environment-1
///       orgId: ${apigeeOrg.id}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Environment can be imported using any of these accepted formats:
///
/// * `{{org_id}}/environments/{{name}}`
///
/// * `{{org_id}}/{{name}}`
///
/// When using the `pulumi import` command, Environment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/environment:Environment default {{org_id}}/environments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/environment:Environment default {{org_id}}/{{name}}
/// ```
///
pub mod environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// Optional. API Proxy type supported by the environment. The type can be set when creating
        /// the Environment and cannot be changed.
        /// Possible values are: `API_PROXY_TYPE_UNSPECIFIED`, `PROGRAMMABLE`, `CONFIGURABLE`.
        #[builder(into, default)]
        pub api_proxy_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. Deployment type supported by the environment. The deployment type can be
        /// set when creating the environment and cannot be changed. When you enable archive
        /// deployment, you will be prevented from performing a subset of actions within the
        /// environment, including:
        /// Managing the deployment of API proxy or shared flow revisions;
        /// Creating, updating, or deleting resource files;
        /// Creating, updating, or deleting target servers.
        /// Possible values are: `DEPLOYMENT_TYPE_UNSPECIFIED`, `PROXY`, `ARCHIVE`.
        #[builder(into, default)]
        pub deployment_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Description of the environment.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Display name of the environment.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. URI of the forward proxy to be applied to the runtime instances in this environment. Must be in the format of {scheme}://{hostname}:{port}. Note that the scheme must be one of "http" or "https", and the port must be supplied.
        #[builder(into, default)]
        pub forward_proxy_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the environment.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// NodeConfig for setting the min/max number of nodes associated with the environment.
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apigee::EnvironmentNodeConfig>,
        >,
        /// The Apigee Organization associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Types that can be selected for an Environment. Each of the types are
        /// limited by capability and capacity. Refer to Apigee's public documentation
        /// to understand about each of these types in details.
        /// An Apigee org can support heterogeneous Environments.
        /// Possible values are: `ENVIRONMENT_TYPE_UNSPECIFIED`, `BASE`, `INTERMEDIATE`, `COMPREHENSIVE`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// Optional. API Proxy type supported by the environment. The type can be set when creating
        /// the Environment and cannot be changed.
        /// Possible values are: `API_PROXY_TYPE_UNSPECIFIED`, `PROGRAMMABLE`, `CONFIGURABLE`.
        pub api_proxy_type: pulumi_wasm_rust::Output<String>,
        /// Optional. Deployment type supported by the environment. The deployment type can be
        /// set when creating the environment and cannot be changed. When you enable archive
        /// deployment, you will be prevented from performing a subset of actions within the
        /// environment, including:
        /// Managing the deployment of API proxy or shared flow revisions;
        /// Creating, updating, or deleting resource files;
        /// Creating, updating, or deleting target servers.
        /// Possible values are: `DEPLOYMENT_TYPE_UNSPECIFIED`, `PROXY`, `ARCHIVE`.
        pub deployment_type: pulumi_wasm_rust::Output<String>,
        /// Description of the environment.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Display name of the environment.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. URI of the forward proxy to be applied to the runtime instances in this environment. Must be in the format of {scheme}://{hostname}:{port}. Note that the scheme must be one of "http" or "https", and the port must be supplied.
        pub forward_proxy_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the environment.
        pub name: pulumi_wasm_rust::Output<String>,
        /// NodeConfig for setting the min/max number of nodes associated with the environment.
        /// Structure is documented below.
        pub node_config: pulumi_wasm_rust::Output<
            super::super::types::apigee::EnvironmentNodeConfig,
        >,
        /// The Apigee Organization associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Types that can be selected for an Environment. Each of the types are
        /// limited by capability and capacity. Refer to Apigee's public documentation
        /// to understand about each of these types in details.
        /// An Apigee org can support heterogeneous Environments.
        /// Possible values are: `ENVIRONMENT_TYPE_UNSPECIFIED`, `BASE`, `INTERMEDIATE`, `COMPREHENSIVE`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_proxy_type_binding = args.api_proxy_type.get_output(context).get_inner();
        let deployment_type_binding = args
            .deployment_type
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let forward_proxy_uri_binding = args
            .forward_proxy_uri
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_config_binding = args.node_config.get_output(context).get_inner();
        let org_id_binding = args.org_id.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiProxyType".into(),
                    value: &api_proxy_type_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentType".into(),
                    value: &deployment_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "forwardProxyUri".into(),
                    value: &forward_proxy_uri_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeConfig".into(),
                    value: &node_config_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvironmentResult {
            api_proxy_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiProxyType"),
            ),
            deployment_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deploymentType"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            forward_proxy_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forwardProxyUri"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            node_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeConfig"),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("orgId")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
