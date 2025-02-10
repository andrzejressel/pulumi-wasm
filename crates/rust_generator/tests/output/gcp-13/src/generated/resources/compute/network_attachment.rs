/// A network attachment is a resource that lets a producer Virtual Private Cloud (VPC) network initiate connections to a consumer VPC network through a Private Service Connect interface.
///
///
/// To get more information about NetworkAttachment, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/networkAttachments)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vpc/docs/about-network-attachments)
///
/// ## Example Usage
///
/// ### Network Attachment Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let acceptedProducerProject = project::create(
///         "acceptedProducerProject",
///         ProjectArgs::builder()
///             .billing_account("000000-0000000-0000000-000000")
///             .deletion_policy("DELETE")
///             .name("prj-accepted")
///             .org_id("123456789")
///             .project_id("prj-accepted")
///             .build_struct(),
///     );
///     let default = network_attachment::create(
///         "default",
///         NetworkAttachmentArgs::builder()
///             .connection_preference("ACCEPT_MANUAL")
///             .description("basic network attachment description")
///             .name("basic-network-attachment")
///             .producer_accept_lists(vec!["${acceptedProducerProject.projectId}",])
///             .producer_reject_lists(vec!["${rejectedProducerProject.projectId}",])
///             .region("us-central1")
///             .subnetworks(vec!["${defaultSubnetwork.selfLink}",])
///             .build_struct(),
///     );
///     let defaultNetwork = network::create(
///         "defaultNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("basic-network")
///             .build_struct(),
///     );
///     let defaultSubnetwork = subnetwork::create(
///         "defaultSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("basic-subnetwork")
///             .network("${defaultNetwork.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let rejectedProducerProject = project::create(
///         "rejectedProducerProject",
///         ProjectArgs::builder()
///             .billing_account("000000-0000000-0000000-000000")
///             .deletion_policy("DELETE")
///             .name("prj-rejected")
///             .org_id("123456789")
///             .project_id("prj-rejected")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Attachment Instance Usage
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = network::create(
///         "default",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("basic-network")
///             .build_struct(),
///     );
///     let defaultInstance = instance::create(
///         "defaultInstance",
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
///             .machine_type("e2-micro")
///             .name("basic-instance")
///             .network_interfaces(
///                 vec![
///                     InstanceNetworkInterface::builder().network("default")
///                     .build_struct(), InstanceNetworkInterface::builder()
///                     .networkAttachment("${defaultNetworkAttachment.selfLink}")
///                     .build_struct(),
///                 ],
///             )
///             .zone("us-central1-a")
///             .build_struct(),
///     );
///     let defaultNetworkAttachment = network_attachment::create(
///         "defaultNetworkAttachment",
///         NetworkAttachmentArgs::builder()
///             .connection_preference("ACCEPT_AUTOMATIC")
///             .description("my basic network attachment")
///             .name("basic-network-attachment")
///             .region("us-central1")
///             .subnetworks(vec!["${defaultSubnetwork.id}",])
///             .build_struct(),
///     );
///     let defaultSubnetwork = subnetwork::create(
///         "defaultSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("basic-subnetwork")
///             .network("${default.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// NetworkAttachment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/networkAttachments/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, NetworkAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/networkAttachment:NetworkAttachment default projects/{{project}}/regions/{{region}}/networkAttachments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkAttachment:NetworkAttachment default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkAttachment:NetworkAttachment default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkAttachment:NetworkAttachment default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkAttachmentArgs {
        /// The connection preference of service attachment. The value can be set to ACCEPT_AUTOMATIC. An ACCEPT_AUTOMATIC service attachment is one that always accepts the connection from consumer forwarding rules.
        /// Possible values are: `ACCEPT_AUTOMATIC`, `ACCEPT_MANUAL`, `INVALID`.
        #[builder(into)]
        pub connection_preference: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional description of this resource. Provide this property when you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression a-z? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Projects that are allowed to connect to this network attachment. The project can be specified using its id or number.
        #[builder(into, default)]
        pub producer_accept_lists: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Projects that are not allowed to connect to this network attachment. The project can be specified using its id or number.
        #[builder(into, default)]
        pub producer_reject_lists: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL of the region where the network attachment resides. This field applies only to the region resource. You must specify this field as part of the HTTP request URL. It is not settable as a field in the request body.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An array of URLs where each entry is the URL of a subnet provided by the service consumer to use for endpoints in the producers that connect to this network attachment.
        #[builder(into)]
        pub subnetworks: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkAttachmentResult {
        /// An array of connections for all the producers connected to this network attachment.
        /// Structure is documented below.
        pub connection_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::NetworkAttachmentConnectionEndpoint>,
        >,
        /// The connection preference of service attachment. The value can be set to ACCEPT_AUTOMATIC. An ACCEPT_AUTOMATIC service attachment is one that always accepts the connection from consumer forwarding rules.
        /// Possible values are: `ACCEPT_AUTOMATIC`, `ACCEPT_MANUAL`, `INVALID`.
        pub connection_preference: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource. Provide this property when you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Fingerprint of this resource. A hash of the contents stored in this object. This
        /// field is used in optimistic locking. An up-to-date fingerprint must be provided in order to patch.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Type of the resource.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression a-z? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The URL of the network which the Network Attachment belongs to. Practically it is inferred by fetching the network of the first subnetwork associated.
        /// Because it is required that all the subnetworks must be from the same network, it is assured that the Network Attachment belongs to the same network as all the subnetworks.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// Projects that are allowed to connect to this network attachment. The project can be specified using its id or number.
        pub producer_accept_lists: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Projects that are not allowed to connect to this network attachment. The project can be specified using its id or number.
        pub producer_reject_lists: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// URL of the region where the network attachment resides. This field applies only to the region resource. You must specify this field as part of the HTTP request URL. It is not settable as a field in the request body.
        ///
        ///
        /// - - -
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Server-defined URL for the resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Server-defined URL for this resource's resource id.
        pub self_link_with_id: pulumi_gestalt_rust::Output<String>,
        /// An array of URLs where each entry is the URL of a subnet provided by the service consumer to use for endpoints in the producers that connect to this network attachment.
        pub subnetworks: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkAttachmentArgs,
    ) -> NetworkAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_preference_binding = args
            .connection_preference
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let producer_accept_lists_binding = args
            .producer_accept_lists
            .get_output(context);
        let producer_reject_lists_binding = args
            .producer_reject_lists
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let subnetworks_binding = args.subnetworks.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/networkAttachment:NetworkAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionPreference".into(),
                    value: connection_preference_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "producerAcceptLists".into(),
                    value: producer_accept_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "producerRejectLists".into(),
                    value: producer_reject_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetworks".into(),
                    value: subnetworks_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkAttachmentResult {
            connection_endpoints: o.get_field("connectionEndpoints"),
            connection_preference: o.get_field("connectionPreference"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            fingerprint: o.get_field("fingerprint"),
            kind: o.get_field("kind"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            producer_accept_lists: o.get_field("producerAcceptLists"),
            producer_reject_lists: o.get_field("producerRejectLists"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            self_link_with_id: o.get_field("selfLinkWithId"),
            subnetworks: o.get_field("subnetworks"),
        }
    }
}
