#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetArgs {
        /// The name of this Elastic SAN.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Elastic SAN exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResult {
        /// The base size of the Elastic SAN resource in TiB.
        pub base_size_in_tib: pulumi_gestalt_rust::Output<i32>,
        /// The base size of the Elastic SAN resource in TiB.
        pub extended_size_in_tib: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Elastic SAN exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The SKU name.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as defined below.
        pub skus: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsan::GetSkus>,
        >,
        /// A mapping of tags assigned to the Elastic SAN.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Total Provisioned IOps of the Elastic SAN resource.
        pub total_iops: pulumi_gestalt_rust::Output<i32>,
        /// Total Provisioned MBps Elastic SAN resource.
        pub total_mbps: pulumi_gestalt_rust::Output<i32>,
        /// Total size of the Elastic SAN resource in TB.
        pub total_size_in_tib: pulumi_gestalt_rust::Output<i32>,
        /// Total size of the provisioned Volumes in GiB.
        pub total_volume_size_in_gib: pulumi_gestalt_rust::Output<i32>,
        /// Total number of volume groups in this Elastic SAN resource.
        pub volume_group_count: pulumi_gestalt_rust::Output<i32>,
        /// Logical zone for the Elastic SAN resource.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context, args: GetArgs) -> GetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:elasticsan/get:get".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResult {
            base_size_in_tib: o.get_field("baseSizeInTib"),
            extended_size_in_tib: o.get_field("extendedSizeInTib"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            skus: o.get_field("skus"),
            tags: o.get_field("tags"),
            total_iops: o.get_field("totalIops"),
            total_mbps: o.get_field("totalMbps"),
            total_size_in_tib: o.get_field("totalSizeInTib"),
            total_volume_size_in_gib: o.get_field("totalVolumeSizeInGib"),
            volume_group_count: o.get_field("volumeGroupCount"),
            zones: o.get_field("zones"),
        }
    }
}
