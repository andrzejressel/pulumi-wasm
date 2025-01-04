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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod entry_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntryTypeArgs {
        /// Description of the EntryType.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The entry type id of the entry type.
        #[builder(into, default)]
        pub entry_type_id: pulumi_wasm_rust::Output<Option<String>>,
        /// User-defined labels for the EntryType.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where entry type will be created in.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The platform that Entries of this type belongs to.
        #[builder(into, default)]
        pub platform: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// AspectInfo for the entry type.
        /// Structure is documented below.
        #[builder(into, default)]
        pub required_aspects: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dataplex::EntryTypeRequiredAspect>>,
        >,
        /// The system that Entries of this type belongs to.
        #[builder(into, default)]
        pub system: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates the class this Entry Type belongs to, for example, TABLE, DATABASE, MODEL.
        #[builder(into, default)]
        pub type_aliases: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct EntryTypeResult {
        /// The time when the EntryType was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Description of the EntryType.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User friendly display name.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The entry type id of the entry type.
        pub entry_type_id: pulumi_wasm_rust::Output<Option<String>>,
        /// User-defined labels for the EntryType.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where entry type will be created in.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative resource name of the EntryType, of the form: projects/{project_number}/locations/{location_id}/entryTypes/{entry_type_id}
        pub name: pulumi_wasm_rust::Output<String>,
        /// The platform that Entries of this type belongs to.
        pub platform: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// AspectInfo for the entry type.
        /// Structure is documented below.
        pub required_aspects: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dataplex::EntryTypeRequiredAspect>>,
        >,
        /// The system that Entries of this type belongs to.
        pub system: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates the class this Entry Type belongs to, for example, TABLE, DATABASE, MODEL.
        pub type_aliases: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// System generated globally unique ID for the EntryType. This ID will be different if the EntryType is deleted and re-created with the same name.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// The time when the EntryType was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EntryTypeArgs) -> EntryTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let entry_type_id_binding = args.entry_type_id.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let platform_binding = args.platform.get_inner();
        let project_binding = args.project.get_inner();
        let required_aspects_binding = args.required_aspects.get_inner();
        let system_binding = args.system.get_inner();
        let type_aliases_binding = args.type_aliases.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataplex/entryType:EntryType".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "entryTypeId".into(),
                    value: &entry_type_id_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "platform".into(),
                    value: &platform_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "requiredAspects".into(),
                    value: &required_aspects_binding,
                },
                register_interface::ObjectField {
                    name: "system".into(),
                    value: &system_binding,
                },
                register_interface::ObjectField {
                    name: "typeAliases".into(),
                    value: &type_aliases_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "entryTypeId".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "requiredAspects".into(),
                },
                register_interface::ResultField {
                    name: "system".into(),
                },
                register_interface::ResultField {
                    name: "typeAliases".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EntryTypeResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            entry_type_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entryTypeId").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            required_aspects: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requiredAspects").unwrap(),
            ),
            system: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("system").unwrap(),
            ),
            type_aliases: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("typeAliases").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
