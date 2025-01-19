/// An Aspect Type is a template for creating Aspects.
///
///
///
/// ## Example Usage
///
/// ### Dataplex Aspect Type Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testAspectTypeBasic = aspect_type::create(
///         "testAspectTypeBasic",
///         AspectTypeArgs::builder()
///             .aspect_type_id("aspect-type-basic")
///             .location("us-central1")
///             .metadata_template(
///                 "{\n  \"name\": \"tf-test-template\",\n  \"type\": \"record\",\n  \"recordFields\": [\n    {\n      \"name\": \"type\",\n      \"type\": \"enum\",\n      \"annotations\": {\n        \"displayName\": \"Type\",\n        \"description\": \"Specifies the type of view represented by the entry.\"\n      },\n      \"index\": 1,\n      \"constraints\": {\n        \"required\": true\n      },\n      \"enumValues\": [\n        {\n          \"name\": \"VIEW\",\n          \"index\": 1\n        }\n      ]\n    }\n  ]\n}",
///             )
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataplex Aspect Type Full
///
///
/// ```yaml
/// resources:
///   testAspectTypeFull:
///     type: gcp:dataplex:AspectType
///     name: test_aspect_type_full
///     properties:
///       aspectTypeId: aspect-type-full
///       project: my-project-name
///       location: us-central1
///       labels:
///         tag: test-tf
///       displayName: terraform aspect type
///       description: aspect type created by Terraform
///       metadataTemplate: |
///         {
///           "type": "record",
///           "name": "Schema",
///           "recordFields": [
///             {
///               "name": "fields",
///               "type": "array",
///               "index": 1,
///               "arrayItems": {
///                 "name": "field",
///                 "type": "record",
///                 "typeId": "field",
///                 "recordFields": [
///                   {
///                     "name": "name",
///                     "type": "string",
///                     "index": 1,
///                     "constraints": {
///                       "required": true
///                     }
///                   },
///                   {
///                     "name": "description",
///                     "type": "string",
///                     "index": 2
///                   },
///                   {
///                     "name": "dataType",
///                     "type": "string",
///                     "index": 3,
///                     "constraints": {
///                       "required": true
///                     }
///                   },
///                   {
///                     "name": "metadataType",
///                     "type": "enum",
///                     "index": 4,
///                     "constraints": {
///                       "required": true
///                     },
///                     "enumValues": [
///                       {
///                         "name": "BOOLEAN",
///                         "index": 1
///                       },
///                       {
///                         "name": "NUMBER",
///                         "index": 2
///                       },
///                       {
///                         "name": "STRING",
///                         "index": 3
///                       },
///                       {
///                         "name": "BYTES",
///                         "index": 4
///                       },
///                       {
///                         "name": "DATETIME",
///                         "index": 5
///                       },
///                       {
///                         "name": "TIMESTAMP",
///                         "index": 6
///                       },
///                       {
///                         "name": "GEOSPATIAL",
///                         "index": 7
///                       },
///                       {
///                         "name": "STRUCT",
///                         "index": 8
///                       },
///                       {
///                         "name": "OTHER",
///                         "index": 100
///                       }
///                     ]
///                   },
///                   {
///                     "name": "mode",
///                     "type": "enum",
///                     "index": 5,
///                     "enumValues": [
///                       {
///                         "name": "NULLABLE",
///                         "index": 1
///                       },
///                       {
///                         "name": "REPEATED",
///                         "index": 2
///                       },
///                       {
///                         "name": "REQUIRED",
///                         "index": 3
///                       }
///                     ]
///                   },
///                   {
///                     "name": "defaultValue",
///                     "type": "string",
///                     "index": 6
///                   },
///                   {
///                     "name": "annotations",
///                     "type": "map",
///                     "index": 7,
///                     "mapItems": {
///                       "name": "label",
///                       "type": "string"
///                     }
///                   },
///                   {
///                     "name": "fields",
///                     "type": "array",
///                     "index": 20,
///                     "arrayItems": {
///                       "name": "field",
///                       "type": "record",
///                       "typeRef": "field"
///                     }
///                   }
///                 ]
///               }
///             }
///           ]
///         }
/// ```
///
/// ## Import
///
/// AspectType can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/aspectTypes/{{aspect_type_id}}`
///
/// * `{{project}}/{{location}}/{{aspect_type_id}}`
///
/// * `{{location}}/{{aspect_type_id}}`
///
/// When using the `pulumi import` command, AspectType can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataplex/aspectType:AspectType default projects/{{project}}/locations/{{location}}/aspectTypes/{{aspect_type_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/aspectType:AspectType default {{project}}/{{location}}/{{aspect_type_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/aspectType:AspectType default {{location}}/{{aspect_type_id}}
/// ```
///
pub mod aspect_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AspectTypeArgs {
        /// The aspect type id of the aspect type.
        #[builder(into, default)]
        pub aspect_type_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the AspectType.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// User-defined labels for the AspectType.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where aspect type will be created in.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// MetadataTemplate of the Aspect.
        #[builder(into, default)]
        pub metadata_template: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AspectTypeResult {
        /// The aspect type id of the aspect type.
        pub aspect_type_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The time when the AspectType was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Description of the AspectType.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User friendly display name.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User-defined labels for the AspectType.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where aspect type will be created in.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// MetadataTemplate of the Aspect.
        pub metadata_template: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative resource name of the AspectType, of the form: projects/{project_number}/locations/{location_id}/aspectTypes/{aspect_type_id}
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Denotes the transfer status of the Aspect Type. It is unspecified
        /// for Aspect Type created from Dataplex API.
        pub transfer_status: pulumi_wasm_rust::Output<String>,
        /// System generated globally unique ID for the AspectType. This ID will be different if the AspectType is deleted and re-created with the same name.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// The time when the AspectType was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AspectTypeArgs) -> AspectTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aspect_type_id_binding = args.aspect_type_id.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let metadata_template_binding = args.metadata_template.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataplex/aspectType:AspectType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aspectTypeId".into(),
                    value: &aspect_type_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
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
                    name: "metadataTemplate".into(),
                    value: &metadata_template_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aspectTypeId".into(),
                },
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
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "metadataTemplate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "transferStatus".into(),
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
        AspectTypeResult {
            aspect_type_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aspectTypeId").unwrap(),
            ),
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
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            metadata_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataTemplate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            transfer_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transferStatus").unwrap(),
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
