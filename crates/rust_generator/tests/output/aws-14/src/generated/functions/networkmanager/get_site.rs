#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_site {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSiteArgs {
        /// ID of the Global Network of the site to retrieve.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the specific site to retrieve.
        #[builder(into)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the Site.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSiteResult {
        /// ARN of the site.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the site.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Site location as documented below.
        pub locations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::networkmanager::GetSiteLocation>,
        >,
        pub site_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the Site.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSiteArgs,
    ) -> GetSiteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let global_network_id_binding_1 = args.global_network_id.get_output(context);
        let global_network_id_binding = global_network_id_binding_1.get_inner();
        let site_id_binding_1 = args.site_id.get_output(context);
        let site_id_binding = site_id_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:networkmanager/getSite:getSite".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSiteResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            global_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalNetworkId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            locations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locations"),
            ),
            site_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
