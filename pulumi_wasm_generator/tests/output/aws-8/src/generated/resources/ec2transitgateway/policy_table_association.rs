/// Manages an EC2 Transit Gateway Policy Table association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyTableAssociationArgs {
        /// Identifier of EC2 Transit Gateway Attachment.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Policy Table.
        #[builder(into)]
        pub transit_gateway_policy_table_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyTableAssociationResult {
        /// Identifier of the resource
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// Type of the resource
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Attachment.
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Policy Table.
        pub transit_gateway_policy_table_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PolicyTableAssociationArgs,
    ) -> PolicyTableAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_inner();
        let transit_gateway_policy_table_id_binding = args
            .transit_gateway_policy_table_id
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayPolicyTableId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyTableAssociationResult {
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
            transit_gateway_policy_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayPolicyTableId").unwrap(),
            ),
        }
    }
}
