/// Provides a CodePipeline.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   codepipeline:
///     type: aws:codepipeline:Pipeline
///     properties:
///       name: tf-test-pipeline
///       roleArn: ${codepipelineRole.arn}
///       artifactStores:
///         - location: ${codepipelineBucket.bucket}
///           type: S3
///           encryptionKey:
///             id: ${s3kmskey.arn}
///             type: KMS
///       stages:
///         - name: Source
///           actions:
///             - name: Source
///               category: Source
///               owner: AWS
///               provider: CodeStarSourceConnection
///               version: '1'
///               outputArtifacts:
///                 - source_output
///               configuration:
///                 ConnectionArn: ${example.arn}
///                 FullRepositoryId: my-organization/example
///                 BranchName: main
///         - name: Build
///           actions:
///             - name: Build
///               category: Build
///               owner: AWS
///               provider: CodeBuild
///               inputArtifacts:
///                 - source_output
///               outputArtifacts:
///                 - build_output
///               version: '1'
///               configuration:
///                 ProjectName: test
///         - name: Deploy
///           actions:
///             - name: Deploy
///               category: Deploy
///               owner: AWS
///               provider: CloudFormation
///               inputArtifacts:
///                 - build_output
///               version: '1'
///               configuration:
///                 ActionMode: REPLACE_ON_FAILURE
///                 Capabilities: CAPABILITY_AUTO_EXPAND,CAPABILITY_IAM
///                 OutputFileName: CreateStackOutput.json
///                 StackName: MyStack
///                 TemplatePath: build_output::sam-templated.yaml
///   example:
///     type: aws:codestarconnections:Connection
///     properties:
///       name: example-connection
///       providerType: GitHub
///   codepipelineBucket:
///     type: aws:s3:BucketV2
///     name: codepipeline_bucket
///     properties:
///       bucket: test-bucket
///   codepipelineBucketPab:
///     type: aws:s3:BucketPublicAccessBlock
///     name: codepipeline_bucket_pab
///     properties:
///       bucket: ${codepipelineBucket.id}
///       blockPublicAcls: true
///       blockPublicPolicy: true
///       ignorePublicAcls: true
///       restrictPublicBuckets: true
///   codepipelineRole:
///     type: aws:iam:Role
///     name: codepipeline_role
///     properties:
///       name: test-role
///       assumeRolePolicy: ${assumeRole.json}
///   codepipelinePolicyRolePolicy:
///     type: aws:iam:RolePolicy
///     name: codepipeline_policy
///     properties:
///       name: codepipeline_policy
///       role: ${codepipelineRole.id}
///       policy: ${codepipelinePolicy.json}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - codepipeline.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   codepipelinePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - s3:GetObject
///               - s3:GetObjectVersion
///               - s3:GetBucketVersioning
///               - s3:PutObjectAcl
///               - s3:PutObject
///             resources:
///               - ${codepipelineBucket.arn}
///               - ${codepipelineBucket.arn}/*
///           - effect: Allow
///             actions:
///               - codestar-connections:UseConnection
///             resources:
///               - ${example.arn}
///           - effect: Allow
///             actions:
///               - codebuild:BatchGetBuilds
///               - codebuild:StartBuild
///             resources:
///               - '*'
///   s3kmskey:
///     fn::invoke:
///       function: aws:kms:getAlias
///       arguments:
///         name: alias/myKmsKey
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodePipelines using the name. For example:
///
/// ```sh
/// $ pulumi import aws:codepipeline/pipeline:Pipeline foo example
/// ```
pub mod pipeline {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineArgs {
        /// One or more artifact_store blocks. Artifact stores are documented below.
        #[builder(into)]
        pub artifact_stores: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::codepipeline::PipelineArtifactStore>,
        >,
        /// The method that the pipeline will use to handle multiple executions. The default mode is `SUPERSEDED`. For value values, refer to the [AWS documentation](https://docs.aws.amazon.com/codepipeline/latest/APIReference/API_PipelineDeclaration.html#CodePipeline-Type-PipelineDeclaration-executionMode).
        ///
        /// **Note:** `QUEUED` or `PARALLEL` mode can only be used with V2 pipelines.
        #[builder(into, default)]
        pub execution_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the pipeline.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Type of the pipeline. Possible values are: `V1` and `V2`. Default value is `V1`.
        #[builder(into, default)]
        pub pipeline_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A service role Amazon Resource Name (ARN) that grants AWS CodePipeline permission to make calls to AWS services on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// A stage block. Stages are documented below.
        #[builder(into)]
        pub stages: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::codepipeline::PipelineStage>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A trigger block. Valid only when `pipeline_type` is `V2`. Triggers are documented below.
        #[builder(into, default)]
        pub triggers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::codepipeline::PipelineTrigger>>,
        >,
        /// A pipeline-level variable block. Valid only when `pipeline_type` is `V2`. Variable are documented below.
        #[builder(into, default)]
        pub variables: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::codepipeline::PipelineVariable>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineResult {
        /// The codepipeline ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// One or more artifact_store blocks. Artifact stores are documented below.
        pub artifact_stores: pulumi_wasm_rust::Output<
            Vec<super::super::types::codepipeline::PipelineArtifactStore>,
        >,
        /// The method that the pipeline will use to handle multiple executions. The default mode is `SUPERSEDED`. For value values, refer to the [AWS documentation](https://docs.aws.amazon.com/codepipeline/latest/APIReference/API_PipelineDeclaration.html#CodePipeline-Type-PipelineDeclaration-executionMode).
        ///
        /// **Note:** `QUEUED` or `PARALLEL` mode can only be used with V2 pipelines.
        pub execution_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the pipeline.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Type of the pipeline. Possible values are: `V1` and `V2`. Default value is `V1`.
        pub pipeline_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A service role Amazon Resource Name (ARN) that grants AWS CodePipeline permission to make calls to AWS services on your behalf.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A stage block. Stages are documented below.
        pub stages: pulumi_wasm_rust::Output<
            Vec<super::super::types::codepipeline::PipelineStage>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A trigger block. Valid only when `pipeline_type` is `V2`. Triggers are documented below.
        pub triggers: pulumi_wasm_rust::Output<
            Vec<super::super::types::codepipeline::PipelineTrigger>,
        >,
        /// A pipeline-level variable block. Valid only when `pipeline_type` is `V2`. Variable are documented below.
        pub variables: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codepipeline::PipelineVariable>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PipelineArgs,
    ) -> PipelineResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let artifact_stores_binding = args
            .artifact_stores
            .get_output(context)
            .get_inner();
        let execution_mode_binding = args.execution_mode.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let pipeline_type_binding = args.pipeline_type.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let stages_binding = args.stages.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let triggers_binding = args.triggers.get_output(context).get_inner();
        let variables_binding = args.variables.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codepipeline/pipeline:Pipeline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "artifactStores".into(),
                    value: &artifact_stores_binding,
                },
                register_interface::ObjectField {
                    name: "executionMode".into(),
                    value: &execution_mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineType".into(),
                    value: &pipeline_type_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "stages".into(),
                    value: &stages_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
                register_interface::ObjectField {
                    name: "variables".into(),
                    value: &variables_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PipelineResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            artifact_stores: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("artifactStores"),
            ),
            execution_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("executionMode"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            pipeline_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pipelineType"),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            stages: pulumi_wasm_rust::__private::into_domain(o.extract_field("stages")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("triggers"),
            ),
            variables: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("variables"),
            ),
        }
    }
}
