/// Provides a VPC DHCP Options Association resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dnsResolver = vpc_dhcp_options_association::create(
///         "dnsResolver",
///         VpcDhcpOptionsAssociationArgs::builder()
///             .dhcp_options_id("${foo.id}")
///             .vpc_id("${fooAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Remarks
///
/// * You can only associate one DHCP Options Set to a given VPC ID.
/// * Removing the DHCP Options Association automatically sets AWS's `default` DHCP Options Set to the VPC.
///
/// ## Import
///
/// Using `pulumi import`, import DHCP associations using the VPC ID associated with the options. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcDhcpOptionsAssociation:VpcDhcpOptionsAssociation imported vpc-0f001273ec18911b1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_dhcp_options_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcDhcpOptionsAssociationArgs {
        /// The ID of the DHCP Options Set to associate to the VPC.
        #[builder(into)]
        pub dhcp_options_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC to which we would like to associate a DHCP Options Set.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcDhcpOptionsAssociationResult {
        /// The ID of the DHCP Options Set to associate to the VPC.
        pub dhcp_options_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC to which we would like to associate a DHCP Options Set.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcDhcpOptionsAssociationArgs,
    ) -> VpcDhcpOptionsAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dhcp_options_id_binding = args.dhcp_options_id.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcDhcpOptionsAssociation:VpcDhcpOptionsAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dhcpOptionsId".into(),
                    value: dhcp_options_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcDhcpOptionsAssociationResult {
            dhcp_options_id: o.get_field("dhcpOptionsId"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
