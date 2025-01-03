/// Provides a CodePipeline Webhook.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   bar:
///     type: aws:codepipeline:Pipeline
///     properties:
///       name: tf-test-pipeline
///       roleArn: ${barAwsIamRole.arn}
///       artifactStores:
///         - location: ${barAwsS3Bucket.bucket}
///           type: S3
///           encryptionKey:
///             id: ${s3kmskey.arn}
///             type: KMS
///       stages:
///         - name: Source
///           actions:
///             - name: Source
///               category: Source
///               owner: ThirdParty
///               provider: GitHub
///               version: '1'
///               outputArtifacts:
///                 - test
///               configuration:
///                 Owner: my-organization
///                 Repo: test
///                 Branch: master
///         - name: Build
///           actions:
///             - name: Build
///               category: Build
///               owner: AWS
///               provider: CodeBuild
///               inputArtifacts:
///                 - test
///               version: '1'
///               configuration:
///                 ProjectName: test
///   barWebhook:
///     type: aws:codepipeline:Webhook
///     name: bar
///     properties:
///       name: test-webhook-github-bar
///       authentication: GITHUB_HMAC
///       targetAction: Source
///       targetPipeline: ${bar.name}
///       authenticationConfiguration:
///         secretToken: ${webhookSecret}
///       filters:
///         - jsonPath: $.ref
///           matchEquals: refs/heads/{Branch}
///   # Wire the CodePipeline webhook into a GitHub repository.
///   barRepositoryWebhook:
///     type: github:RepositoryWebhook
///     name: bar
///     properties:
///       repository: ${repo.name}
///       name: web
///       configuration:
///         url: ${barWebhook.url}
///         contentType: json
///         insecureSsl: true
///         secret: ${webhookSecret}
///       events:
///         - push
/// variables:
///   webhookSecret: super-secret
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodePipeline Webhooks using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codepipeline/webhook:Webhook example arn:aws:codepipeline:us-west-2:123456789012:webhook:example
/// ```
pub mod webhook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebhookArgs {
        /// The type of authentication  to use. One of `IP`, `GITHUB_HMAC`, or `UNAUTHENTICATED`.
        #[builder(into)]
        pub authentication: pulumi_wasm_rust::Output<String>,
        /// An `auth` block. Required for `IP` and `GITHUB_HMAC`. Auth blocks are documented below.
        #[builder(into, default)]
        pub authentication_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::codepipeline::WebhookAuthenticationConfiguration>,
        >,
        /// One or more `filter` blocks. Filter blocks are documented below.
        #[builder(into)]
        pub filters: pulumi_wasm_rust::Output<
            Vec<super::super::types::codepipeline::WebhookFilter>,
        >,
        /// The name of the webhook.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the action in a pipeline you want to connect to the webhook. The action must be from the source (first) stage of the pipeline.
        #[builder(into)]
        pub target_action: pulumi_wasm_rust::Output<String>,
        /// The name of the pipeline.
        #[builder(into)]
        pub target_pipeline: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WebhookResult {
        /// The CodePipeline webhook's ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The type of authentication  to use. One of `IP`, `GITHUB_HMAC`, or `UNAUTHENTICATED`.
        pub authentication: pulumi_wasm_rust::Output<String>,
        /// An `auth` block. Required for `IP` and `GITHUB_HMAC`. Auth blocks are documented below.
        pub authentication_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::codepipeline::WebhookAuthenticationConfiguration>,
        >,
        /// One or more `filter` blocks. Filter blocks are documented below.
        pub filters: pulumi_wasm_rust::Output<
            Vec<super::super::types::codepipeline::WebhookFilter>,
        >,
        /// The name of the webhook.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the action in a pipeline you want to connect to the webhook. The action must be from the source (first) stage of the pipeline.
        pub target_action: pulumi_wasm_rust::Output<String>,
        /// The name of the pipeline.
        pub target_pipeline: pulumi_wasm_rust::Output<String>,
        /// The CodePipeline webhook's URL. POST events to this endpoint to trigger the target.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WebhookArgs) -> WebhookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_binding = args.authentication.get_inner();
        let authentication_configuration_binding = args
            .authentication_configuration
            .get_inner();
        let filters_binding = args.filters.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_action_binding = args.target_action.get_inner();
        let target_pipeline_binding = args.target_pipeline.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codepipeline/webhook:Webhook".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationConfiguration".into(),
                    value: &authentication_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetAction".into(),
                    value: &target_action_binding,
                },
                register_interface::ObjectField {
                    name: "targetPipeline".into(),
                    value: &target_pipeline_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authentication".into(),
                },
                register_interface::ResultField {
                    name: "authenticationConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetAction".into(),
                },
                register_interface::ResultField {
                    name: "targetPipeline".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebhookResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authentication").unwrap(),
            ),
            authentication_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationConfiguration").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetAction").unwrap(),
            ),
            target_pipeline: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetPipeline").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
