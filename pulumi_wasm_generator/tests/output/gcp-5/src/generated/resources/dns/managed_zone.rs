/// A zone is a subtree of the DNS namespace under one administrative
/// responsibility. A ManagedZone is a resource that represents a DNS zone
/// hosted by the Cloud DNS service.
///
///
/// To get more information about ManagedZone, see:
///
/// * [API documentation](https://cloud.google.com/dns/api/v1/managedZones)
/// * How-to Guides
///     * [Managing Zones](https://cloud.google.com/dns/zones/)
///
/// ## Example Usage
///
/// ### Dns Managed Zone Basic
///
///
/// ```yaml
/// resources:
///   example-zone:
///     type: gcp:dns:ManagedZone
///     properties:
///       name: example-zone
///       dnsName: my-domain.com.
///       description: Example DNS zone
///       labels:
///         foo: bar
/// ```
/// ### Dns Managed Zone Private
///
///
/// ```yaml
/// resources:
///   private-zone:
///     type: gcp:dns:ManagedZone
///     properties:
///       name: private-zone
///       dnsName: private.example.com.
///       description: Example private DNS zone
///       labels:
///         foo: bar
///       visibility: private
///       privateVisibilityConfig:
///         networks:
///           - networkUrl: ${["network-1"].id}
///           - networkUrl: ${["network-2"].id}
///   network-1:
///     type: gcp:compute:Network
///     properties:
///       name: network-1
///       autoCreateSubnetworks: false
///   network-2:
///     type: gcp:compute:Network
///     properties:
///       name: network-2
///       autoCreateSubnetworks: false
/// ```
/// ### Dns Managed Zone Private Forwarding
///
///
/// ```yaml
/// resources:
///   private-zone:
///     type: gcp:dns:ManagedZone
///     properties:
///       name: private-zone
///       dnsName: private.example.com.
///       description: Example private DNS zone
///       labels:
///         foo: bar
///       visibility: private
///       privateVisibilityConfig:
///         networks:
///           - networkUrl: ${["network-1"].id}
///           - networkUrl: ${["network-2"].id}
///       forwardingConfig:
///         targetNameServers:
///           - ipv4Address: 172.16.1.10
///           - ipv4Address: 172.16.1.20
///   network-1:
///     type: gcp:compute:Network
///     properties:
///       name: network-1
///       autoCreateSubnetworks: false
///   network-2:
///     type: gcp:compute:Network
///     properties:
///       name: network-2
///       autoCreateSubnetworks: false
/// ```
/// ### Dns Managed Zone Private Gke
///
///
/// ```yaml
/// resources:
///   private-zone-gke:
///     type: gcp:dns:ManagedZone
///     properties:
///       name: private-zone
///       dnsName: private.example.com.
///       description: Example private DNS zone
///       labels:
///         foo: bar
///       visibility: private
///       privateVisibilityConfig:
///         gkeClusters:
///           - gkeClusterName: ${["cluster-1"].id}
///   network-1:
///     type: gcp:compute:Network
///     properties:
///       name: network-1
///       autoCreateSubnetworks: false
///   subnetwork-1:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: ${["network-1"].name}
///       network: ${["network-1"].name}
///       ipCidrRange: 10.0.36.0/24
///       region: us-central1
///       privateIpGoogleAccess: true
///       secondaryIpRanges:
///         - rangeName: pod
///           ipCidrRange: 10.0.0.0/19
///         - rangeName: svc
///           ipCidrRange: 10.0.32.0/22
///   cluster-1:
///     type: gcp:container:Cluster
///     properties:
///       name: cluster-1
///       location: us-central1-c
///       initialNodeCount: 1
///       networkingMode: VPC_NATIVE
///       defaultSnatStatus:
///         disabled: true
///       network: ${["network-1"].name}
///       subnetwork: ${["subnetwork-1"].name}
///       privateClusterConfig:
///         enablePrivateEndpoint: true
///         enablePrivateNodes: true
///         masterIpv4CidrBlock: 10.42.0.0/28
///         masterGlobalAccessConfig:
///           enabled: true
///       masterAuthorizedNetworksConfig: {}
///       ipAllocationPolicy:
///         clusterSecondaryRangeName: ${["subnetwork-1"].secondaryIpRanges[0].rangeName}
///         servicesSecondaryRangeName: ${["subnetwork-1"].secondaryIpRanges[1].rangeName}
///       deletionProtection: true
/// ```
/// ### Dns Managed Zone Private Peering
///
///
/// ```yaml
/// resources:
///   peering-zone:
///     type: gcp:dns:ManagedZone
///     properties:
///       name: peering-zone
///       dnsName: peering.example.com.
///       description: Example private DNS peering zone
///       visibility: private
///       privateVisibilityConfig:
///         networks:
///           - networkUrl: ${["network-source"].id}
///       peeringConfig:
///         targetNetwork:
///           networkUrl: ${["network-target"].id}
///   network-source:
///     type: gcp:compute:Network
///     properties:
///       name: network-source
///       autoCreateSubnetworks: false
///   network-target:
///     type: gcp:compute:Network
///     properties:
///       name: network-target
///       autoCreateSubnetworks: false
/// ```
/// ### Dns Managed Zone Service Directory
///
///
/// ```yaml
/// resources:
///   sd-zone:
///     type: gcp:dns:ManagedZone
///     properties:
///       name: peering-zone
///       dnsName: services.example.com.
///       description: Example private DNS Service Directory zone
///       visibility: private
///       serviceDirectoryConfig:
///         namespace:
///           namespaceUrl: ${example.id}
///   example:
///     type: gcp:servicedirectory:Namespace
///     properties:
///       namespaceId: example
///       location: us-central1
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: network
///       autoCreateSubnetworks: false
/// ```
/// ### Dns Managed Zone Cloud Logging
///
///
/// ```yaml
/// resources:
///   cloud-logging-enabled-zone:
///     type: gcp:dns:ManagedZone
///     properties:
///       name: cloud-logging-enabled-zone
///       dnsName: services.example.com.
///       description: Example cloud logging enabled DNS zone
///       labels:
///         foo: bar
///       cloudLoggingConfig:
///         enableLogging: true
/// ```
///
/// ## Import
///
/// ManagedZone can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/managedZones/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ManagedZone can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dns/managedZone:ManagedZone default projects/{{project}}/managedZones/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/managedZone:ManagedZone default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/managedZone:ManagedZone default {{name}}
/// ```
///
pub mod managed_zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedZoneArgs {
        /// Cloud logging configuration
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloud_logging_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dns::ManagedZoneCloudLoggingConfig>,
        >,
        /// A textual description field. Defaults to 'Managed by Pulumi'.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The DNS name of this managed zone, for instance "example.com.".
        #[builder(into)]
        pub dns_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// DNSSEC configuration
        /// Structure is documented below.
        #[builder(into, default)]
        pub dnssec_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dns::ManagedZoneDnssecConfig>,
        >,
        /// Set this true to delete all records in the zone.
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The presence for this field indicates that outbound forwarding is enabled
        /// for this zone. The value of this field contains the set of destinations
        /// to forward to.
        /// Structure is documented below.
        #[builder(into, default)]
        pub forwarding_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dns::ManagedZoneForwardingConfig>,
        >,
        /// A set of key/value label pairs to assign to this ManagedZone.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User assigned name for this resource.
        /// Must be unique within the project.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The presence of this field indicates that DNS Peering is enabled for this
        /// zone. The value of this field contains the network to peer with.
        /// Structure is documented below.
        #[builder(into, default)]
        pub peering_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dns::ManagedZonePeeringConfig>,
        >,
        /// For privately visible zones, the set of Virtual Private Cloud
        /// resources that the zone is visible from. At least one of `gke_clusters` or `networks` must be specified.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_visibility_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dns::ManagedZonePrivateVisibilityConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies if this is a managed reverse lookup zone. If true, Cloud DNS will resolve reverse
        /// lookup queries using automatically configured records for VPC resources. This only applies
        /// to networks listed under `private_visibility_config`.
        #[builder(into, default)]
        pub reverse_lookup: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The presence of this field indicates that this zone is backed by Service Directory. The value of this field contains information related to the namespace associated with the zone.
        /// Structure is documented below.
        #[builder(into, default)]
        pub service_directory_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dns::ManagedZoneServiceDirectoryConfig>,
        >,
        /// The zone's visibility: public zones are exposed to the Internet,
        /// while private zones are visible only to Virtual Private Cloud resources.
        /// Default value is `public`.
        /// Possible values are: `private`, `public`.
        #[builder(into, default)]
        pub visibility: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagedZoneResult {
        /// Cloud logging configuration
        /// Structure is documented below.
        pub cloud_logging_config: pulumi_wasm_rust::Output<
            super::super::types::dns::ManagedZoneCloudLoggingConfig,
        >,
        /// The time that this resource was created on the server.
        /// This is in RFC3339 text format.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// A textual description field. Defaults to 'Managed by Pulumi'.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The DNS name of this managed zone, for instance "example.com.".
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// DNSSEC configuration
        /// Structure is documented below.
        pub dnssec_config: pulumi_wasm_rust::Output<
            super::super::types::dns::ManagedZoneDnssecConfig,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set this true to delete all records in the zone.
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The presence for this field indicates that outbound forwarding is enabled
        /// for this zone. The value of this field contains the set of destinations
        /// to forward to.
        /// Structure is documented below.
        pub forwarding_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dns::ManagedZoneForwardingConfig>,
        >,
        /// A set of key/value label pairs to assign to this ManagedZone.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Unique identifier for the resource; defined by the server.
        pub managed_zone_id: pulumi_wasm_rust::Output<String>,
        /// User assigned name for this resource.
        /// Must be unique within the project.
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// Delegate your managed_zone to these virtual name servers;
        /// defined by the server
        pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The presence of this field indicates that DNS Peering is enabled for this
        /// zone. The value of this field contains the network to peer with.
        /// Structure is documented below.
        pub peering_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dns::ManagedZonePeeringConfig>,
        >,
        /// For privately visible zones, the set of Virtual Private Cloud
        /// resources that the zone is visible from. At least one of `gke_clusters` or `networks` must be specified.
        /// Structure is documented below.
        pub private_visibility_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dns::ManagedZonePrivateVisibilityConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies if this is a managed reverse lookup zone. If true, Cloud DNS will resolve reverse
        /// lookup queries using automatically configured records for VPC resources. This only applies
        /// to networks listed under `private_visibility_config`.
        pub reverse_lookup: pulumi_wasm_rust::Output<Option<bool>>,
        /// The presence of this field indicates that this zone is backed by Service Directory. The value of this field contains information related to the namespace associated with the zone.
        /// Structure is documented below.
        pub service_directory_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dns::ManagedZoneServiceDirectoryConfig>,
        >,
        /// The zone's visibility: public zones are exposed to the Internet,
        /// while private zones are visible only to Virtual Private Cloud resources.
        /// Default value is `public`.
        /// Possible values are: `private`, `public`.
        pub visibility: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ManagedZoneArgs,
    ) -> ManagedZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloud_logging_config_binding = args
            .cloud_logging_config
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let dns_name_binding = args.dns_name.get_output(context).get_inner();
        let dnssec_config_binding = args.dnssec_config.get_output(context).get_inner();
        let force_destroy_binding = args.force_destroy.get_output(context).get_inner();
        let forwarding_config_binding = args
            .forwarding_config
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let peering_config_binding = args.peering_config.get_output(context).get_inner();
        let private_visibility_config_binding = args
            .private_visibility_config
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let reverse_lookup_binding = args.reverse_lookup.get_output(context).get_inner();
        let service_directory_config_binding = args
            .service_directory_config
            .get_output(context)
            .get_inner();
        let visibility_binding = args.visibility.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dns/managedZone:ManagedZone".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudLoggingConfig".into(),
                    value: &cloud_logging_config_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dnsName".into(),
                    value: &dns_name_binding,
                },
                register_interface::ObjectField {
                    name: "dnssecConfig".into(),
                    value: &dnssec_config_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "forwardingConfig".into(),
                    value: &forwarding_config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "peeringConfig".into(),
                    value: &peering_config_binding,
                },
                register_interface::ObjectField {
                    name: "privateVisibilityConfig".into(),
                    value: &private_visibility_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "reverseLookup".into(),
                    value: &reverse_lookup_binding,
                },
                register_interface::ObjectField {
                    name: "serviceDirectoryConfig".into(),
                    value: &service_directory_config_binding,
                },
                register_interface::ObjectField {
                    name: "visibility".into(),
                    value: &visibility_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cloudLoggingConfig".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "dnssecConfig".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
                },
                register_interface::ResultField {
                    name: "forwardingConfig".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "managedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nameServers".into(),
                },
                register_interface::ResultField {
                    name: "peeringConfig".into(),
                },
                register_interface::ResultField {
                    name: "privateVisibilityConfig".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "reverseLookup".into(),
                },
                register_interface::ResultField {
                    name: "serviceDirectoryConfig".into(),
                },
                register_interface::ResultField {
                    name: "visibility".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedZoneResult {
            cloud_logging_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudLoggingConfig").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            dnssec_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnssecConfig").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
            forwarding_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forwardingConfig").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            managed_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedZoneId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameServers").unwrap(),
            ),
            peering_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peeringConfig").unwrap(),
            ),
            private_visibility_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateVisibilityConfig").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            reverse_lookup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reverseLookup").unwrap(),
            ),
            service_directory_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceDirectoryConfig").unwrap(),
            ),
            visibility: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibility").unwrap(),
            ),
        }
    }
}
