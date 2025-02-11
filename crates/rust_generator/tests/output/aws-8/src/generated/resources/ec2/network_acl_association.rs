/// Provides an network ACL association resource which allows you to associate your network ACL with any subnet(s).
///
/// > **NOTE on Network ACLs and Network ACL Associations:** the provider provides both a standalone network ACL association resource
/// and a network ACL resource with a `subnet_ids` attribute. Do not use the same subnet ID in both a network ACL
/// resource and a network ACL association resource. Doing so will cause a conflict of associations and will overwrite the association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = network_acl_association::create(
///         "main",
///         NetworkAclAssociationArgs::builder()
///             .network_acl_id("${mainAwsNetworkAcl.id}")
///             .subnet_id("${mainAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network ACL associations using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/networkAclAssociation:NetworkAclAssociation main aclassoc-02baf37f20966b3e6
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_acl_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkAclAssociationArgs {
        /// The ID of the network ACL.
        #[builder(into)]
        pub network_acl_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the associated Subnet.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkAclAssociationResult {
        /// The ID of the network ACL.
        pub network_acl_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the associated Subnet.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkAclAssociationArgs,
    ) -> NetworkAclAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let network_acl_id_binding = args.network_acl_id.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/networkAclAssociation:NetworkAclAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkAclId".into(),
                    value: &network_acl_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkAclAssociationResult {
            network_acl_id: o.get_field("networkAclId"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}
