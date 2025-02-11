/// An Entry Group represents a logical grouping of one or more Entries.
///
///
///
/// ## Example Usage
///
/// ### Dataplex Entry Group Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testEntryGroupBasic = entry_group::create(
///         "testEntryGroupBasic",
///         EntryGroupArgs::builder()
///             .entry_group_id("entry-group-basic")
///             .location("us-central1")
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataplex Entry Group Full
///
///
/// ```yaml
/// resources:
///   testEntryGroupFull:
///     type: gcp:dataplex:EntryGroup
///     name: test_entry_group_full
///     properties:
///       entryGroupId: entry-group-full
///       project: my-project-name
///       location: us-central1
///       labels:
///         tag: test-tf
///       displayName: terraform entry group
///       description: entry group created by Terraform
/// ```
///
/// ## Import
///
/// EntryGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/entryGroups/{{entry_group_id}}`
///
/// * `{{project}}/{{location}}/{{entry_group_id}}`
///
/// * `{{location}}/{{entry_group_id}}`
///
/// When using the `pulumi import` command, EntryGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataplex/entryGroup:EntryGroup default projects/{{project}}/locations/{{location}}/entryGroups/{{entry_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/entryGroup:EntryGroup default {{project}}/{{location}}/{{entry_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/entryGroup:EntryGroup default {{location}}/{{entry_group_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod entry_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntryGroupArgs {
        /// Description of the EntryGroup.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The entry group id of the entry group.
        #[builder(into, default)]
        pub entry_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-defined labels for the EntryGroup.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where entry group will be created in.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EntryGroupResult {
        /// The time when the EntryGroup was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the EntryGroup.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User friendly display name.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The entry group id of the entry group.
        pub entry_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// User-defined labels for the EntryGroup.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where entry group will be created in.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The relative resource name of the EntryGroup, of the form: projects/{project_number}/locations/{location_id}/entryGroups/{entry_group_id}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Denotes the transfer status of the Entry Group. It is unspecified
        /// for Entry Group created from Dataplex API.
        pub transfer_status: pulumi_gestalt_rust::Output<String>,
        /// System generated globally unique ID for the EntryGroup. This ID will be different if the EntryGroup is deleted and re-created with the same name.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time when the EntryGroup was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EntryGroupArgs,
    ) -> EntryGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let entry_group_id_binding = args.entry_group_id.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataplex/entryGroup:EntryGroup".into(),
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
                    name: "entryGroupId".into(),
                    value: &entry_group_id_binding.drop_type(),
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
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EntryGroupResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            entry_group_id: o.get_field("entryGroupId"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            transfer_status: o.get_field("transferStatus"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
