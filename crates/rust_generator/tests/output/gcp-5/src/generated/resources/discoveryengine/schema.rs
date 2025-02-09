/// Schema defines the structure and layout of a type of document data.
///
///
/// To get more information about Schema, see:
///
/// * [API documentation](https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1/projects.locations.collections.dataStores.schemas)
/// * How-to Guides
///     * [Provide a schema for your data store](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema)
///
/// ## Example Usage
///
/// ### Discoveryengine Schema Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = schema::create(
///         "basic",
///         SchemaArgs::builder()
///             .data_store_id("${basicDataStore.dataStoreId}")
///             .json_schema(
///                 "{\"$schema\":\"https://json-schema.org/draft/2020-12/schema\",\"datetime_detection\":true,\"type\":\"object\",\"geolocation_detection\":true}",
///             )
///             .location("${basicDataStore.location}")
///             .schema_id("schema-id")
///             .build_struct(),
///     );
///     let basicDataStore = data_store::create(
///         "basicDataStore",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .create_advanced_site_search(false)
///             .data_store_id("data-store-id")
///             .display_name("tf-test-structured-datastore")
///             .industry_vertical("GENERIC")
///             .location("global")
///             .skip_default_schema_creation(true)
///             .solution_types(vec!["SOLUTION_TYPE_SEARCH",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Schema can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}/schemas/{{schema_id}}`
///
/// * `{{project}}/{{location}}/{{data_store_id}}/{{schema_id}}`
///
/// * `{{location}}/{{data_store_id}}/{{schema_id}}`
///
/// When using the `pulumi import` command, Schema can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/schema:Schema default projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}/schemas/{{schema_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/schema:Schema default {{project}}/{{location}}/{{data_store_id}}/{{schema_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/schema:Schema default {{location}}/{{data_store_id}}/{{schema_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod schema {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The unique id of the data store.
        #[builder(into)]
        pub data_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The JSON representation of the schema.
        #[builder(into, default)]
        pub json_schema: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique id of the schema.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub schema_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// The unique id of the data store.
        pub data_store_id: pulumi_gestalt_rust::Output<String>,
        /// The JSON representation of the schema.
        pub json_schema: pulumi_gestalt_rust::Output<Option<String>>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The unique full resource name of the schema. Values are of the format
        /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/schemas/{schema_id}`.
        /// This field must be a UTF-8 encoded string with a length limit of 1024
        /// characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The unique id of the schema.
        ///
        ///
        /// - - -
        pub schema_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SchemaArgs,
    ) -> SchemaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let data_store_id_binding_1 = args.data_store_id.get_output(context);
        let data_store_id_binding = data_store_id_binding_1.get_inner();
        let json_schema_binding_1 = args.json_schema.get_output(context);
        let json_schema_binding = json_schema_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let schema_id_binding_1 = args.schema_id.get_output(context);
        let schema_id_binding = schema_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:discoveryengine/schema:Schema".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataStoreId".into(),
                    value: &data_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "jsonSchema".into(),
                    value: &json_schema_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "schemaId".into(),
                    value: &schema_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SchemaResult {
            data_store_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataStoreId"),
            ),
            json_schema: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("jsonSchema"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            schema_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaId"),
            ),
        }
    }
}
