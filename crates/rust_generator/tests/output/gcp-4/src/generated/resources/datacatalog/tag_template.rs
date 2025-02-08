/// A tag template defines a tag, which can have one or more typed fields.
/// The template is used to create and attach the tag to GCP resources.
///
///
/// To get more information about TagTemplate, see:
///
/// * [API documentation](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.tagTemplates)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/data-catalog/docs)
///
/// ## Example Usage
///
/// ### Data Catalog Tag Template Basic
///
///
/// ```yaml
/// resources:
///   basicTagTemplate:
///     type: gcp:datacatalog:TagTemplate
///     name: basic_tag_template
///     properties:
///       tagTemplateId: my_template
///       region: us-central1
///       displayName: Demo Tag Template
///       fields:
///         - fieldId: source
///           displayName: Source of data asset
///           type:
///             primitiveType: STRING
///           isRequired: true
///         - fieldId: num_rows
///           displayName: Number of rows in the data asset
///           type:
///             primitiveType: DOUBLE
///         - fieldId: pii_type
///           displayName: PII type
///           type:
///             enumType:
///               allowedValues:
///                 - displayName: EMAIL
///                 - displayName: SOCIAL SECURITY NUMBER
///                 - displayName: NONE
///       forceDelete: 'false'
/// ```
///
/// ## Import
///
/// TagTemplate can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TagTemplate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datacatalog/tagTemplate:TagTemplate default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagTemplateArgs {
        /// The display name for this template.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of tag template field IDs and the settings for the field. This set is an exhaustive list of the allowed fields. This set must contain at least one field and at most 500 fields. The change of field_id will be resulting in re-creating of field. The change of primitive_type will be resulting in re-creating of field, however if the field is a required, you cannot update it.
        /// Structure is documented below.
        #[builder(into)]
        pub fields: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::datacatalog::TagTemplateField>,
        >,
        /// This confirms the deletion of any possible tags using this template. Must be set to true in order to delete the tag
        /// template.
        #[builder(into, default)]
        pub force_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Template location region.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The id of the tag template to create.
        #[builder(into)]
        pub tag_template_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagTemplateResult {
        /// The display name for this template.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of tag template field IDs and the settings for the field. This set is an exhaustive list of the allowed fields. This set must contain at least one field and at most 500 fields. The change of field_id will be resulting in re-creating of field. The change of primitive_type will be resulting in re-creating of field, however if the field is a required, you cannot update it.
        /// Structure is documented below.
        pub fields: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datacatalog::TagTemplateField>,
        >,
        /// This confirms the deletion of any possible tags using this template. Must be set to true in order to delete the tag
        /// template.
        pub force_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The resource name of the tag template in URL format. Example: projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Template location region.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The id of the tag template to create.
        pub tag_template_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TagTemplateArgs,
    ) -> TagTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let fields_binding = args.fields.get_output(context).get_inner();
        let force_delete_binding = args.force_delete.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let tag_template_id_binding = args
            .tag_template_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:datacatalog/tagTemplate:TagTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "fields".into(),
                    value: &fields_binding,
                },
                register_interface::ObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "tagTemplateId".into(),
                    value: &tag_template_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TagTemplateResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            fields: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fields"),
            ),
            force_delete: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDelete"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            tag_template_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagTemplateId"),
            ),
        }
    }
}
