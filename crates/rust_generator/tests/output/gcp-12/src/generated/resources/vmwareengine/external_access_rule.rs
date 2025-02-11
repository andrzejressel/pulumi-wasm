/// External access firewall rules for filtering incoming traffic destined to `ExternalAddress` resources.
///
///
/// To get more information about ExternalAccessRule, see:
///
/// * [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.networkPolicies.externalAccessRules)
///
/// ## Example Usage
///
/// ### Vmware Engine External Access Rule Basic
///
///
/// ```yaml
/// resources:
///   external-access-rule-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: sample-nw
///       location: global
///       type: STANDARD
///       description: PC network description.
///   external-access-rule-np:
///     type: gcp:vmwareengine:NetworkPolicy
///     properties:
///       location: us-west1
///       name: sample-np
///       edgeServicesCidr: 192.168.30.0/26
///       vmwareEngineNetwork: ${["external-access-rule-nw"].id}
///   vmw-engine-external-access-rule:
///     type: gcp:vmwareengine:ExternalAccessRule
///     properties:
///       name: sample-external-access-rule
///       parent: ${["external-access-rule-np"].id}
///       priority: 101
///       action: DENY
///       ipProtocol: TCP
///       sourceIpRanges:
///         - ipAddressRange: 0.0.0.0/0
///       sourcePorts:
///         - '80'
///       destinationIpRanges:
///         - ipAddressRange: 0.0.0.0/0
///       destinationPorts:
///         - '433'
/// ```
/// ### Vmware Engine External Access Rule Full
///
///
/// ```yaml
/// resources:
///   external-access-rule-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: sample-nw
///       location: global
///       type: STANDARD
///       description: PC network description.
///   external-access-rule-pc:
///     type: gcp:vmwareengine:PrivateCloud
///     properties:
///       location: us-west1-a
///       name: sample-pc
///       description: Sample test PC.
///       networkConfig:
///         managementCidr: 192.168.50.0/24
///         vmwareEngineNetwork: ${["external-access-rule-nw"].id}
///       managementCluster:
///         clusterId: sample-mgmt-cluster
///         nodeTypeConfigs:
///           - nodeTypeId: standard-72
///             nodeCount: 3
///   external-access-rule-np:
///     type: gcp:vmwareengine:NetworkPolicy
///     properties:
///       location: us-west1
///       name: sample-np
///       edgeServicesCidr: 192.168.30.0/26
///       vmwareEngineNetwork: ${["external-access-rule-nw"].id}
///   external-access-rule-ea:
///     type: gcp:vmwareengine:ExternalAddress
///     properties:
///       name: sample-ea
///       parent: ${["external-access-rule-pc"].id}
///       internalIp: 192.168.0.65
///   vmw-engine-external-access-rule:
///     type: gcp:vmwareengine:ExternalAccessRule
///     properties:
///       name: sample-external-access-rule
///       parent: ${["external-access-rule-np"].id}
///       description: Sample Description
///       priority: 101
///       action: ALLOW
///       ipProtocol: tcp
///       sourceIpRanges:
///         - ipAddressRange: 0.0.0.0/0
///       sourcePorts:
///         - '80'
///       destinationIpRanges:
///         - externalAddress: ${["external-access-rule-ea"].id}
///       destinationPorts:
///         - '433'
/// ```
///
/// ## Import
///
/// ExternalAccessRule can be imported using any of these accepted formats:
///
/// * `{{parent}}/externalAccessRules/{{name}}`
///
/// When using the `pulumi import` command, ExternalAccessRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/externalAccessRule:ExternalAccessRule default {{parent}}/externalAccessRules/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod external_access_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExternalAccessRuleArgs {
        /// The action that the external access rule performs.
        /// Possible values are: `ALLOW`, `DENY`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-provided description for the external access rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If destination ranges are specified, the external access rule applies only to
        /// traffic that has a destination IP address in these ranges.
        /// Structure is documented below.
        #[builder(into)]
        pub destination_ip_ranges: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::vmwareengine::ExternalAccessRuleDestinationIpRange>,
        >,
        /// A list of destination ports to which the external access rule applies.
        #[builder(into)]
        pub destination_ports: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The IP protocol to which the external access rule applies.
        #[builder(into)]
        pub ip_protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the external access rule.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name of the network policy.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// External access rule priority, which determines the external access rule to use when multiple rules apply.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// If source ranges are specified, the external access rule applies only to
        /// traffic that has a source IP address in these ranges.
        /// Structure is documented below.
        #[builder(into)]
        pub source_ip_ranges: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::vmwareengine::ExternalAccessRuleSourceIpRange>,
        >,
        /// A list of source ports to which the external access rule applies.
        #[builder(into)]
        pub source_ports: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct ExternalAccessRuleResult {
        /// The action that the external access rule performs.
        /// Possible values are: `ALLOW`, `DENY`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// Creation time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// User-provided description for the external access rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If destination ranges are specified, the external access rule applies only to
        /// traffic that has a destination IP address in these ranges.
        /// Structure is documented below.
        pub destination_ip_ranges: pulumi_gestalt_rust::Output<
            Vec<super::super::types::vmwareengine::ExternalAccessRuleDestinationIpRange>,
        >,
        /// A list of destination ports to which the external access rule applies.
        pub destination_ports: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The IP protocol to which the external access rule applies.
        pub ip_protocol: pulumi_gestalt_rust::Output<String>,
        /// The ID of the external access rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the network policy.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// External access rule priority, which determines the external access rule to use when multiple rules apply.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// If source ranges are specified, the external access rule applies only to
        /// traffic that has a source IP address in these ranges.
        /// Structure is documented below.
        pub source_ip_ranges: pulumi_gestalt_rust::Output<
            Vec<super::super::types::vmwareengine::ExternalAccessRuleSourceIpRange>,
        >,
        /// A list of source ports to which the external access rule applies.
        pub source_ports: pulumi_gestalt_rust::Output<Vec<String>>,
        /// State of the Cluster.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Last updated time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExternalAccessRuleArgs,
    ) -> ExternalAccessRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let description_binding = args.description.get_output(context);
        let destination_ip_ranges_binding = args
            .destination_ip_ranges
            .get_output(context);
        let destination_ports_binding = args.destination_ports.get_output(context);
        let ip_protocol_binding = args.ip_protocol.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let source_ip_ranges_binding = args.source_ip_ranges.get_output(context);
        let source_ports_binding = args.source_ports.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vmwareengine/externalAccessRule:ExternalAccessRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: &action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationIpRanges".into(),
                    value: &destination_ip_ranges_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationPorts".into(),
                    value: &destination_ports_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipProtocol".into(),
                    value: &ip_protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceIpRanges".into(),
                    value: &source_ip_ranges_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourcePorts".into(),
                    value: &source_ports_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExternalAccessRuleResult {
            action: o.get_field("action"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            destination_ip_ranges: o.get_field("destinationIpRanges"),
            destination_ports: o.get_field("destinationPorts"),
            ip_protocol: o.get_field("ipProtocol"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            priority: o.get_field("priority"),
            source_ip_ranges: o.get_field("sourceIpRanges"),
            source_ports: o.get_field("sourcePorts"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
