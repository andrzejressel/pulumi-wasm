/// Represents a single field in the database.
/// Fields are grouped by their "Collection Group", which represent all collections
/// in the database with the same id.
///
///
/// To get more information about Field, see:
///
/// * [API documentation](https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.collectionGroups.fields)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/firestore/docs/query-data/indexing)
///
/// > **Warning:** This resource creates a Firestore Single Field override on a project that
///  already has a Firestore database. If you haven't already created it, you may
/// create a `gcp.firestore.Database` resource with `location_id` set to your
/// chosen location.
///
/// ## Example Usage
///
/// ### Firestore Field Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = field::create(
///         "basic",
///         FieldArgs::builder()
///             .collection("chatrooms__8493")
///             .database("${database.name}")
///             .field("basic")
///             .index_config(
///                 FieldIndexConfig::builder()
///                     .indexes(
///                         vec![
///                             FieldIndexConfigIndex::builder().order("ASCENDING")
///                             .queryScope("COLLECTION_GROUP").build_struct(),
///                             FieldIndexConfigIndex::builder().arrayConfig("CONTAINS")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .delete_protection_state("DELETE_PROTECTION_ENABLED")
///             .deletion_policy("DELETE")
///             .location_id("nam5")
///             .name("database-id")
///             .project("my-project-name")
///             .type_("FIRESTORE_NATIVE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firestore Field Timestamp
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .delete_protection_state("DELETE_PROTECTION_ENABLED")
///             .deletion_policy("DELETE")
///             .location_id("nam5")
///             .name("database-id")
///             .project("my-project-name")
///             .type_("FIRESTORE_NATIVE")
///             .build_struct(),
///     );
///     let timestamp = field::create(
///         "timestamp",
///         FieldArgs::builder()
///             .collection("chatrooms")
///             .database("${database.name}")
///             .field("timestamp")
///             .index_config(FieldIndexConfig::builder().build_struct())
///             .project("my-project-name")
///             .ttl_config(FieldTtlConfig::builder().build_struct())
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firestore Field Match Override
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .delete_protection_state("DELETE_PROTECTION_ENABLED")
///             .deletion_policy("DELETE")
///             .location_id("nam5")
///             .name("database-id")
///             .project("my-project-name")
///             .type_("FIRESTORE_NATIVE")
///             .build_struct(),
///     );
///     let matchOverride = field::create(
///         "matchOverride",
///         FieldArgs::builder()
///             .collection("chatrooms__9106")
///             .database("${database.name}")
///             .field("field_with_same_configuration_as_ancestor")
///             .index_config(
///                 FieldIndexConfig::builder()
///                     .indexes(
///                         vec![
///                             FieldIndexConfigIndex::builder().order("ASCENDING")
///                             .build_struct(), FieldIndexConfigIndex::builder()
///                             .order("DESCENDING").build_struct(),
///                             FieldIndexConfigIndex::builder().arrayConfig("CONTAINS")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Field can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Field can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firestore/field:Field default {{name}}
/// ```
///
pub mod field {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FieldArgs {
        /// The id of the collection group to configure.
        #[builder(into)]
        pub collection: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Firestore database id. Defaults to `"(default)"`.
        #[builder(into, default)]
        pub database: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The id of the field to configure.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub field: pulumi_wasm_rust::InputOrOutput<String>,
        /// The single field index configuration for this field.
        /// Creating an index configuration for this field will override any inherited configuration with the
        /// indexes specified. Configuring the index configuration with an empty block disables all indexes on
        /// the field.
        /// Structure is documented below.
        #[builder(into, default)]
        pub index_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::firestore::FieldIndexConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The TTL configuration for this Field. If set to an empty block (i.e. `ttl_config {}`), a TTL policy is configured based on the field. If unset, a TTL policy is not configured (or will be disabled upon updating the resource).
        /// Structure is documented below.
        #[builder(into, default)]
        pub ttl_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::firestore::FieldTtlConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct FieldResult {
        /// The id of the collection group to configure.
        pub collection: pulumi_wasm_rust::Output<String>,
        /// The Firestore database id. Defaults to `"(default)"`.
        pub database: pulumi_wasm_rust::Output<Option<String>>,
        /// The id of the field to configure.
        ///
        ///
        /// - - -
        pub field: pulumi_wasm_rust::Output<String>,
        /// The single field index configuration for this field.
        /// Creating an index configuration for this field will override any inherited configuration with the
        /// indexes specified. Configuring the index configuration with an empty block disables all indexes on
        /// the field.
        /// Structure is documented below.
        pub index_config: pulumi_wasm_rust::Output<
            Option<super::super::types::firestore::FieldIndexConfig>,
        >,
        /// The name of this field. Format:
        /// `projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/fields/{{field}}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The TTL configuration for this Field. If set to an empty block (i.e. `ttl_config {}`), a TTL policy is configured based on the field. If unset, a TTL policy is not configured (or will be disabled upon updating the resource).
        /// Structure is documented below.
        pub ttl_config: pulumi_wasm_rust::Output<
            Option<super::super::types::firestore::FieldTtlConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FieldArgs,
    ) -> FieldResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let collection_binding = args.collection.get_output(context).get_inner();
        let database_binding = args.database.get_output(context).get_inner();
        let field_binding = args.field.get_output(context).get_inner();
        let index_config_binding = args.index_config.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let ttl_config_binding = args.ttl_config.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firestore/field:Field".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "collection".into(),
                    value: &collection_binding,
                },
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "field".into(),
                    value: &field_binding,
                },
                register_interface::ObjectField {
                    name: "indexConfig".into(),
                    value: &index_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "ttlConfig".into(),
                    value: &ttl_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FieldResult {
            collection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("collection"),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            field: pulumi_wasm_rust::__private::into_domain(o.extract_field("field")),
            index_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("indexConfig"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            ttl_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ttlConfig"),
            ),
        }
    }
}
