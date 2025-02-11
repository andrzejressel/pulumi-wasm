#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_data_shares {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataSharesArgs {
        /// An array of all data shares in the current region. See `data_shares` below.
        #[builder(into, default)]
        pub data_shares: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::redshift::GetDataSharesDataShare>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDataSharesResult {
        /// An array of all data shares in the current region. See `data_shares` below.
        pub data_shares: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::redshift::GetDataSharesDataShare>>,
        >,
        /// AWS region.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDataSharesArgs,
    ) -> GetDataSharesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_shares_binding = args.data_shares.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:redshift/getDataShares:getDataShares".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataShares".into(),
                    value: &data_shares_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDataSharesResult {
            data_shares: o.get_field("dataShares"),
            id: o.get_field("id"),
        }
    }
}
