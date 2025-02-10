/// Manages an EC2 Instance Connect Endpoint.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_connect_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceConnectEndpointArgs {
        /// Indicates whether your client's IP address is preserved as the source. Default: `true`.
        #[builder(into, default)]
        pub preserve_client_ip: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for the VPC will be associated with the endpoint.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the subnet in which to create the EC2 Instance Connect Endpoint.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ec2transitgateway::InstanceConnectEndpointTimeouts,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceConnectEndpointResult {
        /// The Amazon Resource Name (ARN) of the EC2 Instance Connect Endpoint.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Availability Zone of the EC2 Instance Connect Endpoint.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The DNS name of the EC2 Instance Connect Endpoint.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// The DNS name of the EC2 Instance Connect FIPS Endpoint.
        pub fips_dns_name: pulumi_gestalt_rust::Output<String>,
        /// The IDs of the ENIs that Amazon EC2 automatically created when creating the EC2 Instance Connect Endpoint.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the AWS account that created the EC2 Instance Connect Endpoint.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether your client's IP address is preserved as the source. Default: `true`.
        pub preserve_client_ip: pulumi_gestalt_rust::Output<bool>,
        /// One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for the VPC will be associated with the endpoint.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the subnet in which to create the EC2 Instance Connect Endpoint.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::ec2transitgateway::InstanceConnectEndpointTimeouts,
            >,
        >,
        /// The ID of the VPC in which the EC2 Instance Connect Endpoint was created.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceConnectEndpointArgs,
    ) -> InstanceConnectEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let preserve_client_ip_binding = args.preserve_client_ip.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/instanceConnectEndpoint:InstanceConnectEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preserveClientIp".into(),
                    value: preserve_client_ip_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: security_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceConnectEndpointResult {
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            dns_name: o.get_field("dnsName"),
            fips_dns_name: o.get_field("fipsDnsName"),
            network_interface_ids: o.get_field("networkInterfaceIds"),
            owner_id: o.get_field("ownerId"),
            preserve_client_ip: o.get_field("preserveClientIp"),
            security_group_ids: o.get_field("securityGroupIds"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
