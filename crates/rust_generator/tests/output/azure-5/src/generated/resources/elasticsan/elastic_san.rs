/// Manages an Elastic SAN resource.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod elastic_san {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ElasticSanArgs {
        /// Specifies the base size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.
        ///
        /// > **NOTE** When updating `base_size_in_tib`, the new value should be greater than the existing one.
        #[builder(into)]
        pub base_size_in_tib: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the extended size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.
        ///
        /// > **NOTE** `extended_size_in_tib` cannot be removed and when updating, the new value should be greater than the existing one.
        #[builder(into, default)]
        pub extended_size_in_tib: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Azure Region where the Elastic SAN resource should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Elastic SAN resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group within which this Elastic SAN resource should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::elasticsan::ElasticSanSku,
        >,
        /// A mapping of tags which should be assigned to the Elastic SAN resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Logical zone for the Elastic SAN resource. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** `zones` cannot be specified if `sku.name` is set to `Premium_ZRS`.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ElasticSanResult {
        /// Specifies the base size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.
        ///
        /// > **NOTE** When updating `base_size_in_tib`, the new value should be greater than the existing one.
        pub base_size_in_tib: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the extended size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.
        ///
        /// > **NOTE** `extended_size_in_tib` cannot be removed and when updating, the new value should be greater than the existing one.
        pub extended_size_in_tib: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Azure Region where the Elastic SAN resource should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Elastic SAN resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Elastic SAN resource should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_gestalt_rust::Output<
            super::super::types::elasticsan::ElasticSanSku,
        >,
        /// A mapping of tags which should be assigned to the Elastic SAN resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
        /// Logical zone for the Elastic SAN resource. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** `zones` cannot be specified if `sku.name` is set to `Premium_ZRS`.
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ElasticSanArgs,
    ) -> ElasticSanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let base_size_in_tib_binding = args.base_size_in_tib.get_output(context);
        let extended_size_in_tib_binding = args.extended_size_in_tib.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:elasticsan/elasticSan:ElasticSan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseSizeInTib".into(),
                    value: base_size_in_tib_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extendedSizeInTib".into(),
                    value: extended_size_in_tib_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: zones_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ElasticSanResult {
            base_size_in_tib: o.get_field("baseSizeInTib"),
            extended_size_in_tib: o.get_field("extendedSizeInTib"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
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
