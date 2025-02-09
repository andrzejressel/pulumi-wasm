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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceConnectEndpointArgs,
    ) -> InstanceConnectEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let preserve_client_ip_binding_1 = args.preserve_client_ip.get_output(context);
        let preserve_client_ip_binding = preserve_client_ip_binding_1.get_inner();
        let security_group_ids_binding_1 = args.security_group_ids.get_output(context);
        let security_group_ids_binding = security_group_ids_binding_1.get_inner();
        let subnet_id_binding_1 = args.subnet_id.get_output(context);
        let subnet_id_binding = subnet_id_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceConnectEndpointResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            fips_dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fipsDnsName"),
            ),
            network_interface_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceIds"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            preserve_client_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preserveClientIp"),
            ),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
