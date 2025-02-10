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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod interconnect {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InterconnectArgs {
        /// Administrative status of the interconnect. When this is set to true, the Interconnect is
        /// functional and can carry traffic. When set to false, no packets can be carried over the
        /// interconnect and no BGP routes are exchanged over it. By default, the status is set to true.
        #[builder(into, default)]
        pub admin_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Customer name, to put in the Letter of Authorization as the party authorized to request a
        /// crossconnect. This field is required for Dedicated and Partner Interconnect, should not be specified
        /// for cross-cloud interconnect.
        #[builder(into, default)]
        pub customer_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional description of this resource. Provide this property when you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of interconnect. Note that a value IT_PRIVATE has been deprecated in favor of DEDICATED.
        /// Can take one of the following values:
        /// - PARTNER: A partner-managed interconnection shared between customers though a partner.
        /// - DEDICATED: A dedicated physical interconnection with the customer.
        /// Possible values are: `DEDICATED`, `PARTNER`, `IT_PRIVATE`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub interconnect_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Labels for this resource. These can only be added or modified by the setLabels
        /// method. Each label key/value pair must comply with RFC1035. Label values may be empty.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of link requested. Note that this field indicates the speed of each of the links in the
        /// bundle, not the speed of the entire bundle. Can take one of the following values:
        /// - LINK_TYPE_ETHERNET_10G_LR: A 10G Ethernet with LR optics.
        /// - LINK_TYPE_ETHERNET_100G_LR: A 100G Ethernet with LR optics.
        /// Possible values are: `LINK_TYPE_ETHERNET_10G_LR`, `LINK_TYPE_ETHERNET_100G_LR`.
        #[builder(into)]
        pub link_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// URL of the InterconnectLocation object that represents where this connection is to be provisioned.
        /// Specifies the location inside Google's Networks, should not be passed in case of cross-cloud interconnect.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration that enables Media Access Control security (MACsec) on the Cloud
        /// Interconnect connection between Google and your on-premises router.
        /// Structure is documented below.
        #[builder(into, default)]
        pub macsec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InterconnectMacsec>,
        >,
        /// Enable or disable MACsec on this Interconnect connection.
        /// MACsec enablement fails if the MACsec object is not specified.
        #[builder(into, default)]
        pub macsec_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the resource. Provided by the client when the resource is created. The name must be
        /// 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters must be a dash,
        /// lowercase letter, or digit, except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Email address to contact the customer NOC for operations and maintenance notifications
        /// regarding this Interconnect. If specified, this will be used for notifications in addition to
        /// all other forms described, such as Cloud Monitoring logs alerting and Cloud Notifications.
        /// This field is required for users who sign up for Cloud Interconnect using workforce identity
        /// federation.
        #[builder(into, default)]
        pub noc_contact_email: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates that this is a Cross-Cloud Interconnect. This field specifies the location outside
        /// of Google's network that the interconnect is connected to.
        #[builder(into, default)]
        pub remote_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// interconnects.list of features requested for this Interconnect connection. Options: IF_MACSEC (
        /// If specified then the connection is created on MACsec capable hardware ports. If not
        /// specified, the default value is false, which allocates non-MACsec capable ports first if
        /// available). Note that MACSEC is still technically allowed for compatibility reasons, but it
        /// does not work with the API, and will be removed in an upcoming major version.
        /// Each value may be one of: `MACSEC`, `IF_MACSEC`.
        #[builder(into, default)]
        pub requested_features: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Target number of physical links in the link bundle, as requested by the customer.
        #[builder(into)]
        pub requested_link_count: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct InterconnectResult {
        /// Administrative status of the interconnect. When this is set to true, the Interconnect is
        /// functional and can carry traffic. When set to false, no packets can be carried over the
        /// interconnect and no BGP routes are exchanged over it. By default, the status is set to true.
        pub admin_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// interconnects.list of features available for this Interconnect connection. Can take the value:
        /// MACSEC. If present then the Interconnect connection is provisioned on MACsec capable hardware
        /// ports. If not present then the Interconnect connection is provisioned on non-MACsec capable
        /// ports and MACsec isn't supported and enabling MACsec fails).
        pub available_features: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of CircuitInfo objects, that describe the individual circuits in this LAG.
        /// Structure is documented below.
        pub circuit_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InterconnectCircuitInfo>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Customer name, to put in the Letter of Authorization as the party authorized to request a
        /// crossconnect. This field is required for Dedicated and Partner Interconnect, should not be specified
        /// for cross-cloud interconnect.
        pub customer_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// An optional description of this resource. Provide this property when you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of outages expected for this Interconnect.
        /// Structure is documented below.
        pub expected_outages: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InterconnectExpectedOutage>,
        >,
        /// IP address configured on the Google side of the Interconnect link.
        /// This can be used only for ping tests.
        pub google_ip_address: pulumi_gestalt_rust::Output<String>,
        /// Google reference ID to be used when raising support tickets with Google or otherwise to debug
        /// backend connectivity issues.
        pub google_reference_id: pulumi_gestalt_rust::Output<String>,
        /// A list of the URLs of all InterconnectAttachments configured to use this Interconnect.
        pub interconnect_attachments: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Type of interconnect. Note that a value IT_PRIVATE has been deprecated in favor of DEDICATED.
        /// Can take one of the following values:
        /// - PARTNER: A partner-managed interconnection shared between customers though a partner.
        /// - DEDICATED: A dedicated physical interconnection with the customer.
        /// Possible values are: `DEDICATED`, `PARTNER`, `IT_PRIVATE`.
        ///
        ///
        /// - - -
        pub interconnect_type: pulumi_gestalt_rust::Output<String>,
        /// A fingerprint for the labels being applied to this Interconnect, which is essentially a hash
        /// of the labels set used for optimistic locking. The fingerprint is initially generated by
        /// Compute Engine and changes after every request to modify or update labels.
        /// You must always provide an up-to-date fingerprint hash in order to update or change labels,
        /// otherwise the request will fail with error 412 conditionNotMet.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Labels for this resource. These can only be added or modified by the setLabels
        /// method. Each label key/value pair must comply with RFC1035. Label values may be empty.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of link requested. Note that this field indicates the speed of each of the links in the
        /// bundle, not the speed of the entire bundle. Can take one of the following values:
        /// - LINK_TYPE_ETHERNET_10G_LR: A 10G Ethernet with LR optics.
        /// - LINK_TYPE_ETHERNET_100G_LR: A 100G Ethernet with LR optics.
        /// Possible values are: `LINK_TYPE_ETHERNET_10G_LR`, `LINK_TYPE_ETHERNET_100G_LR`.
        pub link_type: pulumi_gestalt_rust::Output<String>,
        /// URL of the InterconnectLocation object that represents where this connection is to be provisioned.
        /// Specifies the location inside Google's Networks, should not be passed in case of cross-cloud interconnect.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration that enables Media Access Control security (MACsec) on the Cloud
        /// Interconnect connection between Google and your on-premises router.
        /// Structure is documented below.
        pub macsec: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InterconnectMacsec>,
        >,
        /// Enable or disable MACsec on this Interconnect connection.
        /// MACsec enablement fails if the MACsec object is not specified.
        pub macsec_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the resource. Provided by the client when the resource is created. The name must be
        /// 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters must be a dash,
        /// lowercase letter, or digit, except the last character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Email address to contact the customer NOC for operations and maintenance notifications
        /// regarding this Interconnect. If specified, this will be used for notifications in addition to
        /// all other forms described, such as Cloud Monitoring logs alerting and Cloud Notifications.
        /// This field is required for users who sign up for Cloud Interconnect using workforce identity
        /// federation.
        pub noc_contact_email: pulumi_gestalt_rust::Output<Option<String>>,
        /// The current status of this Interconnect's functionality, which can take one of the following:
        /// - OS_ACTIVE: A valid Interconnect, which is turned up and is ready to use. Attachments may
        /// be provisioned on this Interconnect.
        /// - OS_UNPROVISIONED: An Interconnect that has not completed turnup. No attachments may be
        /// provisioned on this Interconnect.
        /// - OS_UNDER_MAINTENANCE: An Interconnect that is undergoing internal maintenance. No
        /// attachments may be provisioned or updated on this Interconnect.
        pub operational_status: pulumi_gestalt_rust::Output<String>,
        /// IP address configured on the customer side of the Interconnect link.
        /// The customer should configure this IP address during turnup when prompted by Google NOC.
        /// This can be used only for ping tests.
        pub peer_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Number of links actually provisioned in this interconnect.
        pub provisioned_link_count: pulumi_gestalt_rust::Output<i32>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates that this is a Cross-Cloud Interconnect. This field specifies the location outside
        /// of Google's network that the interconnect is connected to.
        pub remote_location: pulumi_gestalt_rust::Output<Option<String>>,
        /// interconnects.list of features requested for this Interconnect connection. Options: IF_MACSEC (
        /// If specified then the connection is created on MACsec capable hardware ports. If not
        /// specified, the default value is false, which allocates non-MACsec capable ports first if
        /// available). Note that MACSEC is still technically allowed for compatibility reasons, but it
        /// does not work with the API, and will be removed in an upcoming major version.
        /// Each value may be one of: `MACSEC`, `IF_MACSEC`.
        pub requested_features: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Target number of physical links in the link bundle, as requested by the customer.
        pub requested_link_count: pulumi_gestalt_rust::Output<i32>,
        /// Reserved for future use.
        pub satisfies_pzs: pulumi_gestalt_rust::Output<bool>,
        /// (Output)
        /// State of this notification. Note that the versions of this enum prefixed with "NS_" have
        /// been deprecated in favor of the unprefixed values. Can take one of the following values:
        /// - ACTIVE: This outage notification is active. The event could be in the past, present,
        /// or future. See startTime and endTime for scheduling.
        /// - CANCELLED: The outage associated with this notification was cancelled before the
        /// outage was due to start.
        /// - COMPLETED: The outage associated with this notification is complete.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InterconnectArgs,
    ) -> InterconnectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_enabled_binding = args.admin_enabled.get_output(context);
        let customer_name_binding = args.customer_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let interconnect_type_binding = args.interconnect_type.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let link_type_binding = args.link_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let macsec_binding = args.macsec.get_output(context);
        let macsec_enabled_binding = args.macsec_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let noc_contact_email_binding = args.noc_contact_email.get_output(context);
        let project_binding = args.project.get_output(context);
        let remote_location_binding = args.remote_location.get_output(context);
        let requested_features_binding = args.requested_features.get_output(context);
        let requested_link_count_binding = args.requested_link_count.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/interconnect:Interconnect".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminEnabled".into(),
                    value: admin_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerName".into(),
                    value: customer_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interconnectType".into(),
                    value: interconnect_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkType".into(),
                    value: link_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "macsec".into(),
                    value: macsec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "macsecEnabled".into(),
                    value: macsec_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nocContactEmail".into(),
                    value: noc_contact_email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteLocation".into(),
                    value: remote_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestedFeatures".into(),
                    value: requested_features_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestedLinkCount".into(),
                    value: requested_link_count_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InterconnectResult {
            admin_enabled: o.get_field("adminEnabled"),
            available_features: o.get_field("availableFeatures"),
            circuit_infos: o.get_field("circuitInfos"),
            creation_timestamp: o.get_field("creationTimestamp"),
            customer_name: o.get_field("customerName"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            expected_outages: o.get_field("expectedOutages"),
            google_ip_address: o.get_field("googleIpAddress"),
            google_reference_id: o.get_field("googleReferenceId"),
            interconnect_attachments: o.get_field("interconnectAttachments"),
            interconnect_type: o.get_field("interconnectType"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            link_type: o.get_field("linkType"),
            location: o.get_field("location"),
            macsec: o.get_field("macsec"),
            macsec_enabled: o.get_field("macsecEnabled"),
            name: o.get_field("name"),
            noc_contact_email: o.get_field("nocContactEmail"),
            operational_status: o.get_field("operationalStatus"),
            peer_ip_address: o.get_field("peerIpAddress"),
            project: o.get_field("project"),
            provisioned_link_count: o.get_field("provisionedLinkCount"),
            pulumi_labels: o.get_field("pulumiLabels"),
            remote_location: o.get_field("remoteLocation"),
            requested_features: o.get_field("requestedFeatures"),
            requested_link_count: o.get_field("requestedLinkCount"),
            satisfies_pzs: o.get_field("satisfiesPzs"),
            state: o.get_field("state"),
        }
    }
}
