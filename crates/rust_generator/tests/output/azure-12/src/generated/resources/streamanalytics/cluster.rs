/// Manages a Stream Analytics Cluster.
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
///     let exampleCluster = cluster::create(
///         "exampleCluster",
///         ClusterArgs::builder()
///             .location("${example.location}")
///             .name("examplestreamanalyticscluster")
///             .resource_group_name("${example.name}")
///             .streaming_capacity(36)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Stream Analytics Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/cluster:Cluster example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.StreamAnalytics/clusters/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// The Azure Region where the Stream Analytics Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Stream Analytics Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of streaming units supported by the Cluster. Accepted values are multiples of `36` in the range of `36` to `216`.
        #[builder(into)]
        pub streaming_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A mapping of tags which should be assigned to the Stream Analytics.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// The Azure Region where the Stream Analytics Cluster should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Stream Analytics Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The number of streaming units supported by the Cluster. Accepted values are multiples of `36` in the range of `36` to `216`.
        pub streaming_capacity: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags which should be assigned to the Stream Analytics.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let streaming_capacity_binding = args.streaming_capacity.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
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
                    name: "streamingCapacity".into(),
                    value: streaming_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            streaming_capacity: o.get_field("streamingCapacity"),
            tags: o.get_field("tags"),
        }
    }
}
