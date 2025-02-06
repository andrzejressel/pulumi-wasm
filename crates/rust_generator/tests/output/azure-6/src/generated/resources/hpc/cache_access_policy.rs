/// Manages a HPC Cache Access Policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleCache = cache::create(
///         "exampleCache",
///         CacheArgs::builder()
///             .cache_size_in_gb(3072)
///             .location("${example.location}")
///             .name("examplehpccache")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard_2G")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
///     let exampleCacheAccessPolicy = cache_access_policy::create(
///         "exampleCacheAccessPolicy",
///         CacheAccessPolicyArgs::builder()
///             .access_rules(
///                 vec![
///                     CacheAccessPolicyAccessRule::builder().access("rw").scope("default")
///                     .build_struct(),
///                 ],
///             )
///             .hpc_cache_id("${exampleCache.id}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .name("examplesubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("examplevn")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// HPC Cache Access Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:hpc/cacheAccessPolicy:CacheAccessPolicy example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.StorageCache/caches/cache1/cacheAccessPolicies/policy1
/// ```
///
pub mod cache_access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheAccessPolicyArgs {
        /// One or more `access_rule` blocks (up to three) as defined below.
        #[builder(into)]
        pub access_rules: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::hpc::CacheAccessPolicyAccessRule>,
        >,
        /// The ID of the HPC Cache that this HPC Cache Access Policy resides in. Changing this forces a new HPC Cache Access Policy to be created.
        #[builder(into)]
        pub hpc_cache_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this HPC Cache Access Policy. Changing this forces a new HPC Cache Access Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CacheAccessPolicyResult {
        /// One or more `access_rule` blocks (up to three) as defined below.
        pub access_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::hpc::CacheAccessPolicyAccessRule>,
        >,
        /// The ID of the HPC Cache that this HPC Cache Access Policy resides in. Changing this forces a new HPC Cache Access Policy to be created.
        pub hpc_cache_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this HPC Cache Access Policy. Changing this forces a new HPC Cache Access Policy to be created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CacheAccessPolicyArgs,
    ) -> CacheAccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_rules_binding = args.access_rules.get_output(context).get_inner();
        let hpc_cache_id_binding = args.hpc_cache_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:hpc/cacheAccessPolicy:CacheAccessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessRules".into(),
                    value: &access_rules_binding,
                },
                register_interface::ObjectField {
                    name: "hpcCacheId".into(),
                    value: &hpc_cache_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CacheAccessPolicyResult {
            access_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessRules"),
            ),
            hpc_cache_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hpcCacheId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
