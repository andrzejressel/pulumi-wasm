#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dlp_datasets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDlpDatasetsArgs {
        /// The account ID to fetch DLP Datasets from.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDlpDatasetsResult {
        /// The account ID to fetch DLP Datasets from.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A list of DLP Datasets.
        pub datasets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::GetDlpDatasetsDataset>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDlpDatasetsArgs,
    ) -> GetDlpDatasetsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getDlpDatasets:getDlpDatasets".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDlpDatasetsResult {
            account_id: o.get_field("accountId"),
            datasets: o.get_field("datasets"),
            id: o.get_field("id"),
        }
    }
}
