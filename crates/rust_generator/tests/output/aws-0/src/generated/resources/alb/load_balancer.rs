/// Provides a Load Balancer resource.
///
/// > **Note:** `aws.alb.LoadBalancer` is known as `aws.lb.LoadBalancer`. The functionality is identical.
///
/// ## Example Usage
///
/// ### Application Load Balancer
///
///
/// ### Network Load Balancer
///
///
/// ### Specifying Elastic IPs
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = load_balancer::create(
///         "example",
///         LoadBalancerArgs::builder()
///             .load_balancer_type("network")
///             .name("example")
///             .subnet_mappings(
///                 vec![
///                     LoadBalancerSubnetMapping::builder().allocationId("${example1.id}")
///                     .subnetId("${example1AwsSubnet.id}").build_struct(),
///                     LoadBalancerSubnetMapping::builder().allocationId("${example2.id}")
///                     .subnetId("${example2AwsSubnet.id}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Specifying private IP addresses for an internal-facing load balancer
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = load_balancer::create(
///         "example",
///         LoadBalancerArgs::builder()
///             .load_balancer_type("network")
///             .name("example")
///             .subnet_mappings(
///                 vec![
///                     LoadBalancerSubnetMapping::builder().privateIpv4Address("10.0.1.15")
///                     .subnetId("${example1.id}").build_struct(),
///                     LoadBalancerSubnetMapping::builder().privateIpv4Address("10.0.2.15")
///                     .subnetId("${example2.id}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import LBs using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:alb/loadBalancer:LoadBalancer bar arn:aws:elasticloadbalancing:us-west-2:123456789012:loadbalancer/app/my-load-balancer/50dc6c495c0c9188
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod load_balancer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerArgs {
        /// Access Logs block. See below.
        #[builder(into, default)]
        pub access_logs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alb::LoadBalancerAccessLogs>,
        >,
        /// Client keep alive value in seconds. The valid range is 60-604800 seconds. The default is 3600 seconds.
        #[builder(into, default)]
        pub client_keep_alive: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Connection Logs block. See below. Only valid for Load Balancers of type `application`.
        #[builder(into, default)]
        pub connection_logs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alb::LoadBalancerConnectionLogs>,
        >,
        /// ID of the customer owned ipv4 pool to use for this load balancer.
        #[builder(into, default)]
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// How the load balancer handles requests that might pose a security risk to an application due to HTTP desync. Valid values are `monitor`, `defensive` (default), `strictest`.
        #[builder(into, default)]
        pub desync_mitigation_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// How traffic is distributed among the load balancer Availability Zones. Possible values are `any_availability_zone` (default), `availability_zone_affinity`, or `partial_availability_zone_affinity`. See   [Availability Zone DNS affinity](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html#zonal-dns-affinity) for additional details. Only valid for `network` type load balancers.
        #[builder(into, default)]
        pub dns_record_client_routing_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether HTTP headers with header fields that are not valid are removed by the load balancer (true) or routed to targets (false). The default is false. Elastic Load Balancing requires that message header names contain only alphanumeric characters and hyphens. Only valid for Load Balancers of type `application`.
        #[builder(into, default)]
        pub drop_invalid_header_fields: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true, cross-zone load balancing of the load balancer will be enabled. For `network` and `gateway` type load balancers, this feature is disabled by default (`false`). For `application` load balancer this feature is always enabled (`true`) and cannot be disabled. Defaults to `false`.
        #[builder(into, default)]
        pub enable_cross_zone_load_balancing: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// If true, deletion of the load balancer will be disabled via the AWS API. This will prevent this provider from deleting the load balancer. Defaults to `false`.
        #[builder(into, default)]
        pub enable_deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether HTTP/2 is enabled in `application` load balancers. Defaults to `true`.
        #[builder(into, default)]
        pub enable_http2: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the two headers (`x-amzn-tls-version` and `x-amzn-tls-cipher-suite`), which contain information about the negotiated TLS version and cipher suite, are added to the client request before sending it to the target. Only valid for Load Balancers of type `application`. Defaults to `false`
        #[builder(into, default)]
        pub enable_tls_version_and_cipher_suite_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to allow a WAF-enabled load balancer to route requests to targets if it is unable to forward the request to AWS WAF. Defaults to `false`.
        #[builder(into, default)]
        pub enable_waf_fail_open: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the X-Forwarded-For header should preserve the source port that the client used to connect to the load balancer in `application` load balancers. Defaults to `false`.
        #[builder(into, default)]
        pub enable_xff_client_port: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether zonal shift is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub enable_zonal_shift: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether inbound security group rules are enforced for traffic originating from a PrivateLink. Only valid for Load Balancers of type `network`. The possible values are `on` and `off`.
        #[builder(into, default)]
        pub enforce_security_group_inbound_rules_on_private_link_traffic: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Time in seconds that the connection is allowed to be idle. Only valid for Load Balancers of type `application`. Default: 60.
        #[builder(into, default)]
        pub idle_timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// If true, the LB will be internal. Defaults to `false`.
        #[builder(into, default)]
        pub internal: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Type of IP addresses used by the subnets for your load balancer. The possible values depend upon the load balancer type: `ipv4` (all load balancer types), `dualstack` (all load balancer types), and `dualstack-without-public-ipv4` (type `application` only).
        #[builder(into, default)]
        pub ip_address_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of load balancer to create. Possible values are `application`, `gateway`, or `network`. The default value is `application`.
        #[builder(into, default)]
        pub load_balancer_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the LB. This name must be unique within your AWS account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen. If not specified, this provider will autogenerate a name beginning with `tf-lb`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the Application Load Balancer should preserve the Host header in the HTTP request and send it to the target without any change. Defaults to `false`.
        #[builder(into, default)]
        pub preserve_host_header: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of security group IDs to assign to the LB. Only valid for Load Balancers of type `application` or `network`. For load balancers of type `network` security groups cannot be added if none are currently present, and cannot all be removed once added. If either of these conditions are met, this will force a recreation of the resource.
        #[builder(into, default)]
        pub security_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Subnet mapping block. See below. For Load Balancers of type `network` subnet mappings can only be added.
        #[builder(into, default)]
        pub subnet_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::alb::LoadBalancerSubnetMapping>>,
        >,
        /// List of subnet IDs to attach to the LB. For Load Balancers of type `network` subnets can only be added (see [Availability Zones](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html#availability-zones)), deleting a subnet for load balancers of type `network` will force a recreation of the resource.
        #[builder(into, default)]
        pub subnets: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Determines how the load balancer modifies the `X-Forwarded-For` header in the HTTP request before sending the request to the target. The possible values are `append`, `preserve`, and `remove`. Only valid for Load Balancers of type `application`. The default is `append`.
        #[builder(into, default)]
        pub xff_header_processing_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerResult {
        /// Access Logs block. See below.
        pub access_logs: pulumi_gestalt_rust::Output<
            Option<super::super::types::alb::LoadBalancerAccessLogs>,
        >,
        /// ARN of the load balancer (matches `id`).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN suffix for use with CloudWatch Metrics.
        pub arn_suffix: pulumi_gestalt_rust::Output<String>,
        /// Client keep alive value in seconds. The valid range is 60-604800 seconds. The default is 3600 seconds.
        pub client_keep_alive: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Connection Logs block. See below. Only valid for Load Balancers of type `application`.
        pub connection_logs: pulumi_gestalt_rust::Output<
            Option<super::super::types::alb::LoadBalancerConnectionLogs>,
        >,
        /// ID of the customer owned ipv4 pool to use for this load balancer.
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::Output<Option<String>>,
        /// How the load balancer handles requests that might pose a security risk to an application due to HTTP desync. Valid values are `monitor`, `defensive` (default), `strictest`.
        pub desync_mitigation_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// DNS name of the load balancer.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// How traffic is distributed among the load balancer Availability Zones. Possible values are `any_availability_zone` (default), `availability_zone_affinity`, or `partial_availability_zone_affinity`. See   [Availability Zone DNS affinity](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html#zonal-dns-affinity) for additional details. Only valid for `network` type load balancers.
        pub dns_record_client_routing_policy: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Whether HTTP headers with header fields that are not valid are removed by the load balancer (true) or routed to targets (false). The default is false. Elastic Load Balancing requires that message header names contain only alphanumeric characters and hyphens. Only valid for Load Balancers of type `application`.
        pub drop_invalid_header_fields: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If true, cross-zone load balancing of the load balancer will be enabled. For `network` and `gateway` type load balancers, this feature is disabled by default (`false`). For `application` load balancer this feature is always enabled (`true`) and cannot be disabled. Defaults to `false`.
        pub enable_cross_zone_load_balancing: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If true, deletion of the load balancer will be disabled via the AWS API. This will prevent this provider from deleting the load balancer. Defaults to `false`.
        pub enable_deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether HTTP/2 is enabled in `application` load balancers. Defaults to `true`.
        pub enable_http2: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the two headers (`x-amzn-tls-version` and `x-amzn-tls-cipher-suite`), which contain information about the negotiated TLS version and cipher suite, are added to the client request before sending it to the target. Only valid for Load Balancers of type `application`. Defaults to `false`
        pub enable_tls_version_and_cipher_suite_headers: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Whether to allow a WAF-enabled load balancer to route requests to targets if it is unable to forward the request to AWS WAF. Defaults to `false`.
        pub enable_waf_fail_open: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the X-Forwarded-For header should preserve the source port that the client used to connect to the load balancer in `application` load balancers. Defaults to `false`.
        pub enable_xff_client_port: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether zonal shift is enabled. Defaults to `false`.
        pub enable_zonal_shift: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether inbound security group rules are enforced for traffic originating from a PrivateLink. Only valid for Load Balancers of type `network`. The possible values are `on` and `off`.
        pub enforce_security_group_inbound_rules_on_private_link_traffic: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Time in seconds that the connection is allowed to be idle. Only valid for Load Balancers of type `application`. Default: 60.
        pub idle_timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// If true, the LB will be internal. Defaults to `false`.
        pub internal: pulumi_gestalt_rust::Output<bool>,
        /// Type of IP addresses used by the subnets for your load balancer. The possible values depend upon the load balancer type: `ipv4` (all load balancer types), `dualstack` (all load balancer types), and `dualstack-without-public-ipv4` (type `application` only).
        pub ip_address_type: pulumi_gestalt_rust::Output<String>,
        /// Type of load balancer to create. Possible values are `application`, `gateway`, or `network`. The default value is `application`.
        pub load_balancer_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the LB. This name must be unique within your AWS account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen. If not specified, this provider will autogenerate a name beginning with `tf-lb`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Whether the Application Load Balancer should preserve the Host header in the HTTP request and send it to the target without any change. Defaults to `false`.
        pub preserve_host_header: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of security group IDs to assign to the LB. Only valid for Load Balancers of type `application` or `network`. For load balancers of type `network` security groups cannot be added if none are currently present, and cannot all be removed once added. If either of these conditions are met, this will force a recreation of the resource.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Subnet mapping block. See below. For Load Balancers of type `network` subnet mappings can only be added.
        pub subnet_mappings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alb::LoadBalancerSubnetMapping>,
        >,
        /// List of subnet IDs to attach to the LB. For Load Balancers of type `network` subnets can only be added (see [Availability Zones](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html#availability-zones)), deleting a subnet for load balancers of type `network` will force a recreation of the resource.
        pub subnets: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// Determines how the load balancer modifies the `X-Forwarded-For` header in the HTTP request before sending the request to the target. The possible values are `append`, `preserve`, and `remove`. Only valid for Load Balancers of type `application`. The default is `append`.
        pub xff_header_processing_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Canonical hosted zone ID of the load balancer (to be used in a Route 53 Alias record).
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoadBalancerArgs,
    ) -> LoadBalancerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_logs_binding = args.access_logs.get_output(context);
        let client_keep_alive_binding = args.client_keep_alive.get_output(context);
        let connection_logs_binding = args.connection_logs.get_output(context);
        let customer_owned_ipv4_pool_binding = args
            .customer_owned_ipv4_pool
            .get_output(context);
        let desync_mitigation_mode_binding = args
            .desync_mitigation_mode
            .get_output(context);
        let dns_record_client_routing_policy_binding = args
            .dns_record_client_routing_policy
            .get_output(context);
        let drop_invalid_header_fields_binding = args
            .drop_invalid_header_fields
            .get_output(context);
        let enable_cross_zone_load_balancing_binding = args
            .enable_cross_zone_load_balancing
            .get_output(context);
        let enable_deletion_protection_binding = args
            .enable_deletion_protection
            .get_output(context);
        let enable_http2_binding = args.enable_http2.get_output(context);
        let enable_tls_version_and_cipher_suite_headers_binding = args
            .enable_tls_version_and_cipher_suite_headers
            .get_output(context);
        let enable_waf_fail_open_binding = args.enable_waf_fail_open.get_output(context);
        let enable_xff_client_port_binding = args
            .enable_xff_client_port
            .get_output(context);
        let enable_zonal_shift_binding = args.enable_zonal_shift.get_output(context);
        let enforce_security_group_inbound_rules_on_private_link_traffic_binding = args
            .enforce_security_group_inbound_rules_on_private_link_traffic
            .get_output(context);
        let idle_timeout_binding = args.idle_timeout.get_output(context);
        let internal_binding = args.internal.get_output(context);
        let ip_address_type_binding = args.ip_address_type.get_output(context);
        let load_balancer_type_binding = args.load_balancer_type.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let preserve_host_header_binding = args.preserve_host_header.get_output(context);
        let security_groups_binding = args.security_groups.get_output(context);
        let subnet_mappings_binding = args.subnet_mappings.get_output(context);
        let subnets_binding = args.subnets.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let xff_header_processing_mode_binding = args
            .xff_header_processing_mode
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:alb/loadBalancer:LoadBalancer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessLogs".into(),
                    value: &access_logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientKeepAlive".into(),
                    value: &client_keep_alive_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionLogs".into(),
                    value: &connection_logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerOwnedIpv4Pool".into(),
                    value: &customer_owned_ipv4_pool_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "desyncMitigationMode".into(),
                    value: &desync_mitigation_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsRecordClientRoutingPolicy".into(),
                    value: &dns_record_client_routing_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dropInvalidHeaderFields".into(),
                    value: &drop_invalid_header_fields_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableCrossZoneLoadBalancing".into(),
                    value: &enable_cross_zone_load_balancing_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableDeletionProtection".into(),
                    value: &enable_deletion_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableHttp2".into(),
                    value: &enable_http2_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableTlsVersionAndCipherSuiteHeaders".into(),
                    value: &enable_tls_version_and_cipher_suite_headers_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableWafFailOpen".into(),
                    value: &enable_waf_fail_open_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableXffClientPort".into(),
                    value: &enable_xff_client_port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableZonalShift".into(),
                    value: &enable_zonal_shift_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enforceSecurityGroupInboundRulesOnPrivateLinkTraffic".into(),
                    value: &enforce_security_group_inbound_rules_on_private_link_traffic_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idleTimeout".into(),
                    value: &idle_timeout_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internal".into(),
                    value: &internal_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancerType".into(),
                    value: &load_balancer_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preserveHostHeader".into(),
                    value: &preserve_host_header_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetMappings".into(),
                    value: &subnet_mappings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnets".into(),
                    value: &subnets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "xffHeaderProcessingMode".into(),
                    value: &xff_header_processing_mode_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LoadBalancerResult {
            access_logs: o.get_field("accessLogs"),
            arn: o.get_field("arn"),
            arn_suffix: o.get_field("arnSuffix"),
            client_keep_alive: o.get_field("clientKeepAlive"),
            connection_logs: o.get_field("connectionLogs"),
            customer_owned_ipv4_pool: o.get_field("customerOwnedIpv4Pool"),
            desync_mitigation_mode: o.get_field("desyncMitigationMode"),
            dns_name: o.get_field("dnsName"),
            dns_record_client_routing_policy: o
                .get_field("dnsRecordClientRoutingPolicy"),
            drop_invalid_header_fields: o.get_field("dropInvalidHeaderFields"),
            enable_cross_zone_load_balancing: o
                .get_field("enableCrossZoneLoadBalancing"),
            enable_deletion_protection: o.get_field("enableDeletionProtection"),
            enable_http2: o.get_field("enableHttp2"),
            enable_tls_version_and_cipher_suite_headers: o
                .get_field("enableTlsVersionAndCipherSuiteHeaders"),
            enable_waf_fail_open: o.get_field("enableWafFailOpen"),
            enable_xff_client_port: o.get_field("enableXffClientPort"),
            enable_zonal_shift: o.get_field("enableZonalShift"),
            enforce_security_group_inbound_rules_on_private_link_traffic: o
                .get_field("enforceSecurityGroupInboundRulesOnPrivateLinkTraffic"),
            idle_timeout: o.get_field("idleTimeout"),
            internal: o.get_field("internal"),
            ip_address_type: o.get_field("ipAddressType"),
            load_balancer_type: o.get_field("loadBalancerType"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            preserve_host_header: o.get_field("preserveHostHeader"),
            security_groups: o.get_field("securityGroups"),
            subnet_mappings: o.get_field("subnetMappings"),
            subnets: o.get_field("subnets"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
            xff_header_processing_mode: o.get_field("xffHeaderProcessingMode"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
