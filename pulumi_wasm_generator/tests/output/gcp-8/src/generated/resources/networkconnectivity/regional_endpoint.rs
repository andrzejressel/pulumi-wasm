/// Regional Private Service Connect (PSC) endpoint resource.
///
///
/// To get more information about RegionalEndpoint, see:
///
/// * [API documentation](https://cloud.google.com/network-connectivity/docs/reference/networkconnectivity/rest/v1/projects.locations.regionalEndpoints)
/// * How-to Guides
///     * [Access regional Google APIs through endpoints](https://cloud.google.com/vpc/docs/access-regional-google-apis-endpoints)
///
/// ## Example Usage
///
/// ### Network Connectivity Regional Endpoint Regional Access
///
///
/// ```yaml
/// resources:
///   myNetwork:
///     type: gcp:compute:Network
///     name: my_network
///     properties:
///       name: my-network
///       autoCreateSubnetworks: false
///   mySubnetwork:
///     type: gcp:compute:Subnetwork
///     name: my_subnetwork
///     properties:
///       name: my-subnetwork
///       ipCidrRange: 192.168.0.0/24
///       region: us-central1
///       network: ${myNetwork.id}
///   default:
///     type: gcp:networkconnectivity:RegionalEndpoint
///     properties:
///       name: my-rep
///       location: us-central1
///       targetGoogleApi: storage.us-central1.p.rep.googleapis.com
///       accessType: REGIONAL
///       address: 192.168.0.5
///       network: ${myNetwork.id}
///       subnetwork: ${mySubnetwork.id}
///       description: My RegionalEndpoint targeting Google API storage.us-central1.p.rep.googleapis.com
///       labels:
///         env: default
/// ```
/// ### Network Connectivity Regional Endpoint Global Access
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = regional_endpoint::create(
///         "default",
///         RegionalEndpointArgs::builder()
///             .access_type("GLOBAL")
///             .address("192.168.0.4")
///             .location("us-central1")
///             .name("my-rep")
///             .network("${myNetwork.id}")
///             .subnetwork("${mySubnetwork.id}")
///             .target_google_api("storage.us-central1.p.rep.googleapis.com")
///             .build_struct(),
///     );
///     let myNetwork = network::create(
///         "myNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
///     let mySubnetwork = subnetwork::create(
///         "mySubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("192.168.0.0/24")
///             .name("my-subnetwork")
///             .network("${myNetwork.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RegionalEndpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/regionalEndpoints/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, RegionalEndpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/regionalEndpoint:RegionalEndpoint default projects/{{project}}/locations/{{location}}/regionalEndpoints/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/regionalEndpoint:RegionalEndpoint default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/regionalEndpoint:RegionalEndpoint default {{location}}/{{name}}
/// ```
///
pub mod regional_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionalEndpointArgs {
        /// The access type of this regional endpoint. This field is reflected in the PSC Forwarding Rule configuration to enable global access.
        /// Possible values are: `GLOBAL`, `REGIONAL`.
        #[builder(into)]
        pub access_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The IP Address of the Regional Endpoint. When no address is provided, an IP from the subnetwork is allocated. Use one of the following formats: * IPv4 address as in `10.0.0.1` * Address resource URI as in `projects/{project}/regions/{region}/addresses/{address_name}`
        /// > **Note:** This field accepts both a reference to a Compute Address resource, which is the resource name of which format is given in the description, and IP literal value. If the user chooses to input a reserved address value; they need to make sure that the reserved address is in IPv4 version, its purpose is GCE_ENDPOINT, its type is INTERNAL and its status is RESERVED. If the user chooses to input an IP literal, they need to make sure that it's a valid IPv4 address (x.x.x.x) within the subnetwork.
        #[builder(into, default)]
        pub address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// User-defined labels.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the RegionalEndpoint.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the RegionalEndpoint.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the VPC network for this private regional endpoint. Format: `projects/{project}/global/networks/{network}`
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the subnetwork from which the IP address will be allocated. Format: `projects/{project}/regions/{region}/subnetworks/{subnetwork}`
        #[builder(into, default)]
        pub subnetwork: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The service endpoint this private regional endpoint connects to. Format: `{apiname}.{region}.p.rep.googleapis.com` Example: \"cloudkms.us-central1.p.rep.googleapis.com\".
        #[builder(into)]
        pub target_google_api: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionalEndpointResult {
        /// The access type of this regional endpoint. This field is reflected in the PSC Forwarding Rule configuration to enable global access.
        /// Possible values are: `GLOBAL`, `REGIONAL`.
        pub access_type: pulumi_wasm_rust::Output<String>,
        /// The IP Address of the Regional Endpoint. When no address is provided, an IP from the subnetwork is allocated. Use one of the following formats: * IPv4 address as in `10.0.0.1` * Address resource URI as in `projects/{project}/regions/{region}/addresses/{address_name}`
        /// > **Note:** This field accepts both a reference to a Compute Address resource, which is the resource name of which format is given in the description, and IP literal value. If the user chooses to input a reserved address value; they need to make sure that the reserved address is in IPv4 version, its purpose is GCE_ENDPOINT, its type is INTERNAL and its status is RESERVED. If the user chooses to input an IP literal, they need to make sure that it's a valid IPv4 address (x.x.x.x) within the subnetwork.
        pub address: pulumi_wasm_rust::Output<String>,
        /// Time when the RegionalEndpoint was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User-defined labels.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the RegionalEndpoint.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the RegionalEndpoint.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the VPC network for this private regional endpoint. Format: `projects/{project}/global/networks/{network}`
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The resource reference of the PSC Forwarding Rule created on behalf of the customer. Format: `//compute.googleapis.com/projects/{project}/regions/{region}/forwardingRules/{forwarding_rule_name}`
        pub psc_forwarding_rule: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the subnetwork from which the IP address will be allocated. Format: `projects/{project}/regions/{region}/subnetworks/{subnetwork}`
        pub subnetwork: pulumi_wasm_rust::Output<String>,
        /// The service endpoint this private regional endpoint connects to. Format: `{apiname}.{region}.p.rep.googleapis.com` Example: \"cloudkms.us-central1.p.rep.googleapis.com\".
        pub target_google_api: pulumi_wasm_rust::Output<String>,
        /// Time when the RegionalEndpoint was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegionalEndpointArgs,
    ) -> RegionalEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_type_binding = args.access_type.get_output(context).get_inner();
        let address_binding = args.address.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let subnetwork_binding = args.subnetwork.get_output(context).get_inner();
        let target_google_api_binding = args
            .target_google_api
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkconnectivity/regionalEndpoint:RegionalEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessType".into(),
                    value: &access_type_binding,
                },
                register_interface::ObjectField {
                    name: "address".into(),
                    value: &address_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding,
                },
                register_interface::ObjectField {
                    name: "targetGoogleApi".into(),
                    value: &target_google_api_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionalEndpointResult {
            access_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessType"),
            ),
            address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("address"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            psc_forwarding_rule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pscForwardingRule"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            target_google_api: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetGoogleApi"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
