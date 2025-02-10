/// Provides an IP access control group in AWS WorkSpaces Service
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let contractors = ip_group::create(
///         "contractors",
///         IpGroupArgs::builder()
///             .description("Contractors IP access control group")
///             .name("Contractors")
///             .rules(
///                 vec![
///                     IpGroupRule::builder().description("NY").source("150.24.14.0/24")
///                     .build_struct(), IpGroupRule::builder().description("LA")
///                     .source("125.191.14.85/32").build_struct(), IpGroupRule::builder()
///                     .description("STL").source("44.98.100.0/24").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WorkSpaces IP groups using their GroupID. For example:
///
/// ```sh
/// $ pulumi import aws:workspaces/ipGroup:IpGroup example wsipg-488lrtl3k
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ip_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IpGroupArgs {
        /// The description of the IP group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the IP group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more pairs specifying the IP group rule (in CIDR format) from which web requests originate.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::workspaces::IpGroupRule>>,
        >,
        /// A map of tags assigned to the WorkSpaces directory. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IpGroupResult {
        /// The description of the IP group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the IP group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more pairs specifying the IP group rule (in CIDR format) from which web requests originate.
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::workspaces::IpGroupRule>>,
        >,
        /// A map of tags assigned to the WorkSpaces directory. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: IpGroupArgs,
    ) -> IpGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:workspaces/ipGroup:IpGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IpGroupResult {
            description: o.get_field("description"),
            name: o.get_field("name"),
            rules: o.get_field("rules"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
