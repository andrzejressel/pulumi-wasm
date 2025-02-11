#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_client {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClientArgs {
        /// The name of the brand.
        #[builder(into)]
        pub brand: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The client_id of the brand.
        #[builder(into)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetClientResult {
        pub brand: pulumi_gestalt_rust::Output<String>,
        pub client_id: pulumi_gestalt_rust::Output<String>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub secret: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetClientArgs,
    ) -> GetClientResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let brand_binding = args.brand.get_output(context);
        let client_id_binding = args.client_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:iap/getClient:getClient".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "brand".into(),
                    value: &brand_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClientResult {
            brand: o.get_field("brand"),
            client_id: o.get_field("clientId"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            secret: o.get_field("secret"),
        }
    }
}
