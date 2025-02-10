#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_links {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLinksArgs {
        /// ID of the Global Network of the links to retrieve.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Link provider to retrieve.
        #[builder(into, default)]
        pub provider_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the site of the links to retrieve.
        #[builder(into, default)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Restricts the list to the links with these tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Link type to retrieve.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetLinksResult {
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// IDs of the links.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub provider_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub site_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLinksArgs,
    ) -> GetLinksResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let global_network_id_binding = args.global_network_id.get_output(context);
        let provider_name_binding = args.provider_name.get_output(context);
        let site_id_binding = args.site_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:networkmanager/getLinks:getLinks".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalNetworkId".into(),
                    value: global_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerName".into(),
                    value: provider_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteId".into(),
                    value: site_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLinksResult {
            global_network_id: o.get_field("globalNetworkId"),
            id: o.get_field("id"),
            ids: o.get_field("ids"),
            provider_name: o.get_field("providerName"),
            site_id: o.get_field("siteId"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
        }
    }
}
