/// Each network has its own firewall controlling access to and from the
/// instances.
///
/// All traffic to instances, even from other instances, is blocked by the
/// firewall unless firewall rules are created to allow it.
///
/// The default network has automatically created firewall rules that are
/// shown in default firewall rules. No manually created network has
/// automatically created firewall rules except for a default "allow" rule for
/// outgoing traffic and a default "deny" for incoming traffic. For all
/// networks except the default network, you must create any firewall rules
/// you need.
///
///
/// To get more information about Firewall, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/firewalls)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vpc/docs/firewalls)
///
/// ## Example Usage
///
/// ### Firewall Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = firewall::create(
///         "default",
///         FirewallArgs::builder()
///             .allows(
///                 vec![
///                     FirewallAllow::builder().protocol("icmp").build_struct(),
///                     FirewallAllow::builder().ports(vec!["80", "8080", "1000-2000",])
///                     .protocol("tcp").build_struct(),
///                 ],
///             )
///             .name("test-firewall")
///             .network("${defaultNetwork.name}")
///             .source_tags(vec!["web",])
///             .build_struct(),
///     );
///     let defaultNetwork = network::create(
///         "defaultNetwork",
///         NetworkArgs::builder().name("test-network").build_struct(),
///     );
/// }
/// ```
/// ### Firewall With Target Tags
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let rules = firewall::create(
///         "rules",
///         FirewallArgs::builder()
///             .allows(
///                 vec![
///                     FirewallAllow::builder().ports(vec!["80", "8080", "1000-2000",])
///                     .protocol("tcp").build_struct(),
///                 ],
///             )
///             .description("Creates firewall rule targeting tagged instances")
///             .name("my-firewall-rule")
///             .network("default")
///             .project("my-project-name")
///             .source_tags(vec!["foo",])
///             .target_tags(vec!["web",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Firewall can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/firewalls/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Firewall can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/firewall:Firewall default projects/{{project}}/global/firewalls/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/firewall:Firewall default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/firewall:Firewall default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallArgs {
        /// The list of ALLOW rules specified by this firewall. Each rule
        /// specifies a protocol and port-range tuple that describes a permitted
        /// connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub allows: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::FirewallAllow>>,
        >,
        /// The list of DENY rules specified by this firewall. Each rule specifies
        /// a protocol and port-range tuple that describes a denied connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub denies: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::FirewallDeny>>,
        >,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If destination ranges are specified, the firewall will apply only to
        /// traffic that has destination IP address in these ranges. These ranges
        /// must be expressed in CIDR format. IPv4 or IPv6 ranges are supported.
        #[builder(into, default)]
        pub destination_ranges: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Direction of traffic to which this firewall applies; default is
        /// INGRESS. Note: For INGRESS traffic, one of `source_ranges`,
        /// `source_tags` or `source_service_accounts` is required.
        /// Possible values are: `INGRESS`, `EGRESS`.
        #[builder(into, default)]
        pub direction: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Denotes whether the firewall rule is disabled, i.e not applied to the
        /// network it is associated with. When set to true, the firewall rule is
        /// not enforced and the network behaves as if it did not exist. If this
        /// is unspecified, the firewall rule will be enabled.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// This field denotes whether to enable logging for a particular firewall rule.
        /// If logging is enabled, logs will be exported to Stackdriver. Deprecated in favor of `log_config`
        #[builder(into, default)]
        pub enable_logging: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// This field denotes the logging options for a particular firewall rule.
        /// If defined, logging is enabled, and logs will be exported to Cloud Logging.
        /// Structure is documented below.
        #[builder(into, default)]
        pub log_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::FirewallLogConfig>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name or self_link of the network to attach this firewall to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Priority for this rule. This is an integer between 0 and 65535, both
        /// inclusive. When not specified, the value assumed is 1000. Relative
        /// priorities determine precedence of conflicting rules. Lower value of
        /// priority implies higher precedence (eg, a rule with priority 0 has
        /// higher precedence than a rule with priority 1). DENY rules take
        /// precedence over ALLOW rules having equal priority.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If source ranges are specified, the firewall will apply only to
        /// traffic that has source IP address in these ranges. These ranges must
        /// be expressed in CIDR format. One or both of sourceRanges and
        /// sourceTags may be set. If both properties are set, the firewall will
        /// apply to traffic that has source IP address within sourceRanges OR the
        /// source IP that belongs to a tag listed in the sourceTags property. The
        /// connection does not need to match both properties for the firewall to
        /// apply. IPv4 or IPv6 ranges are supported. For INGRESS traffic, one of
        /// `source_ranges`, `source_tags` or `source_service_accounts` is required.
        #[builder(into, default)]
        pub source_ranges: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// If source service accounts are specified, the firewall will apply only
        /// to traffic originating from an instance with a service account in this
        /// list. Source service accounts cannot be used to control traffic to an
        /// instance's external IP address because service accounts are associated
        /// with an instance, not an IP address. sourceRanges can be set at the
        /// same time as sourceServiceAccounts. If both are set, the firewall will
        /// apply to traffic that has source IP address within sourceRanges OR the
        /// source IP belongs to an instance with service account listed in
        /// sourceServiceAccount. The connection does not need to match both
        /// properties for the firewall to apply. sourceServiceAccounts cannot be
        /// used at the same time as sourceTags or targetTags. For INGRESS traffic,
        /// one of `source_ranges`, `source_tags` or `source_service_accounts` is required.
        #[builder(into, default)]
        pub source_service_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// If source tags are specified, the firewall will apply only to traffic
        /// with source IP that belongs to a tag listed in source tags. Source
        /// tags cannot be used to control traffic to an instance's external IP
        /// address. Because tags are associated with an instance, not an IP
        /// address. One or both of sourceRanges and sourceTags may be set. If
        /// both properties are set, the firewall will apply to traffic that has
        /// source IP address within sourceRanges OR the source IP that belongs to
        /// a tag listed in the sourceTags property. The connection does not need
        /// to match both properties for the firewall to apply. For INGRESS traffic,
        /// one of `source_ranges`, `source_tags` or `source_service_accounts` is required.
        #[builder(into, default)]
        pub source_tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of service accounts indicating sets of instances located in the
        /// network that may make network connections as specified in allowed[].
        /// targetServiceAccounts cannot be used at the same time as targetTags or
        /// sourceTags. If neither targetServiceAccounts nor targetTags are
        /// specified, the firewall rule applies to all instances on the specified
        /// network.
        #[builder(into, default)]
        pub target_service_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A list of instance tags indicating sets of instances located in the
        /// network that may make network connections as specified in allowed[].
        /// If no targetTags are specified, the firewall rule applies to all
        /// instances on the specified network.
        #[builder(into, default)]
        pub target_tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct FirewallResult {
        /// The list of ALLOW rules specified by this firewall. Each rule
        /// specifies a protocol and port-range tuple that describes a permitted
        /// connection.
        /// Structure is documented below.
        pub allows: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::FirewallAllow>>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// The list of DENY rules specified by this firewall. Each rule specifies
        /// a protocol and port-range tuple that describes a denied connection.
        /// Structure is documented below.
        pub denies: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::FirewallDeny>>,
        >,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If destination ranges are specified, the firewall will apply only to
        /// traffic that has destination IP address in these ranges. These ranges
        /// must be expressed in CIDR format. IPv4 or IPv6 ranges are supported.
        pub destination_ranges: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Direction of traffic to which this firewall applies; default is
        /// INGRESS. Note: For INGRESS traffic, one of `source_ranges`,
        /// `source_tags` or `source_service_accounts` is required.
        /// Possible values are: `INGRESS`, `EGRESS`.
        pub direction: pulumi_gestalt_rust::Output<String>,
        /// Denotes whether the firewall rule is disabled, i.e not applied to the
        /// network it is associated with. When set to true, the firewall rule is
        /// not enforced and the network behaves as if it did not exist. If this
        /// is unspecified, the firewall rule will be enabled.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// This field denotes whether to enable logging for a particular firewall rule.
        /// If logging is enabled, logs will be exported to Stackdriver. Deprecated in favor of `log_config`
        pub enable_logging: pulumi_gestalt_rust::Output<bool>,
        /// This field denotes the logging options for a particular firewall rule.
        /// If defined, logging is enabled, and logs will be exported to Cloud Logging.
        /// Structure is documented below.
        pub log_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::FirewallLogConfig>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name or self_link of the network to attach this firewall to.
        ///
        ///
        /// - - -
        pub network: pulumi_gestalt_rust::Output<String>,
        /// Priority for this rule. This is an integer between 0 and 65535, both
        /// inclusive. When not specified, the value assumed is 1000. Relative
        /// priorities determine precedence of conflicting rules. Lower value of
        /// priority implies higher precedence (eg, a rule with priority 0 has
        /// higher precedence than a rule with priority 1). DENY rules take
        /// precedence over ALLOW rules having equal priority.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// If source ranges are specified, the firewall will apply only to
        /// traffic that has source IP address in these ranges. These ranges must
        /// be expressed in CIDR format. One or both of sourceRanges and
        /// sourceTags may be set. If both properties are set, the firewall will
        /// apply to traffic that has source IP address within sourceRanges OR the
        /// source IP that belongs to a tag listed in the sourceTags property. The
        /// connection does not need to match both properties for the firewall to
        /// apply. IPv4 or IPv6 ranges are supported. For INGRESS traffic, one of
        /// `source_ranges`, `source_tags` or `source_service_accounts` is required.
        pub source_ranges: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// If source service accounts are specified, the firewall will apply only
        /// to traffic originating from an instance with a service account in this
        /// list. Source service accounts cannot be used to control traffic to an
        /// instance's external IP address because service accounts are associated
        /// with an instance, not an IP address. sourceRanges can be set at the
        /// same time as sourceServiceAccounts. If both are set, the firewall will
        /// apply to traffic that has source IP address within sourceRanges OR the
        /// source IP belongs to an instance with service account listed in
        /// sourceServiceAccount. The connection does not need to match both
        /// properties for the firewall to apply. sourceServiceAccounts cannot be
        /// used at the same time as sourceTags or targetTags. For INGRESS traffic,
        /// one of `source_ranges`, `source_tags` or `source_service_accounts` is required.
        pub source_service_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// If source tags are specified, the firewall will apply only to traffic
        /// with source IP that belongs to a tag listed in source tags. Source
        /// tags cannot be used to control traffic to an instance's external IP
        /// address. Because tags are associated with an instance, not an IP
        /// address. One or both of sourceRanges and sourceTags may be set. If
        /// both properties are set, the firewall will apply to traffic that has
        /// source IP address within sourceRanges OR the source IP that belongs to
        /// a tag listed in the sourceTags property. The connection does not need
        /// to match both properties for the firewall to apply. For INGRESS traffic,
        /// one of `source_ranges`, `source_tags` or `source_service_accounts` is required.
        pub source_tags: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of service accounts indicating sets of instances located in the
        /// network that may make network connections as specified in allowed[].
        /// targetServiceAccounts cannot be used at the same time as targetTags or
        /// sourceTags. If neither targetServiceAccounts nor targetTags are
        /// specified, the firewall rule applies to all instances on the specified
        /// network.
        pub target_service_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of instance tags indicating sets of instances located in the
        /// network that may make network connections as specified in allowed[].
        /// If no targetTags are specified, the firewall rule applies to all
        /// instances on the specified network.
        pub target_tags: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallArgs,
    ) -> FirewallResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allows_binding = args.allows.get_output(context);
        let denies_binding = args.denies.get_output(context);
        let description_binding = args.description.get_output(context);
        let destination_ranges_binding = args.destination_ranges.get_output(context);
        let direction_binding = args.direction.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let enable_logging_binding = args.enable_logging.get_output(context);
        let log_config_binding = args.log_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let project_binding = args.project.get_output(context);
        let source_ranges_binding = args.source_ranges.get_output(context);
        let source_service_accounts_binding = args
            .source_service_accounts
            .get_output(context);
        let source_tags_binding = args.source_tags.get_output(context);
        let target_service_accounts_binding = args
            .target_service_accounts
            .get_output(context);
        let target_tags_binding = args.target_tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/firewall:Firewall".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allows".into(),
                    value: allows_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "denies".into(),
                    value: denies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationRanges".into(),
                    value: destination_ranges_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "direction".into(),
                    value: direction_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableLogging".into(),
                    value: enable_logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logConfig".into(),
                    value: log_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceRanges".into(),
                    value: source_ranges_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceServiceAccounts".into(),
                    value: source_service_accounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceTags".into(),
                    value: source_tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetServiceAccounts".into(),
                    value: target_service_accounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetTags".into(),
                    value: target_tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallResult {
            allows: o.get_field("allows"),
            creation_timestamp: o.get_field("creationTimestamp"),
            denies: o.get_field("denies"),
            description: o.get_field("description"),
            destination_ranges: o.get_field("destinationRanges"),
            direction: o.get_field("direction"),
            disabled: o.get_field("disabled"),
            enable_logging: o.get_field("enableLogging"),
            log_config: o.get_field("logConfig"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            priority: o.get_field("priority"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            source_ranges: o.get_field("sourceRanges"),
            source_service_accounts: o.get_field("sourceServiceAccounts"),
            source_tags: o.get_field("sourceTags"),
            target_service_accounts: o.get_field("targetServiceAccounts"),
            target_tags: o.get_field("targetTags"),
        }
    }
}
