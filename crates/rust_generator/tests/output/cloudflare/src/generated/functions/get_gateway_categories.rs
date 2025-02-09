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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetGatewayCategoriesArgs,
    ) -> GetGatewayCategoriesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getGatewayCategories:getGatewayCategories".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGatewayCategoriesResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            categories: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("categories"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
