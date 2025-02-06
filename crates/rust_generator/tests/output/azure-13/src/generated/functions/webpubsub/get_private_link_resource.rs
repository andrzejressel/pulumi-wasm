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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPrivateLinkResourceArgs,
    ) -> GetPrivateLinkResourceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let web_pubsub_id_binding = args.web_pubsub_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:webpubsub/getPrivateLinkResource:getPrivateLinkResource"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "webPubsubId".into(),
                    value: &web_pubsub_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPrivateLinkResourceResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            shared_private_link_resource_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sharedPrivateLinkResourceTypes"),
            ),
            web_pubsub_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("webPubsubId"),
            ),
        }
    }
}
