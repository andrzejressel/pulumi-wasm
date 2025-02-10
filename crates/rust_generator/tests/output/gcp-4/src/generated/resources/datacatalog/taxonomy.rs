/// A collection of policy tags that classify data along a common axis.
///
///
/// To get more information about Taxonomy, see:
///
/// * [API documentation](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.taxonomies)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/data-catalog/docs)
///
/// ## Example Usage
///
/// ### Data Catalog Taxonomy Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicTaxonomy = taxonomy::create(
///         "basicTaxonomy",
///         TaxonomyArgs::builder()
///             .activated_policy_types(vec!["FINE_GRAINED_ACCESS_CONTROL",])
///             .description("A collection of policy tags")
///             .display_name("my_taxonomy")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Taxonomy can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Taxonomy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datacatalog/taxonomy:Taxonomy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod taxonomy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TaxonomyArgs {
        /// A list of policy types that are activated for this taxonomy. If not set,
        /// defaults to an empty list.
        /// Each value may be one of: `POLICY_TYPE_UNSPECIFIED`, `FINE_GRAINED_ACCESS_CONTROL`.
        #[builder(into, default)]
        pub activated_policy_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Description of this taxonomy. It must: contain only unicode characters,
        /// tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes
        /// long when encoded in UTF-8. If not set, defaults to an empty description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User defined name of this taxonomy.
        /// The taxonomy display name must be unique within an organization.
        /// It must: contain only unicode letters, numbers, underscores, dashes
        /// and spaces; not start or end with spaces; and be at most 200 bytes
        /// long when encoded in UTF-8.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Taxonomy location region.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TaxonomyResult {
        /// A list of policy types that are activated for this taxonomy. If not set,
        /// defaults to an empty list.
        /// Each value may be one of: `POLICY_TYPE_UNSPECIFIED`, `FINE_GRAINED_ACCESS_CONTROL`.
        pub activated_policy_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Description of this taxonomy. It must: contain only unicode characters,
        /// tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes
        /// long when encoded in UTF-8. If not set, defaults to an empty description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User defined name of this taxonomy.
        /// The taxonomy display name must be unique within an organization.
        /// It must: contain only unicode letters, numbers, underscores, dashes
        /// and spaces; not start or end with spaces; and be at most 200 bytes
        /// long when encoded in UTF-8.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Resource name of this taxonomy, whose format is:
        /// "projects/{project}/locations/{region}/taxonomies/{taxonomy}".
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Taxonomy location region.
        pub region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TaxonomyArgs,
    ) -> TaxonomyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let activated_policy_types_binding = args
            .activated_policy_types
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:datacatalog/taxonomy:Taxonomy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activatedPolicyTypes".into(),
                    value: activated_policy_types_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TaxonomyResult {
            activated_policy_types: o.get_field("activatedPolicyTypes"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
        }
    }
}
