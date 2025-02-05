/// Cloud Firestore indexes enable simple and complex queries against documents in a database.
///  Both Firestore Native and Datastore Mode indexes are supported.
///  This resource manages composite indexes and not single field indexes.
///  To manage single field indexes, use the `gcp.firestore.Field` resource instead.
///
///
/// To get more information about Index, see:
///
/// * [API documentation](https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.collectionGroups.indexes)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/firestore/docs/query-data/indexing)
///
/// > **Warning:** This resource creates a Firestore Index on a project that already has
/// a Firestore database. If you haven't already created it, you may
/// create a `gcp.firestore.Database` resource and `location_id` set
/// to your chosen location. If you wish to use App Engine, you may
/// instead create a `gcp.appengine.Application` resource.
/// Your Firestore location will be the same as the App Engine location specified.
///
/// ## Example Usage
///
/// ### Firestore Index Basic
///
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: my-project-name
///       name: database-id
///       locationId: nam5
///       type: FIRESTORE_NATIVE
///       deleteProtectionState: DELETE_PROTECTION_DISABLED
///       deletionPolicy: DELETE
///   my-index:
///     type: gcp:firestore:Index
///     properties:
///       project: my-project-name
///       database: ${database.name}
///       collection: atestcollection
///       fields:
///         - fieldPath: name
///           order: ASCENDING
///         - fieldPath: description
///           order: DESCENDING
/// ```
/// ### Firestore Index Datastore Mode
///
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: my-project-name
///       name: database-id-dm
///       locationId: nam5
///       type: DATASTORE_MODE
///       deleteProtectionState: DELETE_PROTECTION_DISABLED
///       deletionPolicy: DELETE
///   my-index:
///     type: gcp:firestore:Index
///     properties:
///       project: my-project-name
///       database: ${database.name}
///       collection: atestcollection
///       queryScope: COLLECTION_RECURSIVE
///       apiScope: DATASTORE_MODE_API
///       fields:
///         - fieldPath: name
///           order: ASCENDING
///         - fieldPath: description
///           order: DESCENDING
/// ```
/// ### Firestore Index Vector
///
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: my-project-name
///       name: database-id-vector
///       locationId: nam5
///       type: FIRESTORE_NATIVE
///       deleteProtectionState: DELETE_PROTECTION_DISABLED
///       deletionPolicy: DELETE
///   my-index:
///     type: gcp:firestore:Index
///     properties:
///       project: my-project-name
///       database: ${database.name}
///       collection: atestcollection
///       fields:
///         - fieldPath: field_name
///           order: ASCENDING
///         - fieldPath: __name__
///           order: ASCENDING
///         - fieldPath: description
///           vectorConfig:
///             dimension: 128
///             flat: {}
/// ```
/// ### Firestore Index Name Descending
///
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: my-project-name
///       name: database-id
///       locationId: nam5
///       type: FIRESTORE_NATIVE
///       deleteProtectionState: DELETE_PROTECTION_DISABLED
///       deletionPolicy: DELETE
///   my-index:
///     type: gcp:firestore:Index
///     properties:
///       project: my-project-name
///       database: ${database.name}
///       collection: atestcollection
///       fields:
///         - fieldPath: __name__
///           order: DESCENDING
/// ```
///
/// ## Import
///
/// Index can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Index can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firestore/index:Index default {{name}}
/// ```
///
pub mod index {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IndexArgs {
        /// The API scope at which a query is run. Default value: "ANY_API" Possible values: ["ANY_API", "DATASTORE_MODE_API"]
        #[builder(into, default)]
        pub api_scope: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The collection being indexed.
        #[builder(into)]
        pub collection: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Firestore database id. Defaults to '"(default)"'.
        #[builder(into, default)]
        pub database: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The fields supported by this index. The last non-stored field entry is
        /// always for the field path `__name__`. If, on creation, `__name__` was not
        /// specified as the last field, it will be added automatically with the same
        /// direction as that of the last field defined. If the final field in a
        /// composite index is not directional, the `__name__` will be ordered
        /// `"ASCENDING"` (unless explicitly specified otherwise).
        /// Structure is documented below.
        #[builder(into)]
        pub fields: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::firestore::IndexField>,
        >,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The scope at which a query is run. Default value: "COLLECTION" Possible values: ["COLLECTION", "COLLECTION_GROUP",
        /// "COLLECTION_RECURSIVE"]
        #[builder(into, default)]
        pub query_scope: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IndexResult {
        /// The API scope at which a query is run. Default value: "ANY_API" Possible values: ["ANY_API", "DATASTORE_MODE_API"]
        pub api_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// The collection being indexed.
        pub collection: pulumi_wasm_rust::Output<String>,
        /// The Firestore database id. Defaults to '"(default)"'.
        pub database: pulumi_wasm_rust::Output<Option<String>>,
        /// The fields supported by this index. The last non-stored field entry is
        /// always for the field path `__name__`. If, on creation, `__name__` was not
        /// specified as the last field, it will be added automatically with the same
        /// direction as that of the last field defined. If the final field in a
        /// composite index is not directional, the `__name__` will be ordered
        /// `"ASCENDING"` (unless explicitly specified otherwise).
        /// Structure is documented below.
        pub fields: pulumi_wasm_rust::Output<
            Vec<super::super::types::firestore::IndexField>,
        >,
        /// A server defined name for this index. Format:
        /// `projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/indexes/{{server_generated_id}}`
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The scope at which a query is run. Default value: "COLLECTION" Possible values: ["COLLECTION", "COLLECTION_GROUP",
        /// "COLLECTION_RECURSIVE"]
        pub query_scope: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IndexArgs,
    ) -> IndexResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_scope_binding = args.api_scope.get_output(context).get_inner();
        let collection_binding = args.collection.get_output(context).get_inner();
        let database_binding = args.database.get_output(context).get_inner();
        let fields_binding = args.fields.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let query_scope_binding = args.query_scope.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firestore/index:Index".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiScope".into(),
                    value: &api_scope_binding,
                },
                register_interface::ObjectField {
                    name: "collection".into(),
                    value: &collection_binding,
                },
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "fields".into(),
                    value: &fields_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "queryScope".into(),
                    value: &query_scope_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IndexResult {
            api_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiScope"),
            ),
            collection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("collection"),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            fields: pulumi_wasm_rust::__private::into_domain(o.extract_field("fields")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            query_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queryScope"),
            ),
        }
    }
}
