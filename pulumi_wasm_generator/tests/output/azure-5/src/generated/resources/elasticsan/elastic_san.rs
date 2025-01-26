/// Manages an Elastic SAN resource.
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
///     let exampleElasticSan = elastic_san::create(
///         "exampleElasticSan",
///         ElasticSanArgs::builder()
///             .base_size_in_tib(1)
///             .extended_size_in_tib(2)
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .sku(ElasticSanSku::builder().name("example-value").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// An existing Elastic SAN can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:elasticsan/elasticSan:ElasticSan example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ElasticSan/elasticSans/esan1
/// ```
///
pub mod elastic_san {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ElasticSanArgs {
        /// Specifies the base size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.
        ///
        /// > **NOTE** When updating `base_size_in_tib`, the new value should be greater than the existing one.
        #[builder(into)]
        pub base_size_in_tib: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Specifies the extended size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.
        ///
        /// > **NOTE** `extended_size_in_tib` cannot be removed and when updating, the new value should be greater than the existing one.
        #[builder(into, default)]
        pub extended_size_in_tib: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The Azure Region where the Elastic SAN resource should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Elastic SAN resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group within which this Elastic SAN resource should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::InputOrOutput<
            super::super::types::elasticsan::ElasticSanSku,
        >,
        /// A mapping of tags which should be assigned to the Elastic SAN resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Logical zone for the Elastic SAN resource. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** `zones` cannot be specified if `sku.name` is set to `Premium_ZRS`.
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ElasticSanResult {
        /// Specifies the base size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.
        ///
        /// > **NOTE** When updating `base_size_in_tib`, the new value should be greater than the existing one.
        pub base_size_in_tib: pulumi_wasm_rust::Output<i32>,
        /// Specifies the extended size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.
        ///
        /// > **NOTE** `extended_size_in_tib` cannot be removed and when updating, the new value should be greater than the existing one.
        pub extended_size_in_tib: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Azure Region where the Elastic SAN resource should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Elastic SAN resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Elastic SAN resource should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_wasm_rust::Output<
            super::super::types::elasticsan::ElasticSanSku,
        >,
        /// A mapping of tags which should be assigned to the Elastic SAN resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Total Provisioned IOps of the Elastic SAN resource.
        pub total_iops: pulumi_wasm_rust::Output<i32>,
        /// Total Provisioned MBps Elastic SAN resource.
        pub total_mbps: pulumi_wasm_rust::Output<i32>,
        /// Total size of the Elastic SAN resource in TB.
        pub total_size_in_tib: pulumi_wasm_rust::Output<i32>,
        /// Total size of the provisioned Volumes in GiB.
        pub total_volume_size_in_gib: pulumi_wasm_rust::Output<i32>,
        /// Total number of volume groups in this Elastic SAN resource.
        pub volume_group_count: pulumi_wasm_rust::Output<i32>,
        /// Logical zone for the Elastic SAN resource. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** `zones` cannot be specified if `sku.name` is set to `Premium_ZRS`.
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ElasticSanArgs,
    ) -> ElasticSanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let base_size_in_tib_binding = args
            .base_size_in_tib
            .get_output(context)
            .get_inner();
        let extended_size_in_tib_binding = args
            .extended_size_in_tib
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let zones_binding = args.zones.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:elasticsan/elasticSan:ElasticSan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "baseSizeInTib".into(),
                    value: &base_size_in_tib_binding,
                },
                register_interface::ObjectField {
                    name: "extendedSizeInTib".into(),
                    value: &extended_size_in_tib_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "sku".into(),
                    value: &sku_binding,
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
                    name: "baseSizeInTib".into(),
                },
                register_interface::ResultField {
                    name: "extendedSizeInTib".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "totalIops".into(),
                },
                register_interface::ResultField {
                    name: "totalMbps".into(),
                },
                register_interface::ResultField {
                    name: "totalSizeInTib".into(),
                },
                register_interface::ResultField {
                    name: "totalVolumeSizeInGib".into(),
                },
                register_interface::ResultField {
                    name: "volumeGroupCount".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ElasticSanResult {
            base_size_in_tib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseSizeInTib").unwrap(),
            ),
            extended_size_in_tib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extendedSizeInTib").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            total_iops: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalIops").unwrap(),
            ),
            total_mbps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalMbps").unwrap(),
            ),
            total_size_in_tib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalSizeInTib").unwrap(),
            ),
            total_volume_size_in_gib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalVolumeSizeInGib").unwrap(),
            ),
            volume_group_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeGroupCount").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
