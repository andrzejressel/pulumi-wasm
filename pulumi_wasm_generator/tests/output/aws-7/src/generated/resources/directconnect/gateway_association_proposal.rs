/// Manages a Direct Connect Gateway Association Proposal, typically for enabling cross-account associations. For single account associations, see the `aws.directconnect.GatewayAssociation` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway_association_proposal::create(
///         "example",
///         GatewayAssociationProposalArgs::builder()
///             .associated_gateway_id("${exampleAwsVpnGateway.id}")
///             .dx_gateway_id("${exampleAwsDxGateway.id}")
///             .dx_gateway_owner_account_id("${exampleAwsDxGateway.ownerAccountId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using a proposal ID, Direct Connect Gateway ID and associated gateway ID separated by `/`:
///
/// __With `pulumi import`__, import Direct Connect Gateway Association Proposals using either a proposal ID or proposal ID, Direct Connect Gateway ID and associated gateway ID separated by `/`. For example:
///
/// Using a proposal ID:
///
/// ```sh
/// $ pulumi import aws:directconnect/gatewayAssociationProposal:GatewayAssociationProposal example ac90e981-b718-4364-872d-65478c84fafe
/// ```
/// Using a proposal ID, Direct Connect Gateway ID and associated gateway ID separated by `/`:
///
/// ```sh
/// $ pulumi import aws:directconnect/gatewayAssociationProposal:GatewayAssociationProposal example ac90e981-b718-4364-872d-65478c84fafe/abcd1234-dcba-5678-be23-cdef9876ab45/vgw-12345678
/// ```
/// The latter case is useful when a previous proposal has been accepted and deleted by AWS.
/// The `aws_dx_gateway_association_proposal` resource will then represent a pseudo-proposal for the same Direct Connect Gateway and associated gateway. If no previous proposal is available, use a tool like [`uuidgen`](http://manpages.ubuntu.com/manpages/bionic/man1/uuidgen.1.html) to generate a new random pseudo-proposal ID.
///
pub mod gateway_association_proposal {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayAssociationProposalArgs {
        /// VPC prefixes (CIDRs) to advertise to the Direct Connect gateway. Defaults to the CIDR block of the VPC associated with the Virtual Gateway. To enable drift detection, must be configured.
        #[builder(into, default)]
        pub allowed_prefixes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the VGW or transit gateway with which to associate the Direct Connect gateway.
        #[builder(into)]
        pub associated_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Direct Connect Gateway identifier.
        #[builder(into)]
        pub dx_gateway_id: pulumi_wasm_rust::Output<String>,
        /// AWS Account identifier of the Direct Connect Gateway's owner.
        #[builder(into)]
        pub dx_gateway_owner_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GatewayAssociationProposalResult {
        /// VPC prefixes (CIDRs) to advertise to the Direct Connect gateway. Defaults to the CIDR block of the VPC associated with the Virtual Gateway. To enable drift detection, must be configured.
        pub allowed_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the VGW or transit gateway with which to associate the Direct Connect gateway.
        pub associated_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the VGW or transit gateway with which to associate the Direct Connect gateway.
        pub associated_gateway_owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The type of the associated gateway, `transitGateway` or `virtualPrivateGateway`.
        pub associated_gateway_type: pulumi_wasm_rust::Output<String>,
        /// Direct Connect Gateway identifier.
        pub dx_gateway_id: pulumi_wasm_rust::Output<String>,
        /// AWS Account identifier of the Direct Connect Gateway's owner.
        pub dx_gateway_owner_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: GatewayAssociationProposalArgs,
    ) -> GatewayAssociationProposalResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowed_prefixes_binding = args.allowed_prefixes.get_inner();
        let associated_gateway_id_binding = args.associated_gateway_id.get_inner();
        let dx_gateway_id_binding = args.dx_gateway_id.get_inner();
        let dx_gateway_owner_account_id_binding = args
            .dx_gateway_owner_account_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/gatewayAssociationProposal:GatewayAssociationProposal"
                .into(),
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
                    name: "dxGatewayId".into(),
                    value: &dx_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "dxGatewayOwnerAccountId".into(),
                    value: &dx_gateway_owner_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowedPrefixes".into(),
                },
                register_interface::ResultField {
                    name: "associatedGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "associatedGatewayOwnerAccountId".into(),
                },
                register_interface::ResultField {
                    name: "associatedGatewayType".into(),
                },
                register_interface::ResultField {
                    name: "dxGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "dxGatewayOwnerAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GatewayAssociationProposalResult {
            allowed_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedPrefixes").unwrap(),
            ),
            associated_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associatedGatewayId").unwrap(),
            ),
            associated_gateway_owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associatedGatewayOwnerAccountId").unwrap(),
            ),
            associated_gateway_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associatedGatewayType").unwrap(),
            ),
            dx_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dxGatewayId").unwrap(),
            ),
            dx_gateway_owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dxGatewayOwnerAccountId").unwrap(),
            ),
        }
    }
}
