/// Subnet in a private cloud. A Private Cloud contains two types of subnets: `management` subnets (such as vMotion) that
/// are read-only,and `userDefined`, which can also be updated. This resource should be used to read and update `userDefined`
/// subnets. To read `management` subnets, please utilize the subnet data source.
///
///
/// To get more information about Subnet, see:
///
/// * [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds.subnets)
///
/// ## Example Usage
///
/// ### Vmware Engine Subnet User Defined
///
///
/// ```yaml
/// resources:
///   subnet-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: pc-nw
///       location: global
///       type: STANDARD
///       description: PC network description.
///   subnet-pc:
///     type: gcp:vmwareengine:PrivateCloud
///     properties:
///       location: us-west1-a
///       name: sample-pc
///       description: Sample test PC.
///       networkConfig:
///         managementCidr: 192.168.50.0/24
///         vmwareEngineNetwork: ${["subnet-nw"].id}
///       managementCluster:
///         clusterId: sample-mgmt-cluster
///         nodeTypeConfigs:
///           - nodeTypeId: standard-72
///             nodeCount: 3
///   vmw-engine-subnet:
///     type: gcp:vmwareengine:Subnet
///     properties:
///       name: service-1
///       parent: ${["subnet-pc"].id}
///       ipCidrRange: 192.168.100.0/26
/// ```
///
/// ## Import
///
/// Subnet can be imported using any of these accepted formats:
///
/// * `{{parent}}/subnets/{{name}}`
///
/// When using the `pulumi import` command, Subnet can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/subnet:Subnet default {{parent}}/subnets/{{name}}
/// ```
///
pub mod subnet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetArgs {
        /// The IP address range of the subnet in CIDR format.
        #[builder(into)]
        pub ip_cidr_range: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the subnet. For userDefined subnets, this name should be in the format of "service-n",
        /// where n ranges from 1 to 5.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource name of the private cloud to create a new subnet in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetResult {
        /// Creation time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// DHCP address ranges.
        /// Structure is documented below.
        pub dhcp_address_ranges: pulumi_wasm_rust::Output<
            Vec<super::super::types::vmwareengine::SubnetDhcpAddressRange>,
        >,
        /// The canonical identifier of the logical router that this subnet is attached to.
        pub gateway_id: pulumi_wasm_rust::Output<String>,
        /// The IP address of the gateway of this subnet. Must fall within the IP prefix defined above.
        pub gateway_ip: pulumi_wasm_rust::Output<String>,
        /// The IP address range of the subnet in CIDR format.
        pub ip_cidr_range: pulumi_wasm_rust::Output<String>,
        /// The ID of the subnet. For userDefined subnets, this name should be in the format of "service-n",
        /// where n ranges from 1 to 5.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource name of the private cloud to create a new subnet in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Whether the NSX-T configuration in the backend follows the standard configuration supported by Google Cloud.
        /// If false, the subnet cannot be modified through Google Cloud, only through NSX-T directly.
        pub standard_config: pulumi_wasm_rust::Output<bool>,
        /// State of the subnet.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The type of the subnet.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Last updated time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// VLAN ID of the VLAN on which the subnet is configured.
        pub vlan_id: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SubnetArgs,
    ) -> SubnetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ip_cidr_range_binding = args.ip_cidr_range.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vmwareengine/subnet:Subnet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ipCidrRange".into(),
                    value: &ip_cidr_range_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dhcpAddressRanges".into(),
                },
                register_interface::ResultField {
                    name: "gatewayId".into(),
                },
                register_interface::ResultField {
                    name: "gatewayIp".into(),
                },
                register_interface::ResultField {
                    name: "ipCidrRange".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "standardConfig".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "vlanId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubnetResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dhcp_address_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dhcpAddressRanges").unwrap(),
            ),
            gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayId").unwrap(),
            ),
            gateway_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayIp").unwrap(),
            ),
            ip_cidr_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipCidrRange").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            standard_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("standardConfig").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            vlan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vlanId").unwrap(),
            ),
        }
    }
}
