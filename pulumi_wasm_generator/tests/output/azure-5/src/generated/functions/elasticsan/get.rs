pub mod get {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetArgs {
        /// The name of this Elastic SAN.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Elastic SAN exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetResult {
        /// The base size of the Elastic SAN resource in TiB.
        pub base_size_in_tib: pulumi_wasm_rust::Output<i32>,
        /// The base size of the Elastic SAN resource in TiB.
        pub extended_size_in_tib: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Elastic SAN exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The SKU name.
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as defined below.
        pub skus: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsan::GetSkus>,
        >,
        /// A mapping of tags assigned to the Elastic SAN.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
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
        /// Logical zone for the Elastic SAN resource.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetArgs) -> GetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:elasticsan/get:get".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "id".into(),
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
                    name: "skus".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResult {
            base_size_in_tib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseSizeInTib").unwrap(),
            ),
            extended_size_in_tib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extendedSizeInTib").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            skus: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skus").unwrap(),
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
