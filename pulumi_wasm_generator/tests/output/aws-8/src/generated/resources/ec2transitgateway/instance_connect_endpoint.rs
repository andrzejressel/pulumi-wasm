/// Manages an EC2 Instance Connect Endpoint.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_connect_endpoint::create(
///         "example",
///         InstanceConnectEndpointArgs::builder()
///             .subnet_id("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EC2 Instance Connect Endpoints using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/instanceConnectEndpoint:InstanceConnectEndpoint example eice-012345678
/// ```
pub mod instance_connect_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceConnectEndpointArgs {
        /// Indicates whether your client's IP address is preserved as the source. Default: `true`.
        #[builder(into, default)]
        pub preserve_client_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for the VPC will be associated with the endpoint.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the subnet in which to create the EC2 Instance Connect Endpoint.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ec2transitgateway::InstanceConnectEndpointTimeouts,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceConnectEndpointResult {
        /// The Amazon Resource Name (ARN) of the EC2 Instance Connect Endpoint.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Availability Zone of the EC2 Instance Connect Endpoint.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The DNS name of the EC2 Instance Connect Endpoint.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// The DNS name of the EC2 Instance Connect FIPS Endpoint.
        pub fips_dns_name: pulumi_wasm_rust::Output<String>,
        /// The IDs of the ENIs that Amazon EC2 automatically created when creating the EC2 Instance Connect Endpoint.
        pub network_interface_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the AWS account that created the EC2 Instance Connect Endpoint.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether your client's IP address is preserved as the source. Default: `true`.
        pub preserve_client_ip: pulumi_wasm_rust::Output<bool>,
        /// One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for the VPC will be associated with the endpoint.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the subnet in which to create the EC2 Instance Connect Endpoint.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ec2transitgateway::InstanceConnectEndpointTimeouts,
            >,
        >,
        /// The ID of the VPC in which the EC2 Instance Connect Endpoint was created.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InstanceConnectEndpointArgs,
    ) -> InstanceConnectEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let preserve_client_ip_binding = args.preserve_client_ip.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/instanceConnectEndpoint:InstanceConnectEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "preserveClientIp".into(),
                    value: &preserve_client_ip_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "fipsDnsName".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceIds".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "preserveClientIp".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceConnectEndpointResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            fips_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fipsDnsName").unwrap(),
            ),
            network_interface_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceIds").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            preserve_client_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preserveClientIp").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
