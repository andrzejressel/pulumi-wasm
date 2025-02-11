/// Represents a Global Address resource. Global addresses are used for
/// HTTP(S) load balancing.
///
///
/// To get more information about GlobalAddress, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/globalAddresses)
/// * How-to Guides
///     * [Reserving a Static External IP Address](https://cloud.google.com/compute/docs/ip-addresses/reserve-static-external-ip-address)
///
/// ## Example Usage
///
/// ### Global Address Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = global_address::create(
///         "default",
///         GlobalAddressArgs::builder().name("global-appserver-ip").build_struct(),
///     );
/// }
/// ```
/// ### Global Address Private Services Connect
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = global_address::create(
///         "default",
///         GlobalAddressArgs::builder()
///             .address("100.100.100.105")
///             .address_type("INTERNAL")
///             .name("global-psconnect-ip")
///             .network("${network.id}")
///             .purpose("PRIVATE_SERVICE_CONNECT")
///             .build_struct(),
///     );
///     let network = network::create(
///         "network",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// GlobalAddress can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/addresses/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, GlobalAddress can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/globalAddress:GlobalAddress default projects/{{project}}/global/addresses/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/globalAddress:GlobalAddress default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/globalAddress:GlobalAddress default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod global_address {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalAddressArgs {
        /// The IP address or beginning of the address range represented by this
        /// resource. This can be supplied as an input to reserve a specific
        /// address or omitted to allow GCP to choose a valid one for you.
        #[builder(into, default)]
        pub address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the address to reserve.
        /// * EXTERNAL indicates public/external single IP address.
        /// * INTERNAL indicates internal IP ranges belonging to some network.
        /// Default value is `EXTERNAL`.
        /// Possible values are: `EXTERNAL`, `INTERNAL`.
        #[builder(into, default)]
        pub address_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP Version that will be used by this address. The default value is `IPV4`.
        /// Possible values are: `IPV4`, `IPV6`.
        #[builder(into, default)]
        pub ip_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels to apply to this address.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the network in which to reserve the IP range. The IP range
        /// must be in RFC1918 space. The network cannot be deleted if there are
        /// any reserved IP ranges referring to it.
        /// This should only be set when using an Internal address.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The prefix length of the IP range. If not present, it means the
        /// address field is a single IP address.
        /// This field is not applicable to addresses with addressType=INTERNAL
        /// when purpose=PRIVATE_SERVICE_CONNECT
        #[builder(into, default)]
        pub prefix_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The purpose of the resource. Possible values include:
        /// * VPC_PEERING - for peer networks
        /// * PRIVATE_SERVICE_CONNECT - for  Private Service Connect networks
        #[builder(into, default)]
        pub purpose: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GlobalAddressResult {
        /// The IP address or beginning of the address range represented by this
        /// resource. This can be supplied as an input to reserve a specific
        /// address or omitted to allow GCP to choose a valid one for you.
        pub address: pulumi_gestalt_rust::Output<String>,
        /// The type of the address to reserve.
        /// * EXTERNAL indicates public/external single IP address.
        /// * INTERNAL indicates internal IP ranges belonging to some network.
        /// Default value is `EXTERNAL`.
        /// Possible values are: `EXTERNAL`, `INTERNAL`.
        pub address_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IP Version that will be used by this address. The default value is `IPV4`.
        /// Possible values are: `IPV4`, `IPV6`.
        pub ip_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Labels to apply to this address.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The URL of the network in which to reserve the IP range. The IP range
        /// must be in RFC1918 space. The network cannot be deleted if there are
        /// any reserved IP ranges referring to it.
        /// This should only be set when using an Internal address.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// The prefix length of the IP range. If not present, it means the
        /// address field is a single IP address.
        /// This field is not applicable to addresses with addressType=INTERNAL
        /// when purpose=PRIVATE_SERVICE_CONNECT
        pub prefix_length: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The purpose of the resource. Possible values include:
        /// * VPC_PEERING - for peer networks
        /// * PRIVATE_SERVICE_CONNECT - for  Private Service Connect networks
        pub purpose: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlobalAddressArgs,
    ) -> GlobalAddressResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_binding = args.address.get_output(context);
        let address_type_binding = args.address_type.get_output(context);
        let description_binding = args.description.get_output(context);
        let ip_version_binding = args.ip_version.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let prefix_length_binding = args.prefix_length.get_output(context);
        let project_binding = args.project.get_output(context);
        let purpose_binding = args.purpose.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/globalAddress:GlobalAddress".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "address".into(),
                    value: &address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressType".into(),
                    value: &address_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipVersion".into(),
                    value: &ip_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefixLength".into(),
                    value: &prefix_length_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purpose".into(),
                    value: &purpose_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GlobalAddressResult {
            address: o.get_field("address"),
            address_type: o.get_field("addressType"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            ip_version: o.get_field("ipVersion"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            prefix_length: o.get_field("prefixLength"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            purpose: o.get_field("purpose"),
            self_link: o.get_field("selfLink"),
        }
    }
}
