/// ## Example Usage
///
/// ### Network Management Vpc Flow Logs Config Interconnect Full
///
///
/// ```yaml
/// resources:
///   interconnect-test:
///     type: gcp:networkmanagement:VpcFlowLogsConfig
///     properties:
///       vpcFlowLogsConfigId: full-interconnect-test-id
///       location: global
///       interconnectAttachment: projects/${project.number}/regions/us-east4/interconnectAttachments/${attachment.name}
///       state: ENABLED
///       aggregationInterval: INTERVAL_5_SEC
///       description: VPC Flow Logs over a VPN Gateway.
///       flowSampling: 0.5
///       metadata: INCLUDE_ALL_METADATA
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: full-interconnect-test-network
///   router:
///     type: gcp:compute:Router
///     properties:
///       name: full-interconnect-test-router
///       network: ${network.name}
///       bgp:
///         asn: 16550
///   attachment:
///     type: gcp:compute:InterconnectAttachment
///     properties:
///       name: full-interconnect-test-id
///       edgeAvailabilityDomain: AVAILABILITY_DOMAIN_1
///       type: PARTNER
///       router: ${router.id}
///       mtu: 1500
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Network Management Vpc Flow Logs Config Interconnect Basic
///
///
/// ```yaml
/// resources:
///   interconnect-test:
///     type: gcp:networkmanagement:VpcFlowLogsConfig
///     properties:
///       vpcFlowLogsConfigId: basic-interconnect-test-id
///       location: global
///       interconnectAttachment: projects/${project.number}/regions/us-east4/interconnectAttachments/${attachment.name}
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: basic-interconnect-test-network
///   router:
///     type: gcp:compute:Router
///     properties:
///       name: basic-interconnect-test-router
///       network: ${network.name}
///       bgp:
///         asn: 16550
///   attachment:
///     type: gcp:compute:InterconnectAttachment
///     properties:
///       name: basic-interconnect-test-id
///       edgeAvailabilityDomain: AVAILABILITY_DOMAIN_1
///       type: PARTNER
///       router: ${router.id}
///       mtu: 1500
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Network Management Vpc Flow Logs Config Vpn Basic
///
///
/// ```yaml
/// resources:
///   vpn-test:
///     type: gcp:networkmanagement:VpcFlowLogsConfig
///     properties:
///       vpcFlowLogsConfigId: basic-test-id
///       location: global
///       vpnTunnel: projects/${project.number}/regions/us-central1/vpnTunnels/${tunnel.name}
///   tunnel:
///     type: gcp:compute:VPNTunnel
///     properties:
///       name: basic-test-tunnel
///       peerIp: 15.0.0.120
///       sharedSecret: a secret message
///       targetVpnGateway: ${targetGateway.id}
///     options:
///       dependsOn:
///         - ${frEsp}
///         - ${frUdp500}
///         - ${frUdp4500}
///   targetGateway:
///     type: gcp:compute:VPNGateway
///     name: target_gateway
///     properties:
///       name: basic-test-gateway
///       network: ${network.id}
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: basic-test-network
///   vpnStaticIp:
///     type: gcp:compute:Address
///     name: vpn_static_ip
///     properties:
///       name: basic-test-address
///   frEsp:
///     type: gcp:compute:ForwardingRule
///     name: fr_esp
///     properties:
///       name: basic-test-fresp
///       ipProtocol: ESP
///       ipAddress: ${vpnStaticIp.address}
///       target: ${targetGateway.id}
///   frUdp500:
///     type: gcp:compute:ForwardingRule
///     name: fr_udp500
///     properties:
///       name: basic-test-fr500
///       ipProtocol: UDP
///       portRange: '500'
///       ipAddress: ${vpnStaticIp.address}
///       target: ${targetGateway.id}
///   frUdp4500:
///     type: gcp:compute:ForwardingRule
///     name: fr_udp4500
///     properties:
///       name: basic-test-fr4500
///       ipProtocol: UDP
///       portRange: '4500'
///       ipAddress: ${vpnStaticIp.address}
///       target: ${targetGateway.id}
///   route:
///     type: gcp:compute:Route
///     properties:
///       name: basic-test-route
///       network: ${network.name}
///       destRange: 15.0.0.0/24
///       priority: 1000
///       nextHopVpnTunnel: ${tunnel.id}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Network Management Vpc Flow Logs Config Vpn Full
///
///
/// ```yaml
/// resources:
///   vpn-test:
///     type: gcp:networkmanagement:VpcFlowLogsConfig
///     properties:
///       vpcFlowLogsConfigId: full-test-id
///       location: global
///       vpnTunnel: projects/${project.number}/regions/us-central1/vpnTunnels/${tunnel.name}
///       state: ENABLED
///       aggregationInterval: INTERVAL_5_SEC
///       description: VPC Flow Logs over a VPN Gateway.
///       flowSampling: 0.5
///       metadata: INCLUDE_ALL_METADATA
///   tunnel:
///     type: gcp:compute:VPNTunnel
///     properties:
///       name: full-test-tunnel
///       peerIp: 15.0.0.120
///       sharedSecret: a secret message
///       targetVpnGateway: ${targetGateway.id}
///     options:
///       dependsOn:
///         - ${frEsp}
///         - ${frUdp500}
///         - ${frUdp4500}
///   targetGateway:
///     type: gcp:compute:VPNGateway
///     name: target_gateway
///     properties:
///       name: full-test-gateway
///       network: ${network.id}
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: full-test-network
///   vpnStaticIp:
///     type: gcp:compute:Address
///     name: vpn_static_ip
///     properties:
///       name: full-test-address
///   frEsp:
///     type: gcp:compute:ForwardingRule
///     name: fr_esp
///     properties:
///       name: full-test-fresp
///       ipProtocol: ESP
///       ipAddress: ${vpnStaticIp.address}
///       target: ${targetGateway.id}
///   frUdp500:
///     type: gcp:compute:ForwardingRule
///     name: fr_udp500
///     properties:
///       name: full-test-fr500
///       ipProtocol: UDP
///       portRange: '500'
///       ipAddress: ${vpnStaticIp.address}
///       target: ${targetGateway.id}
///   frUdp4500:
///     type: gcp:compute:ForwardingRule
///     name: fr_udp4500
///     properties:
///       name: full-test-fr4500
///       ipProtocol: UDP
///       portRange: '4500'
///       ipAddress: ${vpnStaticIp.address}
///       target: ${targetGateway.id}
///   route:
///     type: gcp:compute:Route
///     properties:
///       name: full-test-route
///       network: ${network.name}
///       destRange: 15.0.0.0/24
///       priority: 1000
///       nextHopVpnTunnel: ${tunnel.id}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// VpcFlowLogsConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/vpcFlowLogsConfigs/{{vpc_flow_logs_config_id}}`
///
/// * `{{project}}/{{location}}/{{vpc_flow_logs_config_id}}`
///
/// * `{{location}}/{{vpc_flow_logs_config_id}}`
///
/// When using the `pulumi import` command, VpcFlowLogsConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkmanagement/vpcFlowLogsConfig:VpcFlowLogsConfig default projects/{{project}}/locations/{{location}}/vpcFlowLogsConfigs/{{vpc_flow_logs_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkmanagement/vpcFlowLogsConfig:VpcFlowLogsConfig default {{project}}/{{location}}/{{vpc_flow_logs_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkmanagement/vpcFlowLogsConfig:VpcFlowLogsConfig default {{location}}/{{vpc_flow_logs_config_id}}
/// ```
///
pub mod vpc_flow_logs_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcFlowLogsConfigArgs {
        /// Optional. The aggregation interval for the logs. Default value is
        /// INTERVAL_5_SEC.   Possible values:  AGGREGATION_INTERVAL_UNSPECIFIED INTERVAL_5_SEC INTERVAL_30_SEC INTERVAL_1_MIN INTERVAL_5_MIN INTERVAL_10_MIN INTERVAL_15_MIN"
        #[builder(into, default)]
        pub aggregation_interval: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. The user-supplied description of the VPC Flow Logs configuration. Maximum
        /// of 512 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. Export filter used to define which VPC Flow Logs should be logged.
        #[builder(into, default)]
        pub filter_expr: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. The value of the field must be in (0, 1]. The sampling rate
        /// of VPC Flow Logs where 1.0 means all collected logs are reported. Setting the
        /// sampling rate to 0.0 is not allowed. If you want to disable VPC Flow Logs, use
        /// the state field instead. Default value is 1.0.
        #[builder(into, default)]
        pub flow_sampling: pulumi_wasm_rust::InputOrOutput<Option<f64>>,
        /// Traffic will be logged from the Interconnect Attachment. Format: projects/{project_id}/regions/{region}/interconnectAttachments/{name}
        #[builder(into, default)]
        pub interconnect_attachment: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource
        /// within its parent collection as described in https://google.aip.dev/122. See documentation
        /// for resource type `networkmanagement.googleapis.com/VpcFlowLogsConfig`.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Optional. Configures whether all, none or a subset of metadata fields
        /// should be added to the reported VPC flow logs. Default value is INCLUDE_ALL_METADATA.
        /// Possible values:  METADATA_UNSPECIFIED INCLUDE_ALL_METADATA EXCLUDE_ALL_METADATA CUSTOM_METADATA
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. Custom metadata fields to include in the reported VPC flow
        /// logs. Can only be specified if \"metadata\" was set to CUSTOM_METADATA.
        #[builder(into, default)]
        pub metadata_fields: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. The state of the VPC Flow Log configuration. Default value
        /// is ENABLED. When creating a new configuration, it must be enabled.   Possible
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Required. ID of the `VpcFlowLogsConfig`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub vpc_flow_logs_config_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Traffic will be logged from the VPN Tunnel. Format: projects/{project_id}/regions/{region}/vpnTunnels/{name}
        #[builder(into, default)]
        pub vpn_tunnel: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcFlowLogsConfigResult {
        /// Optional. The aggregation interval for the logs. Default value is
        /// INTERVAL_5_SEC.   Possible values:  AGGREGATION_INTERVAL_UNSPECIFIED INTERVAL_5_SEC INTERVAL_30_SEC INTERVAL_1_MIN INTERVAL_5_MIN INTERVAL_10_MIN INTERVAL_15_MIN"
        pub aggregation_interval: pulumi_wasm_rust::Output<String>,
        /// Output only. The time the config was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional. The user-supplied description of the VPC Flow Logs configuration. Maximum
        /// of 512 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Export filter used to define which VPC Flow Logs should be logged.
        pub filter_expr: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. The value of the field must be in (0, 1]. The sampling rate
        /// of VPC Flow Logs where 1.0 means all collected logs are reported. Setting the
        /// sampling rate to 0.0 is not allowed. If you want to disable VPC Flow Logs, use
        /// the state field instead. Default value is 1.0.
        pub flow_sampling: pulumi_wasm_rust::Output<f64>,
        /// Traffic will be logged from the Interconnect Attachment. Format: projects/{project_id}/regions/{region}/interconnectAttachments/{name}
        pub interconnect_attachment: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource
        /// within its parent collection as described in https://google.aip.dev/122. See documentation
        /// for resource type `networkmanagement.googleapis.com/VpcFlowLogsConfig`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Optional. Configures whether all, none or a subset of metadata fields
        /// should be added to the reported VPC flow logs. Default value is INCLUDE_ALL_METADATA.
        /// Possible values:  METADATA_UNSPECIFIED INCLUDE_ALL_METADATA EXCLUDE_ALL_METADATA CUSTOM_METADATA
        pub metadata: pulumi_wasm_rust::Output<String>,
        /// Optional. Custom metadata fields to include in the reported VPC flow
        /// logs. Can only be specified if \"metadata\" was set to CUSTOM_METADATA.
        pub metadata_fields: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Identifier. Unique name of the configuration using the form:     `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. The state of the VPC Flow Log configuration. Default value
        /// is ENABLED. When creating a new configuration, it must be enabled.   Possible
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. The time the config was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Required. ID of the `VpcFlowLogsConfig`.
        ///
        ///
        /// - - -
        pub vpc_flow_logs_config_id: pulumi_wasm_rust::Output<String>,
        /// Traffic will be logged from the VPN Tunnel. Format: projects/{project_id}/regions/{region}/vpnTunnels/{name}
        pub vpn_tunnel: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpcFlowLogsConfigArgs,
    ) -> VpcFlowLogsConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aggregation_interval_binding = args
            .aggregation_interval
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let filter_expr_binding = args.filter_expr.get_output(context).get_inner();
        let flow_sampling_binding = args.flow_sampling.get_output(context).get_inner();
        let interconnect_attachment_binding = args
            .interconnect_attachment
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let metadata_fields_binding = args
            .metadata_fields
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let vpc_flow_logs_config_id_binding = args
            .vpc_flow_logs_config_id
            .get_output(context)
            .get_inner();
        let vpn_tunnel_binding = args.vpn_tunnel.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkmanagement/vpcFlowLogsConfig:VpcFlowLogsConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aggregationInterval".into(),
                    value: &aggregation_interval_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "filterExpr".into(),
                    value: &filter_expr_binding,
                },
                register_interface::ObjectField {
                    name: "flowSampling".into(),
                    value: &flow_sampling_binding,
                },
                register_interface::ObjectField {
                    name: "interconnectAttachment".into(),
                    value: &interconnect_attachment_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "metadataFields".into(),
                    value: &metadata_fields_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "vpcFlowLogsConfigId".into(),
                    value: &vpc_flow_logs_config_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpnTunnel".into(),
                    value: &vpn_tunnel_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcFlowLogsConfigResult {
            aggregation_interval: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("aggregationInterval"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            filter_expr: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filterExpr"),
            ),
            flow_sampling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("flowSampling"),
            ),
            interconnect_attachment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("interconnectAttachment"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            metadata_fields: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadataFields"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            vpc_flow_logs_config_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcFlowLogsConfigId"),
            ),
            vpn_tunnel: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpnTunnel"),
            ),
        }
    }
}
