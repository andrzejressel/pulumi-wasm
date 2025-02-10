#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_received_licenses {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReceivedLicensesArgs {
        /// Custom filter block as described below.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::licensemanager::GetReceivedLicensesFilter,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReceivedLicensesResult {
        /// List of all the license ARNs found.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::licensemanager::GetReceivedLicensesFilter,
                >,
            >,
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
        args: GetReceivedLicensesArgs,
    ) -> GetReceivedLicensesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:licensemanager/getReceivedLicenses:getReceivedLicenses".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReceivedLicensesResult {
            arns: o.get_field("arns"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
        }
    }
}
