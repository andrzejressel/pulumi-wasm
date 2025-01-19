pub mod get_serverless_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessCollectionArgs {
        /// ID of the collection. Either `id` or `name` must be provided.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the collection. Either `name` or `id` must be provided.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServerlessCollectionResult {
        /// Amazon Resource Name (ARN) of the collection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Collection-specific endpoint used to submit index, search, and data upload requests to an OpenSearch Serverless collection.
        pub collection_endpoint: pulumi_wasm_rust::Output<String>,
        /// Date the Collection was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Collection-specific endpoint used to access OpenSearch Dashboards.
        pub dashboard_endpoint: pulumi_wasm_rust::Output<String>,
        /// Description of the collection.
        pub description: pulumi_wasm_rust::Output<String>,
        /// A failure code associated with the collection.
        pub failure_code: pulumi_wasm_rust::Output<String>,
        pub failure_message: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the Amazon Web Services KMS key used to encrypt the collection.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// Date the Collection was last modified.
        pub last_modified_date: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether standby replicas should be used for a collection.
        pub standby_replicas: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the collection.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of collection.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServerlessCollectionArgs) -> GetServerlessCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let name_binding = args.name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "collectionEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "dashboardEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "failureCode".into(),
                },
                register_interface::ResultField {
                    name: "failureMessage".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedDate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "standbyReplicas".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServerlessCollectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            collection_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collectionEndpoint").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            dashboard_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardEndpoint").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            failure_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failureCode").unwrap(),
            ),
            failure_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failureMessage").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            last_modified_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedDate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            standby_replicas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("standbyReplicas").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
