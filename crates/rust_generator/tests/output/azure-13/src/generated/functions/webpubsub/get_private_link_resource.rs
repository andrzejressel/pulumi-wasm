#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_private_link_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrivateLinkResourceArgs {
        /// The ID of an existing Web Pubsub Resource which Private Link Resource should be retrieved for.
        #[builder(into)]
        pub web_pubsub_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPrivateLinkResourceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `shared_private_link_resource_types` block as defined below.
        pub shared_private_link_resource_types: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::webpubsub::GetPrivateLinkResourceSharedPrivateLinkResourceType,
            >,
        >,
        pub web_pubsub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPrivateLinkResourceArgs,
    ) -> GetPrivateLinkResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let web_pubsub_id_binding = args.web_pubsub_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:webpubsub/getPrivateLinkResource:getPrivateLinkResource"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webPubsubId".into(),
                    value: &web_pubsub_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPrivateLinkResourceResult {
            id: o.get_field("id"),
            shared_private_link_resource_types: o
                .get_field("sharedPrivateLinkResourceTypes"),
            web_pubsub_id: o.get_field("webPubsubId"),
        }
    }
}
