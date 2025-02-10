/// A Cloud Security Command Center's (Cloud SCC) finding source. A finding
/// source is an entity or a mechanism that can produce a finding. A source is
/// like a container of findings that come from the same scanner, logger,
/// monitor, etc.
///
///
/// To get more information about OrganizationSource, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v2/organizations.sources)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/security-command-center/docs)
///
/// ## Example Usage
///
/// ### Scc Source Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let customSource = source::create(
///         "customSource",
///         SourceArgs::builder()
///             .description("My custom Cloud Security Command Center Finding Source")
///             .display_name("My Source")
///             .organization("123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OrganizationSource can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/sources/{{name}}`
///
/// * `{{organization}}/{{name}}`
///
/// When using the `pulumi import` command, OrganizationSource can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationSource:V2OrganizationSource default organizations/{{organization}}/sources/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationSource:V2OrganizationSource default {{organization}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_organization_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2OrganizationSourceArgs {
        /// The description of the source (max of 1024 characters).
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The source’s display name. A source’s display name must be unique
        /// amongst its siblings, for example, two sources with the same parent
        /// can't share the same display name. The display name must start and end
        /// with a letter or digit, may contain letters, digits, spaces, hyphens,
        /// and underscores, and can be no longer than 32 characters.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The organization whose Cloud Security Command Center the Source
        /// lives in.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub organization: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct V2OrganizationSourceResult {
        /// The description of the source (max of 1024 characters).
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The source’s display name. A source’s display name must be unique
        /// amongst its siblings, for example, two sources with the same parent
        /// can't share the same display name. The display name must start and end
        /// with a letter or digit, may contain letters, digits, spaces, hyphens,
        /// and underscores, and can be no longer than 32 characters.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The resource name of this source, in the format
        /// `organizations/{{organization}}/sources/{{source}}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The organization whose Cloud Security Command Center the Source
        /// lives in.
        ///
        ///
        /// - - -
        pub organization: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: V2OrganizationSourceArgs,
    ) -> V2OrganizationSourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let organization_binding = args.organization.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2OrganizationSource:V2OrganizationSource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organization".into(),
                    value: organization_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        V2OrganizationSourceResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            organization: o.get_field("organization"),
        }
    }
}
