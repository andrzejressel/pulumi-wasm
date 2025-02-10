/// Creates a site in a global network.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_network::create(
///         "example",
///         GlobalNetworkArgs::builder().build_struct(),
///     );
///     let exampleSite = site::create(
///         "exampleSite",
///         SiteArgs::builder().global_network_id("${example.id}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_site` using the site ARN. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/site:Site example arn:aws:networkmanager::123456789012:site/global-network-0d47f6t230mz46dy4/site-444555aaabbb11223
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod site {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SiteArgs {
        /// Description of the Site.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Global Network to create the site in.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The site location as documented below.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkmanager::SiteLocation>,
        >,
        /// Key-value tags for the Site. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SiteResult {
        /// Site Amazon Resource Name (ARN)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the Site.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Global Network to create the site in.
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The site location as documented below.
        pub location: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkmanager::SiteLocation>,
        >,
        /// Key-value tags for the Site. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SiteArgs,
    ) -> SiteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let global_network_id_binding = args.global_network_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/site:Site".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalNetworkId".into(),
                    value: global_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SiteResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            global_network_id: o.get_field("globalNetworkId"),
            location: o.get_field("location"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
