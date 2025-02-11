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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod webhook {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebhookArgs {
        /// The type of authentication  to use. One of `IP`, `GITHUB_HMAC`, or `UNAUTHENTICATED`.
        #[builder(into)]
        pub authentication: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `auth` block. Required for `IP` and `GITHUB_HMAC`. Auth blocks are documented below.
        #[builder(into, default)]
        pub authentication_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codepipeline::WebhookAuthenticationConfiguration>,
        >,
        /// One or more `filter` blocks. Filter blocks are documented below.
        #[builder(into)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::codepipeline::WebhookFilter>,
        >,
        /// The name of the webhook.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the action in a pipeline you want to connect to the webhook. The action must be from the source (first) stage of the pipeline.
        #[builder(into)]
        pub target_action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the pipeline.
        #[builder(into)]
        pub target_pipeline: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebhookResult {
        /// The CodePipeline webhook's ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The type of authentication  to use. One of `IP`, `GITHUB_HMAC`, or `UNAUTHENTICATED`.
        pub authentication: pulumi_gestalt_rust::Output<String>,
        /// An `auth` block. Required for `IP` and `GITHUB_HMAC`. Auth blocks are documented below.
        pub authentication_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::codepipeline::WebhookAuthenticationConfiguration>,
        >,
        /// One or more `filter` blocks. Filter blocks are documented below.
        pub filters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::codepipeline::WebhookFilter>,
        >,
        /// The name of the webhook.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the action in a pipeline you want to connect to the webhook. The action must be from the source (first) stage of the pipeline.
        pub target_action: pulumi_gestalt_rust::Output<String>,
        /// The name of the pipeline.
        pub target_pipeline: pulumi_gestalt_rust::Output<String>,
        /// The CodePipeline webhook's URL. POST events to this endpoint to trigger the target.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebhookArgs,
    ) -> WebhookResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_binding = args.authentication.get_output(context);
        let authentication_configuration_binding = args
            .authentication_configuration
            .get_output(context);
        let filters_binding = args.filters.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_action_binding = args.target_action.get_output(context);
        let target_pipeline_binding = args.target_pipeline.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codepipeline/webhook:Webhook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationConfiguration".into(),
                    value: &authentication_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetAction".into(),
                    value: &target_action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetPipeline".into(),
                    value: &target_pipeline_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebhookResult {
            arn: o.get_field("arn"),
            authentication: o.get_field("authentication"),
            authentication_configuration: o.get_field("authenticationConfiguration"),
            filters: o.get_field("filters"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_action: o.get_field("targetAction"),
            target_pipeline: o.get_field("targetPipeline"),
            url: o.get_field("url"),
        }
    }
}
