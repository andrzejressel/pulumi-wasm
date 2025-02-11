#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_assets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAssetsArgs {
        /// Outpost ARN.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Filters by list of Host IDs of a Dedicated Host.
        #[builder(into, default)]
        pub host_id_filters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Filters by list of state status. Valid values: "ACTIVE", "RETIRING".
        #[builder(into, default)]
        pub status_id_filters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetAssetsResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of all the asset ids found. This data source will fail if none are found.
        pub asset_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub host_id_filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub status_id_filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAssetsArgs,
    ) -> GetAssetsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let host_id_filters_binding = args.host_id_filters.get_output(context);
        let status_id_filters_binding = args.status_id_filters.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:outposts/getAssets:getAssets".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostIdFilters".into(),
                    value: &host_id_filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statusIdFilters".into(),
                    value: &status_id_filters_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAssetsResult {
            arn: o.get_field("arn"),
            asset_ids: o.get_field("assetIds"),
            host_id_filters: o.get_field("hostIdFilters"),
            id: o.get_field("id"),
            status_id_filters: o.get_field("statusIdFilters"),
        }
    }
}
