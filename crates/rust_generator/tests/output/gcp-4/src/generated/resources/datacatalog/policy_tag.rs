/// Denotes one policy tag in a taxonomy.
///
///
/// To get more information about PolicyTag, see:
///
/// * [API documentation](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.taxonomies.policyTags)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/data-catalog/docs)
///
/// ## Example Usage
///
/// ### Data Catalog Taxonomies Policy Tag Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicPolicyTag = policy_tag::create(
///         "basicPolicyTag",
///         PolicyTagArgs::builder()
///             .description("A policy tag normally associated with low security items")
///             .display_name("Low security")
///             .taxonomy("${myTaxonomy.id}")
///             .build_struct(),
///     );
///     let myTaxonomy = taxonomy::create(
///         "myTaxonomy",
///         TaxonomyArgs::builder()
///             .activated_policy_types(vec!["FINE_GRAINED_ACCESS_CONTROL",])
///             .description("A collection of policy tags")
///             .display_name("taxonomy_display_name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Data Catalog Taxonomies Policy Tag Child Policies
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let childPolicy = policy_tag::create(
///         "childPolicy",
///         PolicyTagArgs::builder()
///             .description("A hash of the users ssn")
///             .display_name("ssn")
///             .parent_policy_tag("${parentPolicy.id}")
///             .taxonomy("${myTaxonomy.id}")
///             .build_struct(),
///     );
///     let childPolicy2 = policy_tag::create(
///         "childPolicy2",
///         PolicyTagArgs::builder()
///             .description("The users date of birth")
///             .display_name("dob")
///             .parent_policy_tag("${parentPolicy.id}")
///             .taxonomy("${myTaxonomy.id}")
///             .build_struct(),
///     );
///     let myTaxonomy = taxonomy::create(
///         "myTaxonomy",
///         TaxonomyArgs::builder()
///             .activated_policy_types(vec!["FINE_GRAINED_ACCESS_CONTROL",])
///             .description("A collection of policy tags")
///             .display_name("taxonomy_display_name")
///             .build_struct(),
///     );
///     let parentPolicy = policy_tag::create(
///         "parentPolicy",
///         PolicyTagArgs::builder()
///             .description("A policy tag category used for high security access")
///             .display_name("High")
///             .taxonomy("${myTaxonomy.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PolicyTag can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, PolicyTag can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datacatalog/policyTag:PolicyTag default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyTagArgs {
        /// Description of this policy tag. It must: contain only unicode characters, tabs,
        /// newlines, carriage returns and page breaks; and be at most 2000 bytes long when
        /// encoded in UTF-8. If not set, defaults to an empty description.
        /// If not set, defaults to an empty description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User defined name of this policy tag. It must: be unique within the parent
        /// taxonomy; contain only unicode letters, numbers, underscores, dashes and spaces;
        /// not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource name of this policy tag's parent policy tag.
        /// If empty, it means this policy tag is a top level policy tag.
        /// If not set, defaults to an empty string.
        #[builder(into, default)]
        pub parent_policy_tag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Taxonomy the policy tag is associated with
        ///
        ///
        /// - - -
        #[builder(into)]
        pub taxonomy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyTagResult {
        /// Resource names of child policy tags of this policy tag.
        pub child_policy_tags: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Description of this policy tag. It must: contain only unicode characters, tabs,
        /// newlines, carriage returns and page breaks; and be at most 2000 bytes long when
        /// encoded in UTF-8. If not set, defaults to an empty description.
        /// If not set, defaults to an empty description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User defined name of this policy tag. It must: be unique within the parent
        /// taxonomy; contain only unicode letters, numbers, underscores, dashes and spaces;
        /// not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Resource name of this policy tag, whose format is:
        /// "projects/{project}/locations/{region}/taxonomies/{taxonomy}/policyTags/{policytag}"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource name of this policy tag's parent policy tag.
        /// If empty, it means this policy tag is a top level policy tag.
        /// If not set, defaults to an empty string.
        pub parent_policy_tag: pulumi_gestalt_rust::Output<Option<String>>,
        /// Taxonomy the policy tag is associated with
        ///
        ///
        /// - - -
        pub taxonomy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyTagArgs,
    ) -> PolicyTagResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let parent_policy_tag_binding = args.parent_policy_tag.get_output(context);
        let taxonomy_binding = args.taxonomy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:datacatalog/policyTag:PolicyTag".into(),
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
                    name: "parentPolicyTag".into(),
                    value: parent_policy_tag_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taxonomy".into(),
                    value: taxonomy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyTagResult {
            child_policy_tags: o.get_field("childPolicyTags"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            parent_policy_tag: o.get_field("parentPolicyTag"),
            taxonomy: o.get_field("taxonomy"),
        }
    }
}
