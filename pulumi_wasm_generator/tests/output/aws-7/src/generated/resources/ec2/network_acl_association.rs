/// Provides an network ACL association resource which allows you to associate your network ACL with any subnet(s).
///
/// > **NOTE on Network ACLs and Network ACL Associations:** the provider provides both a standalone network ACL association resource
/// and a network ACL resource with a `subnet_ids` attribute. Do not use the same subnet ID in both a network ACL
/// resource and a network ACL association resource. Doing so will cause a conflict of associations and will overwrite the association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod network_acl_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkAclAssociationArgs {
        /// The ID of the network ACL.
        #[builder(into)]
        pub network_acl_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the associated Subnet.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkAclAssociationResult {
        /// The ID of the network ACL.
        pub network_acl_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the associated Subnet.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkAclAssociationArgs,
    ) -> NetworkAclAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let network_acl_id_binding = args.network_acl_id.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/networkAclAssociation:NetworkAclAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "networkAclId".into(),
                    value: &network_acl_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "networkAclId".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkAclAssociationResult {
            network_acl_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkAclId").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
        }
    }
}
