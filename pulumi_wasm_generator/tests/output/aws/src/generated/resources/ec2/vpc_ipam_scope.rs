/// Creates a scope for AWS IPAM.
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
///     let current = get_region::invoke(GetRegionArgs::builder().build_struct());
///     let example = vpc_ipam::create(
///         "example",
///         VpcIpamArgs::builder()
///             .operating_regions(
///                 vec![
///                     VpcIpamOperatingRegion::builder().regionName("${current.name}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleVpcIpamScope = vpc_ipam_scope::create(
///         "exampleVpcIpamScope",
///         VpcIpamScopeArgs::builder()
///             .description("Another Scope")
///             .ipam_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the `scope_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamScope:VpcIpamScope example ipam-scope-0513c69f283d11dfb
/// ```
pub mod vpc_ipam_scope {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamScopeArgs {
        /// A description for the scope you're creating.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the IPAM for which you're creating this scope.
        #[builder(into)]
        pub ipam_id: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcIpamScopeResult {
        /// The Amazon Resource Name (ARN) of the scope.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A description for the scope you're creating.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the IPAM for which you're creating this scope.
        pub ipam_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the IPAM for which you're creating this scope.
        pub ipam_id: pulumi_wasm_rust::Output<String>,
        pub ipam_scope_type: pulumi_wasm_rust::Output<String>,
        /// Defines if the scope is the default scope or not.
        pub is_default: pulumi_wasm_rust::Output<bool>,
        /// The number of pools in the scope.
        pub pool_count: pulumi_wasm_rust::Output<i32>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcIpamScopeArgs) -> VpcIpamScopeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let ipam_id_binding = args.ipam_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamScope:VpcIpamScope".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ipamId".into(),
                    value: &ipam_id_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "ipamArn".into(),
                },
                register_interface::ResultField {
                    name: "ipamId".into(),
                },
                register_interface::ResultField {
                    name: "ipamScopeType".into(),
                },
                register_interface::ResultField {
                    name: "isDefault".into(),
                },
                register_interface::ResultField {
                    name: "poolCount".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcIpamScopeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            ipam_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamArn").unwrap(),
            ),
            ipam_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamId").unwrap(),
            ),
            ipam_scope_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamScopeType").unwrap(),
            ),
            is_default: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDefault").unwrap(),
            ),
            pool_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolCount").unwrap(),
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
