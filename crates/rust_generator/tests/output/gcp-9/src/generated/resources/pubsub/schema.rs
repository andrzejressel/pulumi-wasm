/// A schema is a format that messages must follow,
/// creating a contract between publisher and subscriber that Pub/Sub will enforce.
///
///
/// To get more information about Schema, see:
///
/// * [API documentation](https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.schemas)
/// * How-to Guides
///     * [Creating and managing schemas](https://cloud.google.com/pubsub/docs/schemas)
///
/// ## Example Usage
///
/// ### Pubsub Schema Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = schema::create(
///         "example",
///         SchemaArgs::builder()
///             .definition(
///                 "{\n  \"type\" : \"record\",\n  \"name\" : \"Avro\",\n  \"fields\" : [\n    {\n      \"name\" : \"StringField\",\n      \"type\" : \"string\"\n    },\n    {\n      \"name\" : \"IntField\",\n      \"type\" : \"int\"\n    }\n  ]\n}",
///             )
///             .name("example-schema")
///             .type_("AVRO")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Pubsub Schema Protobuf
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = schema::create(
///         "example",
///         SchemaArgs::builder()
///             .definition(
///                 "syntax = \"proto3\";\nmessage Results {\nstring message_request = 1;\nstring message_response = 2;\nstring timestamp_request = 3;\nstring timestamp_response = 4;\n}",
///             )
///             .name("example")
///             .type_("PROTOCOL_BUFFER")
///             .build_struct(),
///     );
///     let exampleTopic = topic::create(
///         "exampleTopic",
///         TopicArgs::builder()
///             .name("example-topic")
///             .schema_settings(
///                 TopicSchemaSettings::builder()
///                     .encoding("JSON")
///                     .schema("projects/my-project-name/schemas/example")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Schema can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/schemas/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Schema can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:pubsub/schema:Schema default projects/{{project}}/schemas/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/schema:Schema default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/schema:Schema default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod schema {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The definition of the schema.
        /// This should contain a string representing the full definition of the schema
        /// that is a valid schema definition of the type specified in type. Changes
        /// to the definition commit new [schema revisions](https://cloud.google.com/pubsub/docs/commit-schema-revision).
        /// A schema can only have up to 20 revisions, so updates that fail with an
        /// error indicating that the limit has been reached require manually
        /// [deleting old revisions](https://cloud.google.com/pubsub/docs/delete-schema-revision).
        #[builder(into, default)]
        pub definition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID to use for the schema, which will become the final component of the schema's resource name.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the schema definition
        /// Default value is `TYPE_UNSPECIFIED`.
        /// Possible values are: `TYPE_UNSPECIFIED`, `PROTOCOL_BUFFER`, `AVRO`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// The definition of the schema.
        /// This should contain a string representing the full definition of the schema
        /// that is a valid schema definition of the type specified in type. Changes
        /// to the definition commit new [schema revisions](https://cloud.google.com/pubsub/docs/commit-schema-revision).
        /// A schema can only have up to 20 revisions, so updates that fail with an
        /// error indicating that the limit has been reached require manually
        /// [deleting old revisions](https://cloud.google.com/pubsub/docs/delete-schema-revision).
        pub definition: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID to use for the schema, which will become the final component of the schema's resource name.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The type of the schema definition
        /// Default value is `TYPE_UNSPECIFIED`.
        /// Possible values are: `TYPE_UNSPECIFIED`, `PROTOCOL_BUFFER`, `AVRO`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SchemaArgs,
    ) -> SchemaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let definition_binding = args.definition.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:pubsub/schema:Schema".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definition".into(),
                    value: definition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SchemaResult {
            definition: o.get_field("definition"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            type_: o.get_field("type"),
        }
    }
}
