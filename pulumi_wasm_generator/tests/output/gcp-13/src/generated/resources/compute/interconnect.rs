/// Represents an Interconnect resource. The Interconnect resource is a dedicated connection between
/// Google's network and your on-premises network.
///
///
/// To get more information about Interconnect, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/interconnects)
/// * How-to Guides
///     * [Create a Dedicated Interconnect](https://cloud.google.com/network-connectivity/docs/interconnect/concepts/dedicated-overview)
///
/// ## Example Usage
///
/// ### Compute Interconnect Basic
///
///
/// ```yaml
/// resources:
///   example-interconnect:
///     type: gcp:compute:Interconnect
///     properties:
///       name: example-interconnect
///       customerName: example_customer
///       interconnectType: DEDICATED
///       linkType: LINK_TYPE_ETHERNET_10G_LR
///       location: https://www.googleapis.com/compute/v1/projects/${project.name}/global/interconnectLocations/iad-zone1-1
///       requestedLinkCount: 1
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Interconnect can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/interconnects/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Interconnect can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/interconnect:Interconnect default projects/{{project}}/global/interconnects/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/interconnect:Interconnect default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/interconnect:Interconnect default {{name}}
/// ```
///
pub mod interconnect {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InterconnectArgs {
        /// Administrative status of the interconnect. When this is set to true, the Interconnect is
        /// functional and can carry traffic. When set to false, no packets can be carried over the
        /// interconnect and no BGP routes are exchanged over it. By default, the status is set to true.
        #[builder(into, default)]
        pub admin_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Customer name, to put in the Letter of Authorization as the party authorized to request a
        /// crossconnect. This field is required for Dedicated and Partner Interconnect, should not be specified
        /// for cross-cloud interconnect.
        #[builder(into, default)]
        pub customer_name: pulumi_wasm_rust::Output<Option<String>>,
        /// An optional description of this resource. Provide this property when you create the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of interconnect. Note that a value IT_PRIVATE has been deprecated in favor of DEDICATED.
        /// Can take one of the following values:
        /// - PARTNER: A partner-managed interconnection shared between customers though a partner.
        /// - DEDICATED: A dedicated physical interconnection with the customer.
        /// Possible values are: `DEDICATED`, `PARTNER`, `IT_PRIVATE`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub interconnect_type: pulumi_wasm_rust::Output<String>,
        /// Labels for this resource. These can only be added or modified by the setLabels
        /// method. Each label key/value pair must comply with RFC1035. Label values may be empty.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of link requested. Note that this field indicates the speed of each of the links in the
        /// bundle, not the speed of the entire bundle. Can take one of the following values:
        /// - LINK_TYPE_ETHERNET_10G_LR: A 10G Ethernet with LR optics.
        /// - LINK_TYPE_ETHERNET_100G_LR: A 100G Ethernet with LR optics.
        /// Possible values are: `LINK_TYPE_ETHERNET_10G_LR`, `LINK_TYPE_ETHERNET_100G_LR`.
        #[builder(into)]
        pub link_type: pulumi_wasm_rust::Output<String>,
        /// URL of the InterconnectLocation object that represents where this connection is to be provisioned.
        /// Specifies the location inside Google's Networks, should not be passed in case of cross-cloud interconnect.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration that enables Media Access Control security (MACsec) on the Cloud
        /// Interconnect connection between Google and your on-premises router.
        /// Structure is documented below.
        #[builder(into, default)]
        pub macsec: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InterconnectMacsec>,
        >,
        /// Enable or disable MACsec on this Interconnect connection.
        /// MACsec enablement fails if the MACsec object is not specified.
        #[builder(into, default)]
        pub macsec_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the resource. Provided by the client when the resource is created. The name must be
        /// 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters must be a dash,
        /// lowercase letter, or digit, except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Email address to contact the customer NOC for operations and maintenance notifications
        /// regarding this Interconnect. If specified, this will be used for notifications in addition to
        /// all other forms described, such as Cloud Monitoring logs alerting and Cloud Notifications.
        /// This field is required for users who sign up for Cloud Interconnect using workforce identity
        /// federation.
        #[builder(into, default)]
        pub noc_contact_email: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates that this is a Cross-Cloud Interconnect. This field specifies the location outside
        /// of Google's network that the interconnect is connected to.
        #[builder(into, default)]
        pub remote_location: pulumi_wasm_rust::Output<Option<String>>,
        /// interconnects.list of features requested for this Interconnect connection. Options: IF_MACSEC (
        /// If specified then the connection is created on MACsec capable hardware ports. If not
        /// specified, the default value is false, which allocates non-MACsec capable ports first if
        /// available). Note that MACSEC is still technically allowed for compatibility reasons, but it
        /// does not work with the API, and will be removed in an upcoming major version.
        /// Each value may be one of: `MACSEC`, `IF_MACSEC`.
        #[builder(into, default)]
        pub requested_features: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Target number of physical links in the link bundle, as requested by the customer.
        #[builder(into)]
        pub requested_link_count: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct InterconnectResult {
        /// Administrative status of the interconnect. When this is set to true, the Interconnect is
        /// functional and can carry traffic. When set to false, no packets can be carried over the
        /// interconnect and no BGP routes are exchanged over it. By default, the status is set to true.
        pub admin_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// interconnects.list of features available for this Interconnect connection. Can take the value:
        /// MACSEC. If present then the Interconnect connection is provisioned on MACsec capable hardware
        /// ports. If not present then the Interconnect connection is provisioned on non-MACsec capable
        /// ports and MACsec isn't supported and enabling MACsec fails).
        pub available_features: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of CircuitInfo objects, that describe the individual circuits in this LAG.
        /// Structure is documented below.
        pub circuit_infos: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::InterconnectCircuitInfo>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// Customer name, to put in the Letter of Authorization as the party authorized to request a
        /// crossconnect. This field is required for Dedicated and Partner Interconnect, should not be specified
        /// for cross-cloud interconnect.
        pub customer_name: pulumi_wasm_rust::Output<Option<String>>,
        /// An optional description of this resource. Provide this property when you create the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of outages expected for this Interconnect.
        /// Structure is documented below.
        pub expected_outages: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::InterconnectExpectedOutage>,
        >,
        /// IP address configured on the Google side of the Interconnect link.
        /// This can be used only for ping tests.
        pub google_ip_address: pulumi_wasm_rust::Output<String>,
        /// Google reference ID to be used when raising support tickets with Google or otherwise to debug
        /// backend connectivity issues.
        pub google_reference_id: pulumi_wasm_rust::Output<String>,
        /// A list of the URLs of all InterconnectAttachments configured to use this Interconnect.
        pub interconnect_attachments: pulumi_wasm_rust::Output<Vec<String>>,
        /// Type of interconnect. Note that a value IT_PRIVATE has been deprecated in favor of DEDICATED.
        /// Can take one of the following values:
        /// - PARTNER: A partner-managed interconnection shared between customers though a partner.
        /// - DEDICATED: A dedicated physical interconnection with the customer.
        /// Possible values are: `DEDICATED`, `PARTNER`, `IT_PRIVATE`.
        ///
        ///
        /// - - -
        pub interconnect_type: pulumi_wasm_rust::Output<String>,
        /// A fingerprint for the labels being applied to this Interconnect, which is essentially a hash
        /// of the labels set used for optimistic locking. The fingerprint is initially generated by
        /// Compute Engine and changes after every request to modify or update labels.
        /// You must always provide an up-to-date fingerprint hash in order to update or change labels,
        /// otherwise the request will fail with error 412 conditionNotMet.
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        /// Labels for this resource. These can only be added or modified by the setLabels
        /// method. Each label key/value pair must comply with RFC1035. Label values may be empty.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of link requested. Note that this field indicates the speed of each of the links in the
        /// bundle, not the speed of the entire bundle. Can take one of the following values:
        /// - LINK_TYPE_ETHERNET_10G_LR: A 10G Ethernet with LR optics.
        /// - LINK_TYPE_ETHERNET_100G_LR: A 100G Ethernet with LR optics.
        /// Possible values are: `LINK_TYPE_ETHERNET_10G_LR`, `LINK_TYPE_ETHERNET_100G_LR`.
        pub link_type: pulumi_wasm_rust::Output<String>,
        /// URL of the InterconnectLocation object that represents where this connection is to be provisioned.
        /// Specifies the location inside Google's Networks, should not be passed in case of cross-cloud interconnect.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration that enables Media Access Control security (MACsec) on the Cloud
        /// Interconnect connection between Google and your on-premises router.
        /// Structure is documented below.
        pub macsec: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InterconnectMacsec>,
        >,
        /// Enable or disable MACsec on this Interconnect connection.
        /// MACsec enablement fails if the MACsec object is not specified.
        pub macsec_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the resource. Provided by the client when the resource is created. The name must be
        /// 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters must be a dash,
        /// lowercase letter, or digit, except the last character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Email address to contact the customer NOC for operations and maintenance notifications
        /// regarding this Interconnect. If specified, this will be used for notifications in addition to
        /// all other forms described, such as Cloud Monitoring logs alerting and Cloud Notifications.
        /// This field is required for users who sign up for Cloud Interconnect using workforce identity
        /// federation.
        pub noc_contact_email: pulumi_wasm_rust::Output<Option<String>>,
        /// The current status of this Interconnect's functionality, which can take one of the following:
        /// - OS_ACTIVE: A valid Interconnect, which is turned up and is ready to use. Attachments may
        /// be provisioned on this Interconnect.
        /// - OS_UNPROVISIONED: An Interconnect that has not completed turnup. No attachments may be
        /// provisioned on this Interconnect.
        /// - OS_UNDER_MAINTENANCE: An Interconnect that is undergoing internal maintenance. No
        /// attachments may be provisioned or updated on this Interconnect.
        pub operational_status: pulumi_wasm_rust::Output<String>,
        /// IP address configured on the customer side of the Interconnect link.
        /// The customer should configure this IP address during turnup when prompted by Google NOC.
        /// This can be used only for ping tests.
        pub peer_ip_address: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Number of links actually provisioned in this interconnect.
        pub provisioned_link_count: pulumi_wasm_rust::Output<i32>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates that this is a Cross-Cloud Interconnect. This field specifies the location outside
        /// of Google's network that the interconnect is connected to.
        pub remote_location: pulumi_wasm_rust::Output<Option<String>>,
        /// interconnects.list of features requested for this Interconnect connection. Options: IF_MACSEC (
        /// If specified then the connection is created on MACsec capable hardware ports. If not
        /// specified, the default value is false, which allocates non-MACsec capable ports first if
        /// available). Note that MACSEC is still technically allowed for compatibility reasons, but it
        /// does not work with the API, and will be removed in an upcoming major version.
        /// Each value may be one of: `MACSEC`, `IF_MACSEC`.
        pub requested_features: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Target number of physical links in the link bundle, as requested by the customer.
        pub requested_link_count: pulumi_wasm_rust::Output<i32>,
        /// Reserved for future use.
        pub satisfies_pzs: pulumi_wasm_rust::Output<bool>,
        /// (Output)
        /// State of this notification. Note that the versions of this enum prefixed with "NS_" have
        /// been deprecated in favor of the unprefixed values. Can take one of the following values:
        /// - ACTIVE: This outage notification is active. The event could be in the past, present,
        /// or future. See startTime and endTime for scheduling.
        /// - CANCELLED: The outage associated with this notification was cancelled before the
        /// outage was due to start.
        /// - COMPLETED: The outage associated with this notification is complete.
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InterconnectArgs) -> InterconnectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_enabled_binding = args.admin_enabled.get_inner();
        let customer_name_binding = args.customer_name.get_inner();
        let description_binding = args.description.get_inner();
        let interconnect_type_binding = args.interconnect_type.get_inner();
        let labels_binding = args.labels.get_inner();
        let link_type_binding = args.link_type.get_inner();
        let location_binding = args.location.get_inner();
        let macsec_binding = args.macsec.get_inner();
        let macsec_enabled_binding = args.macsec_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let noc_contact_email_binding = args.noc_contact_email.get_inner();
        let project_binding = args.project.get_inner();
        let remote_location_binding = args.remote_location.get_inner();
        let requested_features_binding = args.requested_features.get_inner();
        let requested_link_count_binding = args.requested_link_count.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/interconnect:Interconnect".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminEnabled".into(),
                    value: &admin_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "customerName".into(),
                    value: &customer_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "interconnectType".into(),
                    value: &interconnect_type_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "linkType".into(),
                    value: &link_type_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "macsec".into(),
                    value: &macsec_binding,
                },
                register_interface::ObjectField {
                    name: "macsecEnabled".into(),
                    value: &macsec_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nocContactEmail".into(),
                    value: &noc_contact_email_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "remoteLocation".into(),
                    value: &remote_location_binding,
                },
                register_interface::ObjectField {
                    name: "requestedFeatures".into(),
                    value: &requested_features_binding,
                },
                register_interface::ObjectField {
                    name: "requestedLinkCount".into(),
                    value: &requested_link_count_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adminEnabled".into(),
                },
                register_interface::ResultField {
                    name: "availableFeatures".into(),
                },
                register_interface::ResultField {
                    name: "circuitInfos".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "customerName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "expectedOutages".into(),
                },
                register_interface::ResultField {
                    name: "googleIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "googleReferenceId".into(),
                },
                register_interface::ResultField {
                    name: "interconnectAttachments".into(),
                },
                register_interface::ResultField {
                    name: "interconnectType".into(),
                },
                register_interface::ResultField {
                    name: "labelFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "linkType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "macsec".into(),
                },
                register_interface::ResultField {
                    name: "macsecEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nocContactEmail".into(),
                },
                register_interface::ResultField {
                    name: "operationalStatus".into(),
                },
                register_interface::ResultField {
                    name: "peerIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "provisionedLinkCount".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "remoteLocation".into(),
                },
                register_interface::ResultField {
                    name: "requestedFeatures".into(),
                },
                register_interface::ResultField {
                    name: "requestedLinkCount".into(),
                },
                register_interface::ResultField {
                    name: "satisfiesPzs".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InterconnectResult {
            admin_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminEnabled").unwrap(),
            ),
            available_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availableFeatures").unwrap(),
            ),
            circuit_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("circuitInfos").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            customer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            expected_outages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedOutages").unwrap(),
            ),
            google_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("googleIpAddress").unwrap(),
            ),
            google_reference_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("googleReferenceId").unwrap(),
            ),
            interconnect_attachments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interconnectAttachments").unwrap(),
            ),
            interconnect_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interconnectType").unwrap(),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelFingerprint").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            link_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            macsec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("macsec").unwrap(),
            ),
            macsec_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("macsecEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            noc_contact_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nocContactEmail").unwrap(),
            ),
            operational_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operationalStatus").unwrap(),
            ),
            peer_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerIpAddress").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            provisioned_link_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisionedLinkCount").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            remote_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteLocation").unwrap(),
            ),
            requested_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestedFeatures").unwrap(),
            ),
            requested_link_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestedLinkCount").unwrap(),
            ),
            satisfies_pzs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("satisfiesPzs").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
        }
    }
}
