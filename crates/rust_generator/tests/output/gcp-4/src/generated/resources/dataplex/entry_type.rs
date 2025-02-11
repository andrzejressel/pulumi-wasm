/// An Entry Type is a template for creating Entries.
///
///
///
/// ## Example Usage
///
/// ### Dataplex Entry Type Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testEntryTypeBasic = entry_type::create(
///         "testEntryTypeBasic",
///         EntryTypeArgs::builder()
///             .entry_type_id("entry-type-basic")
///             .location("us-central1")
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataplex Entry Type Full
///
///
/// ```yaml
/// resources:
///   testEntryTypeFull:
///     type: gcp:dataplex:AspectType
///     name: test_entry_type_full
///     properties:
///       aspectTypeId: tf-test-aspect-type_22811
///       location: us-central1
///       project: my-project-name
///       metadataTemplate: |
///         {
///           "name": "tf-test-template",
///           "type": "record",
///           "recordFields": [
///             {
///               "name": "type",
///               "type": "enum",
///               "annotations": {
///                 "displayName": "Type",
///                 "description": "Specifies the type of view represented by the entry."
///               },
///               "index": 1,
///               "constraints": {
///                 "required": true
///               },
///               "enumValues": [
///                 {
///                   "name": "VIEW",
///                   "index": 1
///                 }
///               ]
///             }
///           ]
///         }
///   testEntryTypeFullEntryType:
///     type: gcp:dataplex:EntryType
///     name: test_entry_type_full
///     properties:
///       entryTypeId: entry-type-full
///       project: my-project-name
///       location: us-central1
///       labels:
///         tag: test-tf
///       displayName: terraform entry type
///       description: entry type created by Terraform
///       typeAliases:
///         - TABLE
///         - DATABASE
///       platform: GCS
///       system: CloudSQL
///       requiredAspects:
///         - type: ${testEntryTypeFull.name}
/// ```
///
/// ## Import
///
/// EntryType can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/entryTypes/{{entry_type_id}}`
///
/// * `{{project}}/{{location}}/{{entry_type_id}}`
///
/// * `{{location}}/{{entry_type_id}}`
///
/// When using the `pulumi import` command, EntryType can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataplex/entryType:EntryType default projects/{{project}}/locations/{{location}}/entryTypes/{{entry_type_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/entryType:EntryType default {{project}}/{{location}}/{{entry_type_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/entryType:EntryType default {{location}}/{{entry_type_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod entry_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntryTypeArgs {
        /// Description of the EntryType.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The entry type id of the entry type.
        #[builder(into, default)]
        pub entry_type_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-defined labels for the EntryType.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where entry type will be created in.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The platform that Entries of this type belongs to.
        #[builder(into, default)]
        pub platform: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AspectInfo for the entry type.
        /// Structure is documented below.
        #[builder(into, default)]
        pub required_aspects: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dataplex::EntryTypeRequiredAspect>>,
        >,
        /// The system that Entries of this type belongs to.
        #[builder(into, default)]
        pub system: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates the class this Entry Type belongs to, for example, TABLE, DATABASE, MODEL.
        #[builder(into, default)]
        pub type_aliases: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct EntryTypeResult {
        /// The time when the EntryType was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the EntryType.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User friendly display name.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The entry type id of the entry type.
        pub entry_type_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// User-defined labels for the EntryType.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where entry type will be created in.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The relative resource name of the EntryType, of the form: projects/{project_number}/locations/{location_id}/entryTypes/{entry_type_id}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The platform that Entries of this type belongs to.
        pub platform: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// AspectInfo for the entry type.
        /// Structure is documented below.
        pub required_aspects: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::dataplex::EntryTypeRequiredAspect>>,
        >,
        /// The system that Entries of this type belongs to.
        pub system: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates the class this Entry Type belongs to, for example, TABLE, DATABASE, MODEL.
        pub type_aliases: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// System generated globally unique ID for the EntryType. This ID will be different if the EntryType is deleted and re-created with the same name.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time when the EntryType was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EntryTypeArgs,
    ) -> EntryTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let entry_type_id_binding = args.entry_type_id.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let platform_binding = args.platform.get_output(context);
        let project_binding = args.project.get_output(context);
        let required_aspects_binding = args.required_aspects.get_output(context);
        let system_binding = args.system.get_output(context);
        let type_aliases_binding = args.type_aliases.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataplex/entryType:EntryType".into(),
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
                    name: "entryTypeId".into(),
                    value: &entry_type_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platform".into(),
                    value: &platform_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requiredAspects".into(),
                    value: &required_aspects_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "system".into(),
                    value: &system_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "typeAliases".into(),
                    value: &type_aliases_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EntryTypeResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            entry_type_id: o.get_field("entryTypeId"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            platform: o.get_field("platform"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            required_aspects: o.get_field("requiredAspects"),
            system: o.get_field("system"),
            type_aliases: o.get_field("typeAliases"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
