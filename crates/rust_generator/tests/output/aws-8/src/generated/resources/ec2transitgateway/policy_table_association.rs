/// Manages an EC2 Transit Gateway Policy Table association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = policy_table_association::create(
///         "example",
///         PolicyTableAssociationArgs::builder()
///             .transit_gateway_attachment_id(
///                 "${exampleAwsNetworkmanagerTransitGatewayPeering.transitGatewayPeeringAttachmentId}",
///             )
///             .transit_gateway_policy_table_id(
///                 "${exampleAwsEc2TransitGatewayPolicyTable.id}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_policy_table_association` using the EC2 Transit Gateway Policy Table identifier, an underscore, and the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/policyTableAssociation:PolicyTableAssociation example tgw-rtb-12345678_tgw-attach-87654321
/// ```
pub mod policy_table_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyTableAssociationArgs {
        /// Identifier of EC2 Transit Gateway Attachment.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of EC2 Transit Gateway Policy Table.
        #[builder(into)]
        pub transit_gateway_policy_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyTableAssociationResult {
        /// Identifier of the resource
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// Type of the resource
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Attachment.
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Policy Table.
        pub transit_gateway_policy_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PolicyTableAssociationArgs,
    ) -> PolicyTableAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context)
            .get_inner();
        let transit_gateway_policy_table_id_binding = args
            .transit_gateway_policy_table_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/policyTableAssociation:PolicyTableAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayPolicyTableId".into(),
                    value: &transit_gateway_policy_table_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyTableAssociationResult {
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            resource_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceType"),
            ),
            transit_gateway_attachment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayAttachmentId"),
            ),
            transit_gateway_policy_table_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayPolicyTableId"),
            ),
        }
    }
}
