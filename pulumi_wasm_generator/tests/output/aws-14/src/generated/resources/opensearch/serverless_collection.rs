/// Resource for managing an AWS OpenSearch Serverless Collection.
///
/// > **NOTE:** An `aws.opensearch.ServerlessCollection` cannot be created without having an applicable encryption security policy. Use the `depends_on` meta-argument to define this dependency.
///
/// > **NOTE:** An `aws.opensearch.ServerlessCollection` is not accessible without configuring an applicable network security policy. Data cannot be accessed without configuring an applicable data access policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessSecurityPolicy
///     properties:
///       name: example
///       type: encryption
///       policy:
///         fn::toJSON:
///           Rules:
///             - Resource:
///                 - collection/example
///               ResourceType: collection
///           AWSOwnedKey: true
///   exampleServerlessCollection:
///     type: aws:opensearch:ServerlessCollection
///     name: example
///     properties:
///       name: example
///     options:
///       dependsOn:
///         - ${example}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearchServerless Collection using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/serverlessCollection:ServerlessCollection example example
/// ```
pub mod serverless_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessCollectionArgs {
        /// Description of the collection.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the collection.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether standby replicas should be used for a collection. One of `ENABLED` or `DISABLED`. Defaults to `ENABLED`.
        #[builder(into, default)]
        pub standby_replicas: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::ServerlessCollectionTimeouts>,
        >,
        /// Type of collection. One of `SEARCH`, `TIMESERIES`, or `VECTORSEARCH`. Defaults to `TIMESERIES`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServerlessCollectionResult {
        /// Amazon Resource Name (ARN) of the collection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Collection-specific endpoint used to submit index, search, and data upload requests to an OpenSearch Serverless collection.
        pub collection_endpoint: pulumi_wasm_rust::Output<String>,
        /// Collection-specific endpoint used to access OpenSearch Dashboards.
        pub dashboard_endpoint: pulumi_wasm_rust::Output<String>,
        /// Description of the collection.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the Amazon Web Services KMS key used to encrypt the collection.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the collection.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether standby replicas should be used for a collection. One of `ENABLED` or `DISABLED`. Defaults to `ENABLED`.
        pub standby_replicas: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::ServerlessCollectionTimeouts>,
        >,
        /// Type of collection. One of `SEARCH`, `TIMESERIES`, or `VECTORSEARCH`. Defaults to `TIMESERIES`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ServerlessCollectionArgs,
    ) -> ServerlessCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let standby_replicas_binding = args.standby_replicas.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessCollection:ServerlessCollection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "standbyReplicas".into(),
                    value: &standby_replicas_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
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
                    name: "dashboardEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
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
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServerlessCollectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            collection_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collectionEndpoint").unwrap(),
            ),
            dashboard_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardEndpoint").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
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
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
