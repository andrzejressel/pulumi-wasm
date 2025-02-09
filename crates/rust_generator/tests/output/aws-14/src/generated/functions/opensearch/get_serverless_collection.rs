#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_serverless_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessCollectionArgs {
        /// ID of the collection. Either `id` or `name` must be provided.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the collection. Either `name` or `id` must be provided.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServerlessCollectionResult {
        /// Amazon Resource Name (ARN) of the collection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Collection-specific endpoint used to submit index, search, and data upload requests to an OpenSearch Serverless collection.
        pub collection_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Date the Collection was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Collection-specific endpoint used to access OpenSearch Dashboards.
        pub dashboard_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Description of the collection.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// A failure code associated with the collection.
        pub failure_code: pulumi_gestalt_rust::Output<String>,
        pub failure_message: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the Amazon Web Services KMS key used to encrypt the collection.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Date the Collection was last modified.
        pub last_modified_date: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether standby replicas should be used for a collection.
        pub standby_replicas: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the collection.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of collection.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServerlessCollectionArgs,
    ) -> GetServerlessCollectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding_1 = args.id.get_output(context);
        let id_binding = id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:opensearch/getServerlessCollection:getServerlessCollection"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServerlessCollectionResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            collection_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("collectionEndpoint"),
            ),
            created_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            dashboard_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dashboardEndpoint"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            failure_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("failureCode"),
            ),
            failure_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("failureMessage"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            last_modified_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedDate"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            standby_replicas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("standbyReplicas"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
