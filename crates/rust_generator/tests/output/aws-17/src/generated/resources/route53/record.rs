/// Provides a Route53 record resource.
///
/// ## Example Usage
///
/// ### Simple routing policy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let www = record::create(
///         "www",
///         RecordArgs::builder()
///             .name("www.example.com")
///             .records(vec!["${lb.publicIp}",])
///             .ttl(300)
///             .type_("A")
///             .zone_id("${primary.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Weighted routing policy
///
/// Other routing policies are configured similarly. See [Amazon Route 53 Developer Guide](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html) for details.
///
/// ```yaml
/// resources:
///   www-dev:
///     type: aws:route53:Record
///     properties:
///       zoneId: ${primary.zoneId}
///       name: www
///       type: CNAME
///       ttl: 5
///       weightedRoutingPolicies:
///         - weight: 10
///       setIdentifier: dev
///       records:
///         - dev.example.com
///   www-live:
///     type: aws:route53:Record
///     properties:
///       zoneId: ${primary.zoneId}
///       name: www
///       type: CNAME
///       ttl: 5
///       weightedRoutingPolicies:
///         - weight: 90
///       setIdentifier: live
///       records:
///         - live.example.com
/// ```
///
/// ### Geoproximity routing policy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let www = record::create(
///         "www",
///         RecordArgs::builder()
///             .geoproximity_routing_policy(
///                 RecordGeoproximityRoutingPolicy::builder()
///                     .coordinates(
///                         vec![
///                             RecordGeoproximityRoutingPolicyCoordinate::builder()
///                             .latitude("49.22").longitude("-74.01").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("www.example.com")
///             .records(vec!["dev.example.com",])
///             .set_identifier("dev")
///             .ttl(300)
///             .type_("CNAME")
///             .zone_id("${primary.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Alias record
///
/// See [related part of Amazon Route 53 Developer Guide](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/resource-record-sets-choosing-alias-non-alias.html)
/// to understand differences between alias and non-alias records.
///
/// TTL for all alias records is [60 seconds](https://aws.amazon.com/route53/faqs/#dns_failover_do_i_need_to_adjust),
/// you cannot change this, therefore `ttl` has to be omitted in alias records.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = load_balancer::create(
///         "main",
///         LoadBalancerArgs::builder()
///             .availability_zones(vec!["us-east-1c",])
///             .listeners(
///                 vec![
///                     LoadBalancerListener::builder().instancePort(80)
///                     .instanceProtocol("http").lbPort(80).lbProtocol("http")
///                     .build_struct(),
///                 ],
///             )
///             .name("foobar-elb")
///             .build_struct(),
///     );
///     let www = record::create(
///         "www",
///         RecordArgs::builder()
///             .aliases(
///                 vec![
///                     RecordAlias::builder().evaluateTargetHealth(true)
///                     .name("${main.dnsName}").zoneId("${main.zoneId}").build_struct(),
///                 ],
///             )
///             .name("example.com")
///             .type_("A")
///             .zone_id("${primary.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### NS and SOA Record Management
///
/// When creating Route 53 zones, the `NS` and `SOA` records for the zone are automatically created. Enabling the `allow_overwrite` argument will allow managing these records in a single deployment without the requirement for `import`.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zone::create(
///         "example",
///         ZoneArgs::builder().name("test.example.com").build_struct(),
///     );
///     let exampleRecord = record::create(
///         "exampleRecord",
///         RecordArgs::builder()
///             .allow_overwrite(true)
///             .name("test.example.com")
///             .records(
///                 vec![
///                     "${example.nameServers[0]}", "${example.nameServers[1]}",
///                     "${example.nameServers[2]}", "${example.nameServers[3]}",
///                 ],
///             )
///             .ttl(172800)
///             .type_("NS")
///             .zone_id("${example.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the record also contains a set identifier, append it:
///
/// If the record name is the empty string, it can be omitted:
///
/// __Using `pulumi import` to import__ Route53 Records using the ID of the record, record name, record type, and set identifier. For example:
///
/// Using the ID of the record, which is the zone identifier, record name, and record type, separated by underscores (`_`):
///
/// ```sh
/// $ pulumi import aws:route53/record:Record myrecord Z4KAPRWWNC7JR_dev.example.com_NS
/// ```
/// If the record also contains a set identifier, append it:
///
/// ```sh
/// $ pulumi import aws:route53/record:Record myrecord Z4KAPRWWNC7JR_dev.example.com_NS_dev
/// ```
pub mod record {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecordArgs {
        /// An alias block. Conflicts with `ttl` & `records`.
        /// Documented below.
        #[builder(into, default)]
        pub aliases: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::route53::RecordAlias>>,
        >,
        /// Allow creation of this record to overwrite an existing record, if any. This does not affect the ability to update the record using this provider and does not prevent other resources within this provider or manual Route 53 changes outside this provider from overwriting this record. `false` by default. This configuration is not recommended for most environments.
        ///
        /// Exactly one of `records` or `alias` must be specified: this determines whether it's an alias record.
        #[builder(into, default)]
        pub allow_overwrite: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A block indicating a routing policy based on the IP network ranges of requestors. Conflicts with any other routing policy. Documented below.
        #[builder(into, default)]
        pub cidr_routing_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::route53::RecordCidrRoutingPolicy>,
        >,
        /// A block indicating the routing behavior when associated health check fails. Conflicts with any other routing policy. Documented below.
        #[builder(into, default)]
        pub failover_routing_policies: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::route53::RecordFailoverRoutingPolicy>>,
        >,
        /// A block indicating a routing policy based on the geolocation of the requestor. Conflicts with any other routing policy. Documented below.
        #[builder(into, default)]
        pub geolocation_routing_policies: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::route53::RecordGeolocationRoutingPolicy>>,
        >,
        /// A block indicating a routing policy based on the geoproximity of the requestor. Conflicts with any other routing policy. Documented below.
        #[builder(into, default)]
        pub geoproximity_routing_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::route53::RecordGeoproximityRoutingPolicy>,
        >,
        /// The health check the record should be associated with.
        #[builder(into, default)]
        pub health_check_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A block indicating a routing policy based on the latency between the requestor and an AWS region. Conflicts with any other routing policy. Documented below.
        #[builder(into, default)]
        pub latency_routing_policies: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::route53::RecordLatencyRoutingPolicy>>,
        >,
        /// Set to `true` to indicate a multivalue answer routing policy. Conflicts with any other routing policy.
        #[builder(into, default)]
        pub multivalue_answer_routing_policy: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the record.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A string list of records. To specify a single record value longer than 255 characters such as a TXT record for DKIM, add `\"\"` inside the provider configuration string (e.g., `"first255characters\"\"morecharacters"`).
        #[builder(into, default)]
        pub records: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Unique identifier to differentiate records with routing policies from one another. Required if using `cidr_routing_policy`, `failover_routing_policy`, `geolocation_routing_policy`,`geoproximity_routing_policy`, `latency_routing_policy`, `multivalue_answer_routing_policy`, or `weighted_routing_policy`.
        #[builder(into, default)]
        pub set_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The TTL of the record.
        #[builder(into, default)]
        pub ttl: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The record type. Valid values are `A`, `AAAA`, `CAA`, `CNAME`, `DS`, `MX`, `NAPTR`, `NS`, `PTR`, `SOA`, `SPF`, `SRV` and `TXT`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
        /// A block indicating a weighted routing policy. Conflicts with any other routing policy. Documented below.
        #[builder(into, default)]
        pub weighted_routing_policies: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::route53::RecordWeightedRoutingPolicy>>,
        >,
        /// The ID of the hosted zone to contain this record.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RecordResult {
        /// An alias block. Conflicts with `ttl` & `records`.
        /// Documented below.
        pub aliases: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::route53::RecordAlias>>,
        >,
        /// Allow creation of this record to overwrite an existing record, if any. This does not affect the ability to update the record using this provider and does not prevent other resources within this provider or manual Route 53 changes outside this provider from overwriting this record. `false` by default. This configuration is not recommended for most environments.
        ///
        /// Exactly one of `records` or `alias` must be specified: this determines whether it's an alias record.
        pub allow_overwrite: pulumi_wasm_rust::Output<bool>,
        /// A block indicating a routing policy based on the IP network ranges of requestors. Conflicts with any other routing policy. Documented below.
        pub cidr_routing_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::route53::RecordCidrRoutingPolicy>,
        >,
        /// A block indicating the routing behavior when associated health check fails. Conflicts with any other routing policy. Documented below.
        pub failover_routing_policies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::route53::RecordFailoverRoutingPolicy>>,
        >,
        /// [FQDN](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) built using the zone domain and `name`.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// A block indicating a routing policy based on the geolocation of the requestor. Conflicts with any other routing policy. Documented below.
        pub geolocation_routing_policies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::route53::RecordGeolocationRoutingPolicy>>,
        >,
        /// A block indicating a routing policy based on the geoproximity of the requestor. Conflicts with any other routing policy. Documented below.
        pub geoproximity_routing_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::route53::RecordGeoproximityRoutingPolicy>,
        >,
        /// The health check the record should be associated with.
        pub health_check_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A block indicating a routing policy based on the latency between the requestor and an AWS region. Conflicts with any other routing policy. Documented below.
        pub latency_routing_policies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::route53::RecordLatencyRoutingPolicy>>,
        >,
        /// Set to `true` to indicate a multivalue answer routing policy. Conflicts with any other routing policy.
        pub multivalue_answer_routing_policy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the record.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A string list of records. To specify a single record value longer than 255 characters such as a TXT record for DKIM, add `\"\"` inside the provider configuration string (e.g., `"first255characters\"\"morecharacters"`).
        pub records: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Unique identifier to differentiate records with routing policies from one another. Required if using `cidr_routing_policy`, `failover_routing_policy`, `geolocation_routing_policy`,`geoproximity_routing_policy`, `latency_routing_policy`, `multivalue_answer_routing_policy`, or `weighted_routing_policy`.
        pub set_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The TTL of the record.
        pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
        /// The record type. Valid values are `A`, `AAAA`, `CAA`, `CNAME`, `DS`, `MX`, `NAPTR`, `NS`, `PTR`, `SOA`, `SPF`, `SRV` and `TXT`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// A block indicating a weighted routing policy. Conflicts with any other routing policy. Documented below.
        pub weighted_routing_policies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::route53::RecordWeightedRoutingPolicy>>,
        >,
        /// The ID of the hosted zone to contain this record.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RecordArgs,
    ) -> RecordResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aliases_binding = args.aliases.get_output(context).get_inner();
        let allow_overwrite_binding = args
            .allow_overwrite
            .get_output(context)
            .get_inner();
        let cidr_routing_policy_binding = args
            .cidr_routing_policy
            .get_output(context)
            .get_inner();
        let failover_routing_policies_binding = args
            .failover_routing_policies
            .get_output(context)
            .get_inner();
        let geolocation_routing_policies_binding = args
            .geolocation_routing_policies
            .get_output(context)
            .get_inner();
        let geoproximity_routing_policy_binding = args
            .geoproximity_routing_policy
            .get_output(context)
            .get_inner();
        let health_check_id_binding = args
            .health_check_id
            .get_output(context)
            .get_inner();
        let latency_routing_policies_binding = args
            .latency_routing_policies
            .get_output(context)
            .get_inner();
        let multivalue_answer_routing_policy_binding = args
            .multivalue_answer_routing_policy
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let records_binding = args.records.get_output(context).get_inner();
        let set_identifier_binding = args.set_identifier.get_output(context).get_inner();
        let ttl_binding = args.ttl.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let weighted_routing_policies_binding = args
            .weighted_routing_policies
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/record:Record".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aliases".into(),
                    value: &aliases_binding,
                },
                register_interface::ObjectField {
                    name: "allowOverwrite".into(),
                    value: &allow_overwrite_binding,
                },
                register_interface::ObjectField {
                    name: "cidrRoutingPolicy".into(),
                    value: &cidr_routing_policy_binding,
                },
                register_interface::ObjectField {
                    name: "failoverRoutingPolicies".into(),
                    value: &failover_routing_policies_binding,
                },
                register_interface::ObjectField {
                    name: "geolocationRoutingPolicies".into(),
                    value: &geolocation_routing_policies_binding,
                },
                register_interface::ObjectField {
                    name: "geoproximityRoutingPolicy".into(),
                    value: &geoproximity_routing_policy_binding,
                },
                register_interface::ObjectField {
                    name: "healthCheckId".into(),
                    value: &health_check_id_binding,
                },
                register_interface::ObjectField {
                    name: "latencyRoutingPolicies".into(),
                    value: &latency_routing_policies_binding,
                },
                register_interface::ObjectField {
                    name: "multivalueAnswerRoutingPolicy".into(),
                    value: &multivalue_answer_routing_policy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "records".into(),
                    value: &records_binding,
                },
                register_interface::ObjectField {
                    name: "setIdentifier".into(),
                    value: &set_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "weightedRoutingPolicies".into(),
                    value: &weighted_routing_policies_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RecordResult {
            aliases: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("aliases"),
            ),
            allow_overwrite: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowOverwrite"),
            ),
            cidr_routing_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrRoutingPolicy"),
            ),
            failover_routing_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("failoverRoutingPolicies"),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(o.extract_field("fqdn")),
            geolocation_routing_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("geolocationRoutingPolicies"),
            ),
            geoproximity_routing_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("geoproximityRoutingPolicy"),
            ),
            health_check_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthCheckId"),
            ),
            latency_routing_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latencyRoutingPolicies"),
            ),
            multivalue_answer_routing_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multivalueAnswerRoutingPolicy"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            records: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("records"),
            ),
            set_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("setIdentifier"),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(o.extract_field("ttl")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            weighted_routing_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("weightedRoutingPolicies"),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
