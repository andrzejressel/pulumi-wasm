/// Provides a resource to create an association between a VPC endpoint and a subnet.
///
/// > **NOTE on VPC Endpoints and VPC Endpoint Subnet Associations:** This provider provides
/// both a standalone VPC Endpoint Subnet Association (an association between a VPC endpoint
/// and a single `subnet_id`) and a VPC Endpoint resource with a `subnet_ids`
/// attribute. Do not use the same subnet ID in both a VPC Endpoint resource and a VPC Endpoint Subnet
/// Association resource. Doing so will cause a conflict of associations and will overwrite the association.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let snEc2 = vpc_endpoint_subnet_association::create(
///         "snEc2",
///         VpcEndpointSubnetAssociationArgs::builder()
///             .subnet_id("${sn.id}")
///             .vpc_endpoint_id("${ec2.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint Subnet Associations using `vpc_endpoint_id` together with `subnet_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointSubnetAssociation:VpcEndpointSubnetAssociation example vpce-aaaaaaaa/subnet-bbbbbbbbbbbbbbbbb
/// ```
pub mod vpc_endpoint_subnet_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointSubnetAssociationArgs {
        /// The ID of the subnet to be associated with the VPC endpoint.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC endpoint with which the subnet will be associated.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointSubnetAssociationResult {
        /// The ID of the subnet to be associated with the VPC endpoint.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC endpoint with which the subnet will be associated.
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcEndpointSubnetAssociationArgs,
    ) -> VpcEndpointSubnetAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let subnet_id_binding = args.subnet_id.get_inner();
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointSubnetAssociation:VpcEndpointSubnetAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcEndpointSubnetAssociationResult {
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            vpc_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointId").unwrap(),
            ),
        }
    }
}