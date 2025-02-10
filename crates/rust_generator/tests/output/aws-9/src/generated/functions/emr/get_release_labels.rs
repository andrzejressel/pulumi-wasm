#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_release_labels {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReleaseLabelsArgs {
        /// Filters the results of the request. Prefix specifies the prefix of release labels to return. Application specifies the application (with/without version) of release labels to return. See Filters.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::emr::GetReleaseLabelsFilters>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReleaseLabelsResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::emr::GetReleaseLabelsFilters>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Returned release labels.
        pub release_labels: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetReleaseLabelsArgs,
    ) -> GetReleaseLabelsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:emr/getReleaseLabels:getReleaseLabels".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReleaseLabelsResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            release_labels: o.get_field("releaseLabels"),
        }
    }
}
