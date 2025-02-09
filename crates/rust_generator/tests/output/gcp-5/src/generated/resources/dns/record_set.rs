/// ## Example Usage
///
/// ### Binding a DNS name to the ephemeral IP of a new instance:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let frontend = record_set::create(
///         "frontend",
///         RecordSetArgs::builder()
///             .managed_zone("${prod.name}")
///             .name("frontend.${prod.dnsName}")
///             .rrdatas(
///                 vec!["${frontendInstance.networkInterfaces[0].accessConfigs[0].natIp}",],
///             )
///             .ttl(300)
///             .type_("A")
///             .build_struct(),
///     );
///     let frontendInstance = instance::create(
///         "frontendInstance",
///         InstanceArgs::builder()
///             .boot_disk(
///                 InstanceBootDisk::builder()
///                     .initializeParams(
///                         InstanceBootDiskInitializeParams::builder()
///                             .image("debian-cloud/debian-11")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .machine_type("g1-small")
///             .name("frontend")
///             .network_interfaces(
///                 vec![
///                     InstanceNetworkInterface::builder()
///                     .accessConfigs(vec![InstanceNetworkInterfaceAccessConfig::builder()
///                     .build_struct(),]).network("default").build_struct(),
///                 ],
///             )
///             .zone("us-central1-b")
///             .build_struct(),
///     );
///     let prod = managed_zone::create(
///         "prod",
///         ManagedZoneArgs::builder()
///             .dns_name("prod.mydomain.com.")
///             .name("prod-zone")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Adding an A record
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let a = record_set::create(
///         "a",
///         RecordSetArgs::builder()
///             .managed_zone("${prod.name}")
///             .name("backend.${prod.dnsName}")
///             .rrdatas(vec!["8.8.8.8",])
///             .ttl(300)
///             .type_("A")
///             .build_struct(),
///     );
///     let prod = managed_zone::create(
///         "prod",
///         ManagedZoneArgs::builder()
///             .dns_name("prod.mydomain.com.")
///             .name("prod-zone")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Adding an MX record
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let mx = record_set::create(
///         "mx",
///         RecordSetArgs::builder()
///             .managed_zone("${prod.name}")
///             .name("${prod.dnsName}")
///             .rrdatas(
///                 vec![
///                     "1 aspmx.l.google.com.", "5 alt1.aspmx.l.google.com.",
///                     "5 alt2.aspmx.l.google.com.", "10 alt3.aspmx.l.google.com.",
///                     "10 alt4.aspmx.l.google.com.",
///                 ],
///             )
///             .ttl(3600)
///             .type_("MX")
///             .build_struct(),
///     );
///     let prod = managed_zone::create(
///         "prod",
///         ManagedZoneArgs::builder()
///             .dns_name("prod.mydomain.com.")
///             .name("prod-zone")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Adding an SPF record
///
/// Quotes (`""`) must be added around your `rrdatas` for a SPF record. Otherwise `rrdatas` string gets split on spaces.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let prod = managed_zone::create(
///         "prod",
///         ManagedZoneArgs::builder()
///             .dns_name("prod.mydomain.com.")
///             .name("prod-zone")
///             .build_struct(),
///     );
///     let spf = record_set::create(
///         "spf",
///         RecordSetArgs::builder()
///             .managed_zone("${prod.name}")
///             .name("frontend.${prod.dnsName}")
///             .rrdatas(
///                 vec![
///                     "\"v=spf1 ip4:111.111.111.111 include:backoff.email-example.com -all\"",
///                 ],
///             )
///             .ttl(300)
///             .type_("TXT")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Adding a CNAME record
///
///  The list of `rrdatas` should only contain a single string corresponding to the Canonical Name intended.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cname = record_set::create(
///         "cname",
///         RecordSetArgs::builder()
///             .managed_zone("${prod.name}")
///             .name("frontend.${prod.dnsName}")
///             .rrdatas(vec!["frontend.mydomain.com.",])
///             .ttl(300)
///             .type_("CNAME")
///             .build_struct(),
///     );
///     let prod = managed_zone::create(
///         "prod",
///         ManagedZoneArgs::builder()
///             .dns_name("prod.mydomain.com.")
///             .name("prod-zone")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Setting Routing Policy instead of using rrdatas
/// ### Geolocation
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let geo = record_set::create(
///         "geo",
///         RecordSetArgs::builder()
///             .managed_zone("${prod.name}")
///             .name("backend.${prod.dnsName}")
///             .routing_policy(
///                 RecordSetRoutingPolicy::builder()
///                     .geos(
///                         vec![
///                             RecordSetRoutingPolicyGeo::builder().location("asia-east1")
///                             .rrdatas(vec!["10.128.1.1",]).build_struct(),
///                             RecordSetRoutingPolicyGeo::builder().location("us-central1")
///                             .rrdatas(vec!["10.130.1.1",]).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .ttl(300)
///             .type_("A")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Failover
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let a = record_set::create(
///         "a",
///         RecordSetArgs::builder()
///             .managed_zone("${prod.name}")
///             .name("backend.${prod.dnsName}")
///             .routing_policy(
///                 RecordSetRoutingPolicy::builder()
///                     .primaryBackup(
///                         RecordSetRoutingPolicyPrimaryBackup::builder()
///                             .backupGeos(
///                                 vec![
///                                     RecordSetRoutingPolicyPrimaryBackupBackupGeo::builder()
///                                     .location("asia-east1").rrdatas(vec!["10.128.1.1",])
///                                     .build_struct(),
///                                     RecordSetRoutingPolicyPrimaryBackupBackupGeo::builder()
///                                     .location("us-west1").rrdatas(vec!["10.130.1.1",])
///                                     .build_struct(),
///                                 ],
///                             )
///                             .primary(
///                                 RecordSetRoutingPolicyPrimaryBackupPrimary::builder()
///                                     .internalLoadBalancers(
///                                         vec![
///                                             RecordSetRoutingPolicyPrimaryBackupPrimaryInternalLoadBalancer::builder()
///                                             .ipAddress("${prodForwardingRule.ipAddress}")
///                                             .ipProtocol("tcp").loadBalancerType("regionalL4ilb")
///                                             .networkUrl("${prodNetwork.id}").port("80")
///                                             .project("${prodForwardingRule.project}")
///                                             .region("${prodForwardingRule.region}").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .trickleRatio(0.1)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .ttl(300)
///             .type_("A")
///             .build_struct(),
///     );
///     let prod = managed_zone::create(
///         "prod",
///         ManagedZoneArgs::builder()
///             .dns_name("prod.mydomain.com.")
///             .name("prod-zone")
///             .build_struct(),
///     );
///     let prodForwardingRule = forwarding_rule::create(
///         "prodForwardingRule",
///         ForwardingRuleArgs::builder()
///             .all_ports(true)
///             .allow_global_access(true)
///             .backend_service("${prodRegionBackendService.id}")
///             .load_balancing_scheme("INTERNAL")
///             .name("prod-ilb")
///             .network("${prodNetwork.name}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let prodNetwork = network::create(
///         "prodNetwork",
///         NetworkArgs::builder().name("prod-network").build_struct(),
///     );
///     let prodRegionBackendService = region_backend_service::create(
///         "prodRegionBackendService",
///         RegionBackendServiceArgs::builder()
///             .name("prod-backend")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DNS record sets can be imported using either of these accepted formats:
///
/// * `projects/{{project}}/managedZones/{{zone}}/rrsets/{{name}}/{{type}}`
///
/// * `{{project}}/{{zone}}/{{name}}/{{type}}`
///
/// * `{{zone}}/{{name}}/{{type}}`
///
/// When using the `pulumi import` command, DNS record sets can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dns/recordSet:RecordSet default projects/{{project}}/managedZones/{{zone}}/rrsets/{{name}}/{{type}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/recordSet:RecordSet default {{project}}/{{zone}}/{{name}}/{{type}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/recordSet:RecordSet default {{zone}}/{{name}}/{{type}}
/// ```
///
/// Note: The record name must include the trailing dot at the end.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod record_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecordSetArgs {
        /// The name of the zone in which this record set will
        /// reside.
        #[builder(into)]
        pub managed_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The DNS name this record set will apply to.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The configuration for steering traffic based on query.
        /// Now you can specify either Weighted Round Robin(WRR) type or Geolocation(GEO) type.
        /// Structure is documented below.
        #[builder(into, default)]
        pub routing_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dns::RecordSetRoutingPolicy>,
        >,
        #[builder(into, default)]
        pub rrdatas: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The time-to-live of this record set (seconds).
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The DNS record set type.
        ///
        /// - - -
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RecordSetResult {
        /// The name of the zone in which this record set will
        /// reside.
        pub managed_zone: pulumi_gestalt_rust::Output<String>,
        /// The DNS name this record set will apply to.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The configuration for steering traffic based on query.
        /// Now you can specify either Weighted Round Robin(WRR) type or Geolocation(GEO) type.
        /// Structure is documented below.
        pub routing_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::dns::RecordSetRoutingPolicy>,
        >,
        pub rrdatas: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The time-to-live of this record set (seconds).
        pub ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The DNS record set type.
        ///
        /// - - -
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecordSetArgs,
    ) -> RecordSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let managed_zone_binding = args.managed_zone.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let routing_policy_binding = args.routing_policy.get_output(context);
        let rrdatas_binding = args.rrdatas.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dns/recordSet:RecordSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedZone".into(),
                    value: managed_zone_binding.get_id(),
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
                    name: "routingPolicy".into(),
                    value: routing_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rrdatas".into(),
                    value: rrdatas_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: ttl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RecordSetResult {
            managed_zone: o.get_field("managedZone"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            routing_policy: o.get_field("routingPolicy"),
            rrdatas: o.get_field("rrdatas"),
            ttl: o.get_field("ttl"),
            type_: o.get_field("type"),
        }
    }
}
