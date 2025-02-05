/// Associates a Direct Connect Gateway with a VGW or transit gateway.
///
/// To create a cross-account association, create an `aws.directconnect.GatewayAssociationProposal` resource
/// in the AWS account that owns the VGW or transit gateway and then accept the proposal in the AWS account that owns the Direct Connect Gateway
/// by creating an `aws.directconnect.GatewayAssociation` resource with the `proposal_id` and `associated_gateway_owner_account_id` attributes set.
///
/// ## Example Usage
///
/// ### VPN Gateway Association
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway::create(
///         "example",
///         GatewayArgs::builder().amazon_side_asn("64512").name("example").build_struct(),
///     );
///     let exampleGatewayAssociation = gateway_association::create(
///         "exampleGatewayAssociation",
///         GatewayAssociationArgs::builder()
///             .associated_gateway_id("${exampleVpnGateway.id}")
///             .dx_gateway_id("${example.id}")
///             .build_struct(),
///     );
///     let exampleVpc = vpc::create(
///         "exampleVpc",
///         VpcArgs::builder().cidr_block("10.255.255.0/28").build_struct(),
///     );
///     let exampleVpnGateway = vpn_gateway::create(
///         "exampleVpnGateway",
///         VpnGatewayArgs::builder().vpc_id("${exampleVpc.id}").build_struct(),
///     );
/// }
/// ```
///
/// ### Transit Gateway Association
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway::create(
///         "example",
///         GatewayArgs::builder().amazon_side_asn("64512").name("example").build_struct(),
///     );
///     let exampleGatewayAssociation = gateway_association::create(
///         "exampleGatewayAssociation",
///         GatewayAssociationArgs::builder()
///             .allowed_prefixes(vec!["10.255.255.0/30", "10.255.255.8/30",])
///             .associated_gateway_id("${exampleTransitGateway.id}")
///             .dx_gateway_id("${example.id}")
///             .build_struct(),
///     );
///     let exampleTransitGateway = transit_gateway::create(
///         "exampleTransitGateway",
///         TransitGatewayArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ### Allowed Prefixes
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway::create(
///         "example",
///         GatewayArgs::builder().amazon_side_asn("64512").name("example").build_struct(),
///     );
///     let exampleGatewayAssociation = gateway_association::create(
///         "exampleGatewayAssociation",
///         GatewayAssociationArgs::builder()
///             .allowed_prefixes(vec!["210.52.109.0/24", "175.45.176.0/22",])
///             .associated_gateway_id("${exampleVpnGateway.id}")
///             .dx_gateway_id("${example.id}")
///             .build_struct(),
///     );
///     let exampleVpc = vpc::create(
///         "exampleVpc",
///         VpcArgs::builder().cidr_block("10.255.255.0/28").build_struct(),
///     );
///     let exampleVpnGateway = vpn_gateway::create(
///         "exampleVpnGateway",
///         VpnGatewayArgs::builder().vpc_id("${exampleVpc.id}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect gateway associations using `dx_gateway_id` together with `associated_gateway_id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/gatewayAssociation:GatewayAssociation example 345508c3-7215-4aef-9832-07c125d5bd0f/vgw-98765432
/// ```
pub mod gateway_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayAssociationArgs {
        /// VPC prefixes (CIDRs) to advertise to the Direct Connect gateway. Defaults to the CIDR block of the VPC associated with the Virtual Gateway. To enable drift detection, must be configured.
        #[builder(into, default)]
        pub allowed_prefixes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the VGW or transit gateway with which to associate the Direct Connect gateway.
        /// Used for single account Direct Connect gateway associations.
        #[builder(into, default)]
        pub associated_gateway_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the AWS account that owns the VGW or transit gateway with which to associate the Direct Connect gateway.
        /// Used for cross-account Direct Connect gateway associations.
        #[builder(into, default)]
        pub associated_gateway_owner_account_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Direct Connect gateway.
        #[builder(into)]
        pub dx_gateway_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Direct Connect gateway association proposal.
        /// Used for cross-account Direct Connect gateway associations.
        #[builder(into, default)]
        pub proposal_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub vpn_gateway_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GatewayAssociationResult {
        /// VPC prefixes (CIDRs) to advertise to the Direct Connect gateway. Defaults to the CIDR block of the VPC associated with the Virtual Gateway. To enable drift detection, must be configured.
        pub allowed_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the VGW or transit gateway with which to associate the Direct Connect gateway.
        /// Used for single account Direct Connect gateway associations.
        pub associated_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the VGW or transit gateway with which to associate the Direct Connect gateway.
        /// Used for cross-account Direct Connect gateway associations.
        pub associated_gateway_owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The type of the associated gateway, `transitGateway` or `virtualPrivateGateway`.
        pub associated_gateway_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the Direct Connect gateway association.
        pub dx_gateway_association_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Direct Connect gateway.
        pub dx_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the Direct Connect gateway.
        pub dx_gateway_owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Direct Connect gateway association proposal.
        /// Used for cross-account Direct Connect gateway associations.
        pub proposal_id: pulumi_wasm_rust::Output<Option<String>>,
        pub vpn_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GatewayAssociationArgs,
    ) -> GatewayAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowed_prefixes_binding = args
            .allowed_prefixes
            .get_output(context)
            .get_inner();
        let associated_gateway_id_binding = args
            .associated_gateway_id
            .get_output(context)
            .get_inner();
        let associated_gateway_owner_account_id_binding = args
            .associated_gateway_owner_account_id
            .get_output(context)
            .get_inner();
        let dx_gateway_id_binding = args.dx_gateway_id.get_output(context).get_inner();
        let proposal_id_binding = args.proposal_id.get_output(context).get_inner();
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/gatewayAssociation:GatewayAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedPrefixes".into(),
                    value: &allowed_prefixes_binding,
                },
                register_interface::ObjectField {
                    name: "associatedGatewayId".into(),
                    value: &associated_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "associatedGatewayOwnerAccountId".into(),
                    value: &associated_gateway_owner_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "dxGatewayId".into(),
                    value: &dx_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "proposalId".into(),
                    value: &proposal_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpnGatewayId".into(),
                    value: &vpn_gateway_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GatewayAssociationResult {
            allowed_prefixes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowedPrefixes"),
            ),
            associated_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associatedGatewayId"),
            ),
            associated_gateway_owner_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associatedGatewayOwnerAccountId"),
            ),
            associated_gateway_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associatedGatewayType"),
            ),
            dx_gateway_association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dxGatewayAssociationId"),
            ),
            dx_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dxGatewayId"),
            ),
            dx_gateway_owner_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dxGatewayOwnerAccountId"),
            ),
            proposal_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("proposalId"),
            ),
            vpn_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpnGatewayId"),
            ),
        }
    }
}
