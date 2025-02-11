/// Provides a resource to manage an S3 Outposts Endpoint.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// Type of access for the network connectivity. Valid values are `Private` or `CustomerOwnedIp`.
        #[builder(into, default)]
        pub access_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of a Customer Owned IP Pool. For more on customer owned IP addresses see the [User Guide](https://docs.aws.amazon.com/outposts/latest/userguide/local-rack.html#local-gateway-subnet).
        #[builder(into, default)]
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the Outpost to contain this endpoint.
        #[builder(into)]
        pub outpost_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the EC2 Security Group.
        #[builder(into)]
        pub security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the EC2 Subnet.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// Type of access for the network connectivity. Valid values are `Private` or `CustomerOwnedIp`.
        pub access_type: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the endpoint.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// VPC CIDR block of the endpoint.
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        /// UTC creation time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// The ID of a Customer Owned IP Pool. For more on customer owned IP addresses see the [User Guide](https://docs.aws.amazon.com/outposts/latest/userguide/local-rack.html#local-gateway-subnet).
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of nested attributes for associated Elastic Network Interfaces (ENIs).
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::types::s3outposts::EndpointNetworkInterface>,
        >,
        /// Identifier of the Outpost to contain this endpoint.
        pub outpost_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the EC2 Security Group.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the EC2 Subnet.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointArgs,
    ) -> EndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_type_binding = args.access_type.get_output(context);
        let customer_owned_ipv4_pool_binding = args
            .customer_owned_ipv4_pool
            .get_output(context);
        let outpost_id_binding = args.outpost_id.get_output(context);
        let security_group_id_binding = args.security_group_id.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3outposts/endpoint:Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessType".into(),
                    value: &access_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerOwnedIpv4Pool".into(),
                    value: &customer_owned_ipv4_pool_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outpostId".into(),
                    value: &outpost_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupId".into(),
                    value: &security_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointResult {
            access_type: o.get_field("accessType"),
            arn: o.get_field("arn"),
            cidr_block: o.get_field("cidrBlock"),
            creation_time: o.get_field("creationTime"),
            customer_owned_ipv4_pool: o.get_field("customerOwnedIpv4Pool"),
            network_interfaces: o.get_field("networkInterfaces"),
            outpost_id: o.get_field("outpostId"),
            security_group_id: o.get_field("securityGroupId"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}
