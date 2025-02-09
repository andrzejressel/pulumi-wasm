/// Manages an individual EC2 resource tag. This resource should only be used in cases where EC2 resources are created outside the provider (e.g. AMIs), being shared via Resource Access Manager (RAM), or implicitly created by other means (e.g. Transit Gateway VPN Attachments).
///
/// > **NOTE:** This tagging resource should not be combined with the providers resource for managing the parent resource. For example, using `aws.ec2.Vpc` and `aws.ec2.Tag` to manage tags of the same VPC will cause a perpetual difference where the `aws.ec2.Vpc` resource will try to remove the tag being added by the `aws.ec2.Tag` resource.
///
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2transitgateway:TransitGateway
///   exampleCustomerGateway:
///     type: aws:ec2:CustomerGateway
///     name: example
///     properties:
///       bgpAsn: 65000
///       ipAddress: 172.0.0.1
///       type: ipsec.1
///   exampleVpnConnection:
///     type: aws:ec2:VpnConnection
///     name: example
///     properties:
///       customerGatewayId: ${exampleCustomerGateway.id}
///       transitGatewayId: ${example.id}
///       type: ${exampleCustomerGateway.type}
///   exampleTag:
///     type: aws:ec2:Tag
///     name: example
///     properties:
///       resourceId: ${exampleVpnConnection.transitGatewayAttachmentId}
///       key: Name
///       value: Hello World
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_tag` using the EC2 resource identifier and key, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ec2/tag:Tag example tgw-attach-1234567890abcdef,Name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagArgs {
        /// The tag name.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the EC2 resource to manage the tag for.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The value of the tag.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagResult {
        /// The tag name.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the EC2 resource to manage the tag for.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// The value of the tag.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TagArgs,
    ) -> TagResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_binding_1 = args.key.get_output(context);
        let key_binding = key_binding_1.get_inner();
        let resource_id_binding_1 = args.resource_id.get_output(context);
        let resource_id_binding = resource_id_binding_1.get_inner();
        let value_binding_1 = args.value.get_output(context);
        let value_binding = value_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/tag:Tag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TagResult {
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
