#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLinkArgs {
        /// ID of the Global Network of the link to retrieve.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the specific link to retrieve.
        #[builder(into)]
        pub link_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the link.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLinkResult {
        /// ARN of the link.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Upload speed and download speed of the link as documented below
        pub bandwidths: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::networkmanager::GetLinkBandwidth>,
        >,
        /// Description of the link.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub link_id: pulumi_gestalt_rust::Output<String>,
        /// Provider of the link.
        pub provider_name: pulumi_gestalt_rust::Output<String>,
        /// ID of the site.
        pub site_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the link.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of the link.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLinkArgs,
    ) -> GetLinkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let global_network_id_binding = args.global_network_id.get_output(context);
        let link_id_binding = args.link_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:networkmanager/getLink:getLink".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkId".into(),
                    value: &link_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLinkResult {
            arn: o.get_field("arn"),
            bandwidths: o.get_field("bandwidths"),
            description: o.get_field("description"),
            global_network_id: o.get_field("globalNetworkId"),
            id: o.get_field("id"),
            link_id: o.get_field("linkId"),
            provider_name: o.get_field("providerName"),
            site_id: o.get_field("siteId"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
        }
    }
}
