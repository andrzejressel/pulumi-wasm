#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_gateway_app_types {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayAppTypesArgs {
        /// The account ID to fetch Gateway App Types from.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayAppTypesResult {
        /// The account ID to fetch Gateway App Types from.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A list of Gateway App Types.
        pub app_types: pulumi_gestalt_rust::Output<
            Vec<super::super::types::GetGatewayAppTypesAppType>,
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
        args: GetGatewayAppTypesArgs,
    ) -> GetGatewayAppTypesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getGatewayAppTypes:getGatewayAppTypes".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGatewayAppTypesResult {
            account_id: o.get_field("accountId"),
            app_types: o.get_field("appTypes"),
            id: o.get_field("id"),
        }
    }
}
