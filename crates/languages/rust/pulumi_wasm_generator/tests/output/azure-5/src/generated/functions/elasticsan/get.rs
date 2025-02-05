pub mod get {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetArgs {
        /// The name of this Elastic SAN.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Elastic SAN exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetArgs,
    ) -> GetResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResult {
            base_size_in_tib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("baseSizeInTib"),
            ),
            extended_size_in_tib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("extendedSizeInTib"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            skus: pulumi_wasm_rust::__private::into_domain(o.extract_field("skus")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            total_iops: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("totalIops"),
            ),
            total_mbps: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("totalMbps"),
            ),
            total_size_in_tib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("totalSizeInTib"),
            ),
            total_volume_size_in_gib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("totalVolumeSizeInGib"),
            ),
            volume_group_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("volumeGroupCount"),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
