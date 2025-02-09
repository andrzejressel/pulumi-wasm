#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_regions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionsArgs {
        /// If true the source will query all regions regardless of availability.
        #[builder(into, default)]
        pub all_regions: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block(s) to use as filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::GetRegionsFilter>>,
        >,
        /// Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionsResult {
        pub all_regions: pulumi_gestalt_rust::Output<Option<bool>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::GetRegionsFilter>>,
        >,
        /// Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Names of regions that meets the criteria.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegionsArgs,
    ) -> GetRegionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let all_regions_binding = args.all_regions.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:index/getRegions:getRegions".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allRegions".into(),
                    value: all_regions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegionsResult {
            all_regions: o.get_field("allRegions"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            names: o.get_field("names"),
        }
    }
}
