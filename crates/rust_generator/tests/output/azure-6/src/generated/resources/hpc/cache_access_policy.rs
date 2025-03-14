/// Manages a HPC Cache Access Policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cache_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheAccessPolicyArgs {
        /// One or more `access_rule` blocks (up to three) as defined below.
        #[builder(into)]
        pub access_rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::hpc::CacheAccessPolicyAccessRule>,
        >,
        /// The ID of the HPC Cache that this HPC Cache Access Policy resides in. Changing this forces a new HPC Cache Access Policy to be created.
        #[builder(into)]
        pub hpc_cache_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this HPC Cache Access Policy. Changing this forces a new HPC Cache Access Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CacheAccessPolicyResult {
        /// One or more `access_rule` blocks (up to three) as defined below.
        pub access_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::hpc::CacheAccessPolicyAccessRule>,
        >,
        /// The ID of the HPC Cache that this HPC Cache Access Policy resides in. Changing this forces a new HPC Cache Access Policy to be created.
        pub hpc_cache_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this HPC Cache Access Policy. Changing this forces a new HPC Cache Access Policy to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CacheAccessPolicyArgs,
    ) -> CacheAccessPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_rules_binding = args.access_rules.get_output(context);
        let hpc_cache_id_binding = args.hpc_cache_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:hpc/cacheAccessPolicy:CacheAccessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessRules".into(),
                    value: &access_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hpcCacheId".into(),
                    value: &hpc_cache_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CacheAccessPolicyResult {
            access_rules: o.get_field("accessRules"),
            hpc_cache_id: o.get_field("hpcCacheId"),
            name: o.get_field("name"),
        }
    }
}
