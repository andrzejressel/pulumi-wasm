/// Entry Metadata. A Data Catalog Entry resource represents another resource in Google Cloud Platform
/// (such as a BigQuery dataset or a Pub/Sub topic) or outside of Google Cloud Platform. Clients can use
/// the linkedResource field in the Entry resource to refer to the original resource ID of the source system.
///
/// An Entry resource contains resource details, such as its schema. An Entry can also be used to attach
/// flexible metadata, such as a Tag.
///
///
/// To get more information about Entry, see:
///
/// * [API documentation](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/data-catalog/docs)
///
/// ## Example Usage
///
/// ### Data Catalog Entry Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicEntry = entry::create(
///         "basicEntry",
///         EntryArgs::builder()
///             .entry_group("${entryGroup.id}")
///             .entry_id("my_entry")
///             .user_specified_system("SomethingExternal")
///             .user_specified_type("my_custom_type")
///             .build_struct(),
///     );
///     let entryGroup = entry_group::create(
///         "entryGroup",
///         EntryGroupArgs::builder().entry_group_id("my_group").build_struct(),
///     );
/// }
/// ```
/// ### Data Catalog Entry Fileset
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicEntry = entry::create(
///         "basicEntry",
///         EntryArgs::builder()
///             .entry_group("${entryGroup.id}")
///             .entry_id("my_entry")
///             .gcs_fileset_spec(
///                 EntryGcsFilesetSpec::builder()
///                     .filePatterns(vec!["gs://fake_bucket/dir/*",])
///                     .build_struct(),
///             )
///             .type_("FILESET")
///             .build_struct(),
///     );
///     let entryGroup = entry_group::create(
///         "entryGroup",
///         EntryGroupArgs::builder().entry_group_id("my_group").build_struct(),
///     );
/// }
/// ```
/// ### Data Catalog Entry Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicEntry = entry::create(
///         "basicEntry",
///         EntryArgs::builder()
///             .description("a custom type entry for a user specified system")
///             .display_name("my custom type entry")
///             .entry_group("${entryGroup.id}")
///             .entry_id("my_entry")
///             .linked_resource("my/linked/resource")
///             .schema(
///                 "{\n  \"columns\": [\n    {\n      \"column\": \"first_name\",\n      \"description\": \"First name\",\n      \"mode\": \"REQUIRED\",\n      \"type\": \"STRING\"\n    },\n    {\n      \"column\": \"last_name\",\n      \"description\": \"Last name\",\n      \"mode\": \"REQUIRED\",\n      \"type\": \"STRING\"\n    },\n    {\n      \"column\": \"address\",\n      \"description\": \"Address\",\n      \"mode\": \"REPEATED\",\n      \"subcolumns\": [\n        {\n          \"column\": \"city\",\n          \"description\": \"City\",\n          \"mode\": \"NULLABLE\",\n          \"type\": \"STRING\"\n        },\n        {\n          \"column\": \"state\",\n          \"description\": \"State\",\n          \"mode\": \"NULLABLE\",\n          \"type\": \"STRING\"\n        }\n      ],\n      \"type\": \"RECORD\"\n    }\n  ]\n}\n",
///             )
///             .user_specified_system("Something_custom")
///             .user_specified_type("my_user_specified_type")
///             .build_struct(),
///     );
///     let entryGroup = entry_group::create(
///         "entryGroup",
///         EntryGroupArgs::builder().entry_group_id("my_group").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Entry can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Entry can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datacatalog/entry:Entry default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod entry {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntryArgs {
        /// Entry description, which can consist of several sentences or paragraphs that describe entry contents.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Display information such as title and description. A short name to identify the entry,
        /// for example, "Analytics Data - Jan 2011".
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the entry group this entry is in.
        #[builder(into)]
        pub entry_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the entry to create.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub entry_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specification that applies to a Cloud Storage fileset. This is only valid on entries of type FILESET.
        /// Structure is documented below.
        #[builder(into, default)]
        pub gcs_fileset_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datacatalog::EntryGcsFilesetSpec>,
        >,
        /// The resource this metadata entry refers to.
        /// For Google Cloud Platform resources, linkedResource is the full name of the resource.
        /// For example, the linkedResource for a table resource from BigQuery is:
        /// //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId
        /// Output only when Entry is of type in the EntryType enum. For entries with userSpecifiedType,
        /// this field is optional and defaults to an empty string.
        #[builder(into, default)]
        pub linked_resource: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Schema of the entry (e.g. BigQuery, GoogleSQL, Avro schema), as a json string. An entry might not have any schema
        /// attached to it. See
        /// https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries#schema
        /// for what fields this schema can contain.
        #[builder(into, default)]
        pub schema: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the entry. Only used for Entries with types in the EntryType enum.
        /// Currently, only FILESET enum value is allowed. All other entries created through Data Catalog must use userSpecifiedType.
        /// Possible values are: `FILESET`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field indicates the entry's source system that Data Catalog does not integrate with.
        /// userSpecifiedSystem strings must begin with a letter or underscore and can only contain letters, numbers,
        /// and underscores; are case insensitive; must be at least 1 character and at most 64 characters long.
        #[builder(into, default)]
        pub user_specified_system: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Entry type if it does not fit any of the input-allowed values listed in EntryType enum above.
        /// When creating an entry, users should check the enum values first, if nothing matches the entry
        /// to be created, then provide a custom value, for example "my_special_type".
        /// userSpecifiedType strings must begin with a letter or underscore and can only contain letters,
        /// numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long.
        #[builder(into, default)]
        pub user_specified_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EntryResult {
        /// Specification for a group of BigQuery tables with name pattern [prefix]YYYYMMDD.
        /// Context: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding.
        /// Structure is documented below.
        pub bigquery_date_sharded_specs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datacatalog::EntryBigqueryDateShardedSpec>,
        >,
        /// Specification that applies to a BigQuery table. This is only valid on entries of type TABLE.
        /// Structure is documented below.
        pub bigquery_table_specs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datacatalog::EntryBigqueryTableSpec>,
        >,
        /// Entry description, which can consist of several sentences or paragraphs that describe entry contents.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Display information such as title and description. A short name to identify the entry,
        /// for example, "Analytics Data - Jan 2011".
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the entry group this entry is in.
        pub entry_group: pulumi_gestalt_rust::Output<String>,
        /// The id of the entry to create.
        ///
        ///
        /// - - -
        pub entry_id: pulumi_gestalt_rust::Output<String>,
        /// Specification that applies to a Cloud Storage fileset. This is only valid on entries of type FILESET.
        /// Structure is documented below.
        pub gcs_fileset_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::datacatalog::EntryGcsFilesetSpec>,
        >,
        /// This field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub.
        pub integrated_system: pulumi_gestalt_rust::Output<String>,
        /// The resource this metadata entry refers to.
        /// For Google Cloud Platform resources, linkedResource is the full name of the resource.
        /// For example, the linkedResource for a table resource from BigQuery is:
        /// //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId
        /// Output only when Entry is of type in the EntryType enum. For entries with userSpecifiedType,
        /// this field is optional and defaults to an empty string.
        pub linked_resource: pulumi_gestalt_rust::Output<String>,
        /// The Data Catalog resource name of the entry in URL format.
        /// Example: projects/{project_id}/locations/{location}/entryGroups/{entryGroupId}/entries/{entryId}.
        /// Note that this Entry and its child resources may not actually be stored in the location in this name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Schema of the entry (e.g. BigQuery, GoogleSQL, Avro schema), as a json string. An entry might not have any schema
        /// attached to it. See
        /// https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries#schema
        /// for what fields this schema can contain.
        pub schema: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of the entry. Only used for Entries with types in the EntryType enum.
        /// Currently, only FILESET enum value is allowed. All other entries created through Data Catalog must use userSpecifiedType.
        /// Possible values are: `FILESET`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// This field indicates the entry's source system that Data Catalog does not integrate with.
        /// userSpecifiedSystem strings must begin with a letter or underscore and can only contain letters, numbers,
        /// and underscores; are case insensitive; must be at least 1 character and at most 64 characters long.
        pub user_specified_system: pulumi_gestalt_rust::Output<Option<String>>,
        /// Entry type if it does not fit any of the input-allowed values listed in EntryType enum above.
        /// When creating an entry, users should check the enum values first, if nothing matches the entry
        /// to be created, then provide a custom value, for example "my_special_type".
        /// userSpecifiedType strings must begin with a letter or underscore and can only contain letters,
        /// numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long.
        pub user_specified_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EntryArgs,
    ) -> EntryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let entry_group_binding = args.entry_group.get_output(context);
        let entry_id_binding = args.entry_id.get_output(context);
        let gcs_fileset_spec_binding = args.gcs_fileset_spec.get_output(context);
        let linked_resource_binding = args.linked_resource.get_output(context);
        let schema_binding = args.schema.get_output(context);
        let type__binding = args.type_.get_output(context);
        let user_specified_system_binding = args
            .user_specified_system
            .get_output(context);
        let user_specified_type_binding = args.user_specified_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:datacatalog/entry:Entry".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entryGroup".into(),
                    value: &entry_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entryId".into(),
                    value: &entry_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gcsFilesetSpec".into(),
                    value: &gcs_fileset_spec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkedResource".into(),
                    value: &linked_resource_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schema".into(),
                    value: &schema_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userSpecifiedSystem".into(),
                    value: &user_specified_system_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userSpecifiedType".into(),
                    value: &user_specified_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EntryResult {
            bigquery_date_sharded_specs: o.get_field("bigqueryDateShardedSpecs"),
            bigquery_table_specs: o.get_field("bigqueryTableSpecs"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            entry_group: o.get_field("entryGroup"),
            entry_id: o.get_field("entryId"),
            gcs_fileset_spec: o.get_field("gcsFilesetSpec"),
            integrated_system: o.get_field("integratedSystem"),
            linked_resource: o.get_field("linkedResource"),
            name: o.get_field("name"),
            schema: o.get_field("schema"),
            type_: o.get_field("type"),
            user_specified_system: o.get_field("userSpecifiedSystem"),
            user_specified_type: o.get_field("userSpecifiedType"),
        }
    }
}
