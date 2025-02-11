/// Provides a resource to create an association between a VPC endpoint and a security group.
///
/// > **NOTE on VPC Endpoints and VPC Endpoint Security Group Associations:** The provider provides
/// both a standalone VPC Endpoint Security Group Association (an association between a VPC endpoint
/// and a single `security_group_id`) and a VPC Endpoint resource with a `security_group_ids`
/// attribute. Do not use the same security group ID in both a VPC Endpoint resource and a VPC Endpoint Security
/// Group Association resource. Doing so will cause a conflict of associations and will overwrite the association.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sgEc2 = security_group_association::create(
///         "sgEc2",
///         SecurityGroupAssociationArgs::builder()
///             .security_group_id("${sg.id}")
///             .vpc_endpoint_id("${ec2.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_group_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityGroupAssociationArgs {
        /// Whether this association should replace the association with the VPC's default security group that is created when no security groups are specified during VPC endpoint creation. At most 1 association per-VPC endpoint should be configured with `replace_default_association = true`.
        #[builder(into, default)]
        pub replace_default_association: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the security group to be associated with the VPC endpoint.
        #[builder(into)]
        pub security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC endpoint with which the security group will be associated.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecurityGroupAssociationResult {
        /// Whether this association should replace the association with the VPC's default security group that is created when no security groups are specified during VPC endpoint creation. At most 1 association per-VPC endpoint should be configured with `replace_default_association = true`.
        pub replace_default_association: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the security group to be associated with the VPC endpoint.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC endpoint with which the security group will be associated.
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityGroupAssociationArgs,
    ) -> SecurityGroupAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let replace_default_association_binding = args
            .replace_default_association
            .get_output(context);
        let security_group_id_binding = args.security_group_id.get_output(context);
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/securityGroupAssociation:SecurityGroupAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replaceDefaultAssociation".into(),
                    value: &replace_default_association_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupId".into(),
                    value: &security_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityGroupAssociationResult {
            replace_default_association: o.get_field("replaceDefaultAssociation"),
            security_group_id: o.get_field("securityGroupId"),
            vpc_endpoint_id: o.get_field("vpcEndpointId"),
        }
    }
}
