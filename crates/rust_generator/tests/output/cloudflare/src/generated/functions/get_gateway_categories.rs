#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_gateway_categories {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayCategoriesArgs {
        /// The account ID to fetch Gateway Categories from.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayCategoriesResult {
        /// The account ID to fetch Gateway Categories from.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A list of Gateway Categories.
        pub categories: pulumi_gestalt_rust::Output<
            Vec<super::super::types::GetGatewayCategoriesCategory>,
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
        args: GetGatewayCategoriesArgs,
    ) -> GetGatewayCategoriesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getGatewayCategories:getGatewayCategories".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGatewayCategoriesResult {
            account_id: o.get_field("accountId"),
            categories: o.get_field("categories"),
            id: o.get_field("id"),
        }
    }
}
