/// Manages a Redis Enterprise Cluster.
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
///             .name("example-redisenterprise")
///             .build_struct(),
///     );
///     let exampleEnterpriseCluster = enterprise_cluster::create(
///         "exampleEnterpriseCluster",
///         EnterpriseClusterArgs::builder()
///             .location("${example.location}")
///             .name("example-redisenterprise")
///             .resource_group_name("${example.name}")
///             .sku_name("EnterpriseFlash_F300-3")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Redis Enterprise Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/enterpriseCluster:EnterpriseCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redisEnterprise/cluster1
/// ```
///
pub mod enterprise_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnterpriseClusterArgs {
        /// The Azure Region where the Redis Enterprise Cluster should exist. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The minimum TLS version. Possible values are `1.0`, `1.1` and `1.2`. Defaults to `1.2`. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Redis Enterprise Cluster. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Redis Enterprise Cluster should exist. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The `sku_name` is comprised of two segments separated by a hyphen (e.g. `Enterprise_E10-2`). The first segment of the `sku_name` defines the `name` of the SKU, possible values are `Enterprise_E5`, `Enterprise_E10`, `Enterprise_E20`, `Enterprise_E50`, `Enterprise_E100`, `Enterprise_E200`, `Enterprise_E400`, `EnterpriseFlash_F300`, `EnterpriseFlash_F700` or `EnterpriseFlash_F1500`. The second segment defines the `capacity` of the `sku_name`, possible values for `Enteprise` SKUs are (`2`, `4`, `6`, ...). Possible values for `EnterpriseFlash` SKUs are (`3`, `9`, `15`, ...). Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Redis Enterprise Cluster.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Redis Enterprise Cluster should be located. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct EnterpriseClusterResult {
        /// DNS name of the cluster endpoint.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Redis Enterprise Cluster should exist. Changing this forces a new Redis Enterprise Cluster to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The minimum TLS version. Possible values are `1.0`, `1.1` and `1.2`. Defaults to `1.2`. Changing this forces a new Redis Enterprise Cluster to be created.
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Redis Enterprise Cluster. Changing this forces a new Redis Enterprise Cluster to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Redis Enterprise Cluster should exist. Changing this forces a new Redis Enterprise Cluster to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The `sku_name` is comprised of two segments separated by a hyphen (e.g. `Enterprise_E10-2`). The first segment of the `sku_name` defines the `name` of the SKU, possible values are `Enterprise_E5`, `Enterprise_E10`, `Enterprise_E20`, `Enterprise_E50`, `Enterprise_E100`, `Enterprise_E200`, `Enterprise_E400`, `EnterpriseFlash_F300`, `EnterpriseFlash_F700` or `EnterpriseFlash_F1500`. The second segment defines the `capacity` of the `sku_name`, possible values for `Enteprise` SKUs are (`2`, `4`, `6`, ...). Possible values for `EnterpriseFlash` SKUs are (`3`, `9`, `15`, ...). Changing this forces a new Redis Enterprise Cluster to be created.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Redis Enterprise Cluster.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Redis Enterprise Cluster should be located. Changing this forces a new Redis Enterprise Cluster to be created.
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnterpriseClusterArgs) -> EnterpriseClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let minimum_tls_version_binding = args.minimum_tls_version.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:redis/enterpriseCluster:EnterpriseCluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "minimumTlsVersion".into(),
                    value: &minimum_tls_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "minimumTlsVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnterpriseClusterResult {
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            minimum_tls_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumTlsVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}