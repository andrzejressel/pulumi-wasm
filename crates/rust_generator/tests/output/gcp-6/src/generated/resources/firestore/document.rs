/// In Cloud Firestore, the unit of storage is the document. A document is a lightweight record
/// that contains fields, which map to values. Each document is identified by a name.
///
///
/// To get more information about Document, see:
///
/// * [API documentation](https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/firestore/docs/manage-data/add-data)
///
/// > **Warning:** This resource creates a Firestore Document on a project that already has
/// a Firestore database. If you haven't already created it, you may
/// create a `gcp.firestore.Database` resource with `type` set to
/// `"FIRESTORE_NATIVE"` and `location_id` set to your chosen location.
/// If you wish to use App Engine, you may instead create a
/// `gcp.appengine.Application` resource with `database_type` set to
/// `"CLOUD_FIRESTORE"`. Your Firestore location will be the same as
/// the App Engine location specified.
///
/// ## Example Usage
///
/// ### Firestore Document Basic
///
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:organizations:Project
///     properties:
///       projectId: project-id
///       name: project-id
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   wait60Seconds:
///     type: time:sleep
///     name: wait_60_seconds
///     properties:
///       createDuration: 60s
///     options:
///       dependsOn:
///         - ${project}
///   firestore:
///     type: gcp:projects:Service
///     properties:
///       project: ${project.projectId}
///       service: firestore.googleapis.com
///     options:
///       dependsOn:
///         - ${wait60Seconds}
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: ${project.projectId}
///       name: (default)
///       locationId: nam5
///       type: FIRESTORE_NATIVE
///     options:
///       dependsOn:
///         - ${firestore}
///   mydoc:
///     type: gcp:firestore:Document
///     properties:
///       project: ${project.projectId}
///       database: ${database.name}
///       collection: somenewcollection
///       documentId: my-doc-id
///       fields: '{"something":{"mapValue":{"fields":{"akey":{"stringValue":"avalue"}}}}}'
/// ```
/// ### Firestore Document Nested Document
///
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:organizations:Project
///     properties:
///       projectId: project-id
///       name: project-id
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   wait60Seconds:
///     type: time:sleep
///     name: wait_60_seconds
///     properties:
///       createDuration: 60s
///     options:
///       dependsOn:
///         - ${project}
///   firestore:
///     type: gcp:projects:Service
///     properties:
///       project: ${project.projectId}
///       service: firestore.googleapis.com
///     options:
///       dependsOn:
///         - ${wait60Seconds}
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: ${project.projectId}
///       name: (default)
///       locationId: nam5
///       type: FIRESTORE_NATIVE
///     options:
///       dependsOn:
///         - ${firestore}
///   mydoc:
///     type: gcp:firestore:Document
///     properties:
///       project: ${project.projectId}
///       database: ${database.name}
///       collection: somenewcollection
///       documentId: my-doc-id
///       fields: '{"something":{"mapValue":{"fields":{"akey":{"stringValue":"avalue"}}}}}'
///   subDocument:
///     type: gcp:firestore:Document
///     name: sub_document
///     properties:
///       project: ${project.projectId}
///       database: ${database.name}
///       collection: ${mydoc.path}/subdocs
///       documentId: bitcoinkey
///       fields: '{"something":{"mapValue":{"fields":{"ayo":{"stringValue":"val2"}}}}}'
///   subSubDocument:
///     type: gcp:firestore:Document
///     name: sub_sub_document
///     properties:
///       project: ${project.projectId}
///       database: ${database.name}
///       collection: ${subDocument.path}/subsubdocs
///       documentId: asecret
///       fields: '{"something":{"mapValue":{"fields":{"secret":{"stringValue":"hithere"}}}}}'
/// ```
///
/// ## Import
///
/// Document can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Document can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firestore/document:Document default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod document {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DocumentArgs {
        /// The collection ID, relative to database. For example: chatrooms or chatrooms/my-document/private-messages.
        #[builder(into)]
        pub collection: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Firestore database id. Defaults to `"(default)"`.
        #[builder(into, default)]
        pub database: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The client-assigned document ID to use for this document during creation.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub document_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The document's [fields](https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents) formated as a json string.
        #[builder(into)]
        pub fields: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DocumentResult {
        /// The collection ID, relative to database. For example: chatrooms or chatrooms/my-document/private-messages.
        pub collection: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The Firestore database id. Defaults to `"(default)"`.
        pub database: pulumi_gestalt_rust::Output<Option<String>>,
        /// The client-assigned document ID to use for this document during creation.
        ///
        ///
        /// - - -
        pub document_id: pulumi_gestalt_rust::Output<String>,
        /// The document's [fields](https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents) formated as a json string.
        pub fields: pulumi_gestalt_rust::Output<String>,
        /// A server defined name for this document. Format:
        /// `projects/{{project_id}}/databases/{{database_id}}/documents/{{path}}/{{document_id}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A relative path to the collection this document exists within
        pub path: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Last update timestamp in RFC3339 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DocumentArgs,
    ) -> DocumentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let collection_binding = args.collection.get_output(context).get_inner();
        let database_binding = args.database.get_output(context).get_inner();
        let document_id_binding = args.document_id.get_output(context).get_inner();
        let fields_binding = args.fields.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firestore/document:Document".into(),
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
                    name: "documentId".into(),
                    value: &document_id_binding,
                },
                register_interface::ObjectField {
                    name: "fields".into(),
                    value: &fields_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DocumentResult {
            collection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("collection"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            database: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            document_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("documentId"),
            ),
            fields: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fields"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
