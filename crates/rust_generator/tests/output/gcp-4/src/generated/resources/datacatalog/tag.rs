/// Tags are used to attach custom metadata to Data Catalog resources. Tags conform to the specifications within their tag template.
///
/// See [Data Catalog IAM](https://cloud.google.com/data-catalog/docs/concepts/iam) for information on the permissions needed to create or view tags.
///
///
/// To get more information about Tag, see:
///
/// * [API documentation](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.tags)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/data-catalog/docs)
///
/// ## Example Usage
///
/// ### Data Catalog Entry Tag Basic
///
///
/// ```yaml
/// resources:
///   entry:
///     type: gcp:datacatalog:Entry
///     properties:
///       entryGroup: ${entryGroup.id}
///       entryId: my_entry
///       userSpecifiedType: my_custom_type
///       userSpecifiedSystem: SomethingExternal
///   entryGroup:
///     type: gcp:datacatalog:EntryGroup
///     name: entry_group
///     properties:
///       entryGroupId: my_entry_group
///   tagTemplate:
///     type: gcp:datacatalog:TagTemplate
///     name: tag_template
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
///   basicTag:
///     type: gcp:datacatalog:Tag
///     name: basic_tag
///     properties:
///       parent: ${entry.id}
///       template: ${tagTemplate.id}
///       fields:
///         - fieldName: source
///           stringValue: my-string
/// ```
/// ### Data Catalog Entry Group Tag
///
///
/// ```yaml
/// resources:
///   firstEntry:
///     type: gcp:datacatalog:Entry
///     name: first_entry
///     properties:
///       entryGroup: ${entryGroup.id}
///       entryId: first_entry
///       userSpecifiedType: my_custom_type
///       userSpecifiedSystem: SomethingExternal
///   secondEntry:
///     type: gcp:datacatalog:Entry
///     name: second_entry
///     properties:
///       entryGroup: ${entryGroup.id}
///       entryId: second_entry
///       userSpecifiedType: another_custom_type
///       userSpecifiedSystem: SomethingElseExternal
///   entryGroup:
///     type: gcp:datacatalog:EntryGroup
///     name: entry_group
///     properties:
///       entryGroupId: my_entry_group
///   tagTemplate:
///     type: gcp:datacatalog:TagTemplate
///     name: tag_template
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
///   entryGroupTag:
///     type: gcp:datacatalog:Tag
///     name: entry_group_tag
///     properties:
///       parent: ${entryGroup.id}
///       template: ${tagTemplate.id}
///       fields:
///         - fieldName: source
///           stringValue: my-string
/// ```
/// ### Data Catalog Entry Tag Full
///
///
/// ```yaml
/// resources:
///   entry:
///     type: gcp:datacatalog:Entry
///     properties:
///       entryGroup: ${entryGroup.id}
///       entryId: my_entry
///       userSpecifiedType: my_custom_type
///       userSpecifiedSystem: SomethingExternal
///       schema: |
///         {
///           "columns": [
///             {
///               "column": "first_name",
///               "description": "First name",
///               "mode": "REQUIRED",
///               "type": "STRING"
///             },
///             {
///               "column": "last_name",
///               "description": "Last name",
///               "mode": "REQUIRED",
///               "type": "STRING"
///             },
///             {
///               "column": "address",
///               "description": "Address",
///               "mode": "REPEATED",
///               "subcolumns": [
///                 {
///                   "column": "city",
///                   "description": "City",
///                   "mode": "NULLABLE",
///                   "type": "STRING"
///                 },
///                 {
///                   "column": "state",
///                   "description": "State",
///                   "mode": "NULLABLE",
///                   "type": "STRING"
///                 }
///               ],
///               "type": "RECORD"
///             }
///           ]
///         }
///   entryGroup:
///     type: gcp:datacatalog:EntryGroup
///     name: entry_group
///     properties:
///       entryGroupId: my_entry_group
///   tagTemplate:
///     type: gcp:datacatalog:TagTemplate
///     name: tag_template
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
///   basicTag:
///     type: gcp:datacatalog:Tag
///     name: basic_tag
///     properties:
///       parent: ${entry.id}
///       template: ${tagTemplate.id}
///       fields:
///         - fieldName: source
///           stringValue: my-string
///         - fieldName: num_rows
///           doubleValue: 5
///         - fieldName: pii_type
///           enumValue: EMAIL
///       column: address
///   second-tag:
///     type: gcp:datacatalog:Tag
///     properties:
///       parent: ${entry.id}
///       template: ${tagTemplate.id}
///       fields:
///         - fieldName: source
///           stringValue: my-string
///         - fieldName: pii_type
///           enumValue: NONE
///       column: first_name
/// ```
///
/// ## Import
///
/// Tag can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Tag can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datacatalog/tag:Tag default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagArgs {
        /// Resources like Entry can have schemas associated with them. This scope allows users to attach tags to an individual
        /// column based on that schema. For attaching a tag to a nested column, use '.' to separate the column names. Example:
        /// 'outer_column.inner_column'
        #[builder(into, default)]
        pub column: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This maps the ID of a tag field to the value of and additional information about that field.
        /// Valid field IDs are defined by the tag's template. A tag must have at least 1 field and at most 500 fields.
        /// Structure is documented below.
        #[builder(into)]
        pub fields: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::datacatalog::TagField>,
        >,
        /// The name of the parent this tag is attached to. This can be the name of an entry or an entry group. If an entry group,
        /// the tag will be attached to all entries in that group.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name of the tag template that this tag uses. Example:
        /// projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}
        /// This field cannot be modified after creation.
        #[builder(into)]
        pub template: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagResult {
        /// Resources like Entry can have schemas associated with them. This scope allows users to attach tags to an individual
        /// column based on that schema. For attaching a tag to a nested column, use '.' to separate the column names. Example:
        /// 'outer_column.inner_column'
        pub column: pulumi_gestalt_rust::Output<Option<String>>,
        /// This maps the ID of a tag field to the value of and additional information about that field.
        /// Valid field IDs are defined by the tag's template. A tag must have at least 1 field and at most 500 fields.
        /// Structure is documented below.
        pub fields: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datacatalog::TagField>,
        >,
        /// The resource name of the tag in URL format. Example:
        /// projects/{project_id}/locations/{location}/entrygroups/{entryGroupId}/entries/{entryId}/tags/{tag_id} or
        /// projects/{project_id}/locations/{location}/entrygroups/{entryGroupId}/tags/{tag_id}
        /// where tag_id is a system-generated identifier. Note that this Tag may not actually be stored in the location in this name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the parent this tag is attached to. This can be the name of an entry or an entry group. If an entry group,
        /// the tag will be attached to all entries in that group.
        pub parent: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of the tag template that this tag uses. Example:
        /// projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}
        /// This field cannot be modified after creation.
        pub template: pulumi_gestalt_rust::Output<String>,
        /// The display name of the tag template.
        pub template_displayname: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TagArgs,
    ) -> TagResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let column_binding = args.column.get_output(context).get_inner();
        let fields_binding = args.fields.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let template_binding = args.template.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:datacatalog/tag:Tag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "column".into(),
                    value: &column_binding,
                },
                register_interface::ObjectField {
                    name: "fields".into(),
                    value: &fields_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "template".into(),
                    value: &template_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TagResult {
            column: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("column"),
            ),
            fields: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fields"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            template: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("template"),
            ),
            template_displayname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateDisplayname"),
            ),
        }
    }
}
