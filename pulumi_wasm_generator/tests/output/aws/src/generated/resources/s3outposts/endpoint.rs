/// Provides a resource to manage an S3 Outposts Endpoint.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = endpoint::create(
///         "example",
///         EndpointArgs::builder()
///             .outpost_id("${exampleAwsOutpostsOutpost.id}")
///             .security_group_id("${exampleAwsSecurityGroup.id}")
///             .subnet_id("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Outposts Endpoints using Amazon Resource Name (ARN), EC2 Security Group identifier, and EC2 Subnet identifier, separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:s3outposts/endpoint:Endpoint example arn:aws:s3-outposts:us-east-1:123456789012:outpost/op-12345678/endpoint/0123456789abcdef,sg-12345678,subnet-12345678
/// ```
pub mod endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// Type of access for the network connectivity. Valid values are `Private` or `CustomerOwnedIp`.
        #[builder(into, default)]
        pub access_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of a Customer Owned IP Pool. For more on customer owned IP addresses see the [User Guide](https://docs.aws.amazon.com/outposts/latest/userguide/local-rack.html#local-gateway-subnet).
        #[builder(into, default)]
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the Outpost to contain this endpoint.
        #[builder(into)]
        pub outpost_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the EC2 Security Group.
        #[builder(into)]
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the EC2 Subnet.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// Type of access for the network connectivity. Valid values are `Private` or `CustomerOwnedIp`.
        pub access_type: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the endpoint.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// VPC CIDR block of the endpoint.
        pub cidr_block: pulumi_wasm_rust::Output<String>,
        /// UTC creation time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// The ID of a Customer Owned IP Pool. For more on customer owned IP addresses see the [User Guide](https://docs.aws.amazon.com/outposts/latest/userguide/local-rack.html#local-gateway-subnet).
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of nested attributes for associated Elastic Network Interfaces (ENIs).
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3outposts::EndpointNetworkInterface>,
        >,
        /// Identifier of the Outpost to contain this endpoint.
        pub outpost_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the EC2 Security Group.
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the EC2 Subnet.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EndpointArgs) -> EndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_type_binding = args.access_type.get_inner();
        let customer_owned_ipv4_pool_binding = args.customer_owned_ipv4_pool.get_inner();
        let outpost_id_binding = args.outpost_id.get_inner();
        let security_group_id_binding = args.security_group_id.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3outposts/endpoint:Endpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessType".into(),
                    value: &access_type_binding,
                },
                register_interface::ObjectField {
                    name: "customerOwnedIpv4Pool".into(),
                    value: &customer_owned_ipv4_pool_binding,
                },
                register_interface::ObjectField {
                    name: "outpostId".into(),
                    value: &outpost_id_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupId".into(),
                    value: &security_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessType".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "customerOwnedIpv4Pool".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "outpostId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EndpointResult {
            access_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessType").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlock").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            customer_owned_ipv4_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOwnedIpv4Pool").unwrap(),
            ),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaces").unwrap(),
            ),
            outpost_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostId").unwrap(),
            ),
            security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupId").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
        }
    }
}