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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod serverless_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessCollectionArgs {
        /// Description of the collection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the collection.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether standby replicas should be used for a collection. One of `ENABLED` or `DISABLED`. Defaults to `ENABLED`.
        #[builder(into, default)]
        pub standby_replicas: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::opensearch::ServerlessCollectionTimeouts>,
        >,
        /// Type of collection. One of `SEARCH`, `TIMESERIES`, or `VECTORSEARCH`. Defaults to `TIMESERIES`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServerlessCollectionResult {
        /// Amazon Resource Name (ARN) of the collection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Collection-specific endpoint used to submit index, search, and data upload requests to an OpenSearch Serverless collection.
        pub collection_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Collection-specific endpoint used to access OpenSearch Dashboards.
        pub dashboard_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Description of the collection.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the Amazon Web Services KMS key used to encrypt the collection.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the collection.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether standby replicas should be used for a collection. One of `ENABLED` or `DISABLED`. Defaults to `ENABLED`.
        pub standby_replicas: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::opensearch::ServerlessCollectionTimeouts>,
        >,
        /// Type of collection. One of `SEARCH`, `TIMESERIES`, or `VECTORSEARCH`. Defaults to `TIMESERIES`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerlessCollectionArgs,
    ) -> ServerlessCollectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let standby_replicas_binding = args.standby_replicas.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessCollection:ServerlessCollection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "standbyReplicas".into(),
                    value: standby_replicas_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServerlessCollectionResult {
            arn: o.get_field("arn"),
            collection_endpoint: o.get_field("collectionEndpoint"),
            dashboard_endpoint: o.get_field("dashboardEndpoint"),
            description: o.get_field("description"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            name: o.get_field("name"),
            standby_replicas: o.get_field("standbyReplicas"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            type_: o.get_field("type"),
        }
    }
}
