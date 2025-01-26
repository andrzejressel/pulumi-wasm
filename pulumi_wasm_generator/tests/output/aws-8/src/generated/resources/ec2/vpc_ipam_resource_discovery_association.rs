/// Provides an association between an Amazon IP Address Manager (IPAM) and a IPAM Resource Discovery. IPAM Resource Discoveries are resources meant for multi-organization customers. If you wish to use a single IPAM across multiple orgs, a resource discovery can be created and shared from a subordinate organization to the management organizations IPAM delegated admin account.
///
/// Once an association is created between two organizations via IPAM & a IPAM Resource Discovery, IPAM Pools can be shared via Resource Access Manager (RAM) to accounts in the subordinate organization; these RAM shares must be accepted by the end user account. Pools can then also discover and monitor IPAM resources in the subordinate organization.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   test:
///     type: aws:ec2:VpcIpamResourceDiscoveryAssociation
///     properties:
///       ipamId: ${testAwsVpcIpam.id}
///       ipamResourceDiscoveryId: ${testAwsVpcIpamResourceDiscovery.id}
///       tags:
///         Name: test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the IPAM resource discovery association `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamResourceDiscoveryAssociation:VpcIpamResourceDiscoveryAssociation example ipam-res-disco-assoc-0178368ad2146a492
/// ```
pub mod vpc_ipam_resource_discovery_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamResourceDiscoveryAssociationArgs {
        /// The ID of the IPAM to associate.
        #[builder(into)]
        pub ipam_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Resource Discovery to associate.
        #[builder(into)]
        pub ipam_resource_discovery_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to add to the IPAM resource discovery association resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcIpamResourceDiscoveryAssociationResult {
        /// The Amazon Resource Name (ARN) of IPAM Resource Discovery Association.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IPAM.
        pub ipam_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the IPAM to associate.
        pub ipam_id: pulumi_wasm_rust::Output<String>,
        /// The home region of the IPAM.
        pub ipam_region: pulumi_wasm_rust::Output<String>,
        /// The ID of the Resource Discovery to associate.
        pub ipam_resource_discovery_id: pulumi_wasm_rust::Output<String>,
        /// A boolean to identify if the Resource Discovery is the accounts default resource discovery.
        pub is_default: pulumi_wasm_rust::Output<bool>,
        /// The account ID for the account that manages the Resource Discovery
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// The lifecycle state of the association when you associate or disassociate a resource discovery.
        pub state: pulumi_wasm_rust::Output<String>,
        /// A map of tags to add to the IPAM resource discovery association resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpcIpamResourceDiscoveryAssociationArgs,
    ) -> VpcIpamResourceDiscoveryAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ipam_id_binding = args.ipam_id.get_output(context).get_inner();
        let ipam_resource_discovery_id_binding = args
            .ipam_resource_discovery_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamResourceDiscoveryAssociation:VpcIpamResourceDiscoveryAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ipamId".into(),
                    value: &ipam_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipamResourceDiscoveryId".into(),
                    value: &ipam_resource_discovery_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "ipamArn".into(),
                },
                register_interface::ResultField {
                    name: "ipamId".into(),
                },
                register_interface::ResultField {
                    name: "ipamRegion".into(),
                },
                register_interface::ResultField {
                    name: "ipamResourceDiscoveryId".into(),
                },
                register_interface::ResultField {
                    name: "isDefault".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcIpamResourceDiscoveryAssociationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            ipam_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamArn").unwrap(),
            ),
            ipam_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamId").unwrap(),
            ),
            ipam_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamRegion").unwrap(),
            ),
            ipam_resource_discovery_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamResourceDiscoveryId").unwrap(),
            ),
            is_default: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDefault").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
