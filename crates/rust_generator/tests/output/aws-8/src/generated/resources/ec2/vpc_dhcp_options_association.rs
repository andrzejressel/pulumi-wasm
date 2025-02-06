/// Provides a VPC DHCP Options Association resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod vpc_dhcp_options_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcDhcpOptionsAssociationArgs {
        /// The ID of the DHCP Options Set to associate to the VPC.
        #[builder(into)]
        pub dhcp_options_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the VPC to which we would like to associate a DHCP Options Set.
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcDhcpOptionsAssociationResult {
        /// The ID of the DHCP Options Set to associate to the VPC.
        pub dhcp_options_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC to which we would like to associate a DHCP Options Set.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpcDhcpOptionsAssociationArgs,
    ) -> VpcDhcpOptionsAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dhcp_options_id_binding = args
            .dhcp_options_id
            .get_output(context)
            .get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcDhcpOptionsAssociation:VpcDhcpOptionsAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dhcpOptionsId".into(),
                    value: &dhcp_options_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcDhcpOptionsAssociationResult {
            dhcp_options_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dhcpOptionsId"),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
