/// Provides a CodeBuild Project resource. See also the `aws.codebuild.Webhook` resource, which manages the webhook to the source (e.g., the "rebuild every time a code change is pushed" option in the CodeBuild web console).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucket: example
///   exampleBucketAclV2:
///     type: aws:s3:BucketAclV2
///     name: example
///     properties:
///       bucket: ${exampleBucketV2.id}
///       acl: private
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example
///       assumeRolePolicy: ${assumeRole.json}
///   exampleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: example
///     properties:
///       role: ${exampleRole.name}
///       policy: ${example.json}
///   exampleProject:
///     type: aws:codebuild:Project
///     name: example
///     properties:
///       name: test-project
///       description: test_codebuild_project
///       buildTimeout: 5
///       serviceRole: ${exampleRole.arn}
///       artifacts:
///         type: NO_ARTIFACTS
///       cache:
///         type: S3
///         location: ${exampleBucketV2.bucket}
///       environment:
///         computeType: BUILD_GENERAL1_SMALL
///         image: aws/codebuild/amazonlinux2-x86_64-standard:4.0
///         type: LINUX_CONTAINER
///         imagePullCredentialsType: CODEBUILD
///         environmentVariables:
///           - name: SOME_KEY1
///             value: SOME_VALUE1
///           - name: SOME_KEY2
///             value: SOME_VALUE2
///             type: PARAMETER_STORE
///       logsConfig:
///         cloudwatchLogs:
///           groupName: log-group
///           streamName: log-stream
///         s3Logs:
///           status: ENABLED
///           location: ${exampleBucketV2.id}/build-log
///       source:
///         type: GITHUB
///         location: https://github.com/mitchellh/packer.git
///         gitCloneDepth: 1
///         gitSubmodulesConfig:
///           fetchSubmodules: true
///       sourceVersion: master
///       vpcConfig:
///         vpcId: ${exampleAwsVpc.id}
///         subnets:
///           - ${example1.id}
///           - ${example2.id}
///         securityGroupIds:
///           - ${example1AwsSecurityGroup.id}
///           - ${example2AwsSecurityGroup.id}
///       tags:
///         Environment: Test
///   project-with-cache:
///     type: aws:codebuild:Project
///     properties:
///       name: test-project-cache
///       description: test_codebuild_project_cache
///       buildTimeout: 5
///       queuedTimeout: 5
///       serviceRole: ${exampleRole.arn}
///       artifacts:
///         type: NO_ARTIFACTS
///       cache:
///         type: LOCAL
///         modes:
///           - LOCAL_DOCKER_LAYER_CACHE
///           - LOCAL_SOURCE_CACHE
///       environment:
///         computeType: BUILD_GENERAL1_SMALL
///         image: aws/codebuild/amazonlinux2-x86_64-standard:4.0
///         type: LINUX_CONTAINER
///         imagePullCredentialsType: CODEBUILD
///         environmentVariables:
///           - name: SOME_KEY1
///             value: SOME_VALUE1
///       source:
///         type: GITHUB
///         location: https://github.com/mitchellh/packer.git
///         gitCloneDepth: 1
///       tags:
///         Environment: Test
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
///                   - codebuild.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - logs:CreateLogGroup
///               - logs:CreateLogStream
///               - logs:PutLogEvents
///             resources:
///               - '*'
///           - effect: Allow
///             actions:
///               - ec2:CreateNetworkInterface
///               - ec2:DescribeDhcpOptions
///               - ec2:DescribeNetworkInterfaces
///               - ec2:DeleteNetworkInterface
///               - ec2:DescribeSubnets
///               - ec2:DescribeSecurityGroups
///               - ec2:DescribeVpcs
///             resources:
///               - '*'
///           - effect: Allow
///             actions:
///               - ec2:CreateNetworkInterfacePermission
///             resources:
///               - arn:aws:ec2:us-east-1:123456789012:network-interface/*
///             conditions:
///               - test: StringEquals
///                 variable: ec2:Subnet
///                 values:
///                   - ${example1.arn}
///                   - ${example2.arn}
///               - test: StringEquals
///                 variable: ec2:AuthorizedService
///                 values:
///                   - codebuild.amazonaws.com
///           - effect: Allow
///             actions:
///               - s3:*
///             resources:
///               - ${exampleBucketV2.arn}
///               - ${exampleBucketV2.arn}/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeBuild Project using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:codebuild/project:Project name project-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// Configuration block. Detailed below.
        #[builder(into)]
        pub artifacts: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::codebuild::ProjectArtifacts,
        >,
        /// Generates a publicly-accessible URL for the projects build badge. Available as `badge_url` attribute when enabled.
        #[builder(into, default)]
        pub badge_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Defines the batch build options for the project.
        #[builder(into, default)]
        pub build_batch_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codebuild::ProjectBuildBatchConfig>,
        >,
        /// Number of minutes, from 5 to 2160 (36 hours), for AWS CodeBuild to wait until timing out any related build that does not get marked as completed. The default is 60 minutes. The `build_timeout` property is not available on the `Lambda` compute type.
        #[builder(into, default)]
        pub build_timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub cache: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codebuild::ProjectCache>,
        >,
        /// Specify a maximum number of concurrent builds for the project. The value specified must be greater than 0 and less than the account concurrent running builds limit.
        #[builder(into, default)]
        pub concurrent_build_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Short description of the project.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build project's build output artifacts.
        #[builder(into, default)]
        pub encryption_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block. Detailed below.
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::codebuild::ProjectEnvironment,
        >,
        /// A set of file system locations to mount inside the build. File system locations are documented below.
        #[builder(into, default)]
        pub file_system_locations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codebuild::ProjectFileSystemLocation>>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub logs_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codebuild::ProjectLogsConfig>,
        >,
        /// Project's name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the visibility of the project's builds. Possible values are: `PUBLIC_READ` and `PRIVATE`. Default value is `PRIVATE`.
        #[builder(into, default)]
        pub project_visibility: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of minutes, from 5 to 480 (8 hours), a build is allowed to be queued before it times out. The default is 8 hours. The `queued_timeout` property is not available on the `Lambda` compute type.
        #[builder(into, default)]
        pub queued_timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ARN of the IAM role that enables CodeBuild to access the CloudWatch Logs and Amazon S3 artifacts for the project's builds in order to display them publicly. Only applicable if `project_visibility` is `PUBLIC_READ`.
        #[builder(into, default)]
        pub resource_access_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub secondary_artifacts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codebuild::ProjectSecondaryArtifact>>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub secondary_source_versions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codebuild::ProjectSecondarySourceVersion>>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub secondary_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codebuild::ProjectSecondarySource>>,
        >,
        /// Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.
        #[builder(into)]
        pub service_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::codebuild::ProjectSource,
        >,
        /// Version of the build input to be built for this project. If not specified, the latest version is used.
        #[builder(into, default)]
        pub source_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codebuild::ProjectVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// ARN of the CodeBuild project.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block. Detailed below.
        pub artifacts: pulumi_gestalt_rust::Output<
            super::super::types::codebuild::ProjectArtifacts,
        >,
        /// Generates a publicly-accessible URL for the projects build badge. Available as `badge_url` attribute when enabled.
        pub badge_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// URL of the build badge when `badge_enabled` is enabled.
        pub badge_url: pulumi_gestalt_rust::Output<String>,
        /// Defines the batch build options for the project.
        pub build_batch_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::codebuild::ProjectBuildBatchConfig>,
        >,
        /// Number of minutes, from 5 to 2160 (36 hours), for AWS CodeBuild to wait until timing out any related build that does not get marked as completed. The default is 60 minutes. The `build_timeout` property is not available on the `Lambda` compute type.
        pub build_timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Configuration block. Detailed below.
        pub cache: pulumi_gestalt_rust::Output<
            Option<super::super::types::codebuild::ProjectCache>,
        >,
        /// Specify a maximum number of concurrent builds for the project. The value specified must be greater than 0 and less than the account concurrent running builds limit.
        pub concurrent_build_limit: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Short description of the project.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build project's build output artifacts.
        pub encryption_key: pulumi_gestalt_rust::Output<String>,
        /// Configuration block. Detailed below.
        pub environment: pulumi_gestalt_rust::Output<
            super::super::types::codebuild::ProjectEnvironment,
        >,
        /// A set of file system locations to mount inside the build. File system locations are documented below.
        pub file_system_locations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codebuild::ProjectFileSystemLocation>>,
        >,
        /// Configuration block. Detailed below.
        pub logs_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::codebuild::ProjectLogsConfig>,
        >,
        /// Project's name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the visibility of the project's builds. Possible values are: `PUBLIC_READ` and `PRIVATE`. Default value is `PRIVATE`.
        pub project_visibility: pulumi_gestalt_rust::Output<Option<String>>,
        /// The project identifier used with the public build APIs.
        pub public_project_alias: pulumi_gestalt_rust::Output<String>,
        /// Number of minutes, from 5 to 480 (8 hours), a build is allowed to be queued before it times out. The default is 8 hours. The `queued_timeout` property is not available on the `Lambda` compute type.
        pub queued_timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ARN of the IAM role that enables CodeBuild to access the CloudWatch Logs and Amazon S3 artifacts for the project's builds in order to display them publicly. Only applicable if `project_visibility` is `PUBLIC_READ`.
        pub resource_access_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        pub secondary_artifacts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codebuild::ProjectSecondaryArtifact>>,
        >,
        /// Configuration block. Detailed below.
        pub secondary_source_versions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codebuild::ProjectSecondarySourceVersion>>,
        >,
        /// Configuration block. Detailed below.
        pub secondary_sources: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codebuild::ProjectSecondarySource>>,
        >,
        /// Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.
        pub service_role: pulumi_gestalt_rust::Output<String>,
        /// Configuration block. Detailed below.
        ///
        /// The following arguments are optional:
        pub source: pulumi_gestalt_rust::Output<
            super::super::types::codebuild::ProjectSource,
        >,
        /// Version of the build input to be built for this project. If not specified, the latest version is used.
        pub source_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block. Detailed below.
        pub vpc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::codebuild::ProjectVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let artifacts_binding = args.artifacts.get_output(context);
        let badge_enabled_binding = args.badge_enabled.get_output(context);
        let build_batch_config_binding = args.build_batch_config.get_output(context);
        let build_timeout_binding = args.build_timeout.get_output(context);
        let cache_binding = args.cache.get_output(context);
        let concurrent_build_limit_binding = args
            .concurrent_build_limit
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let encryption_key_binding = args.encryption_key.get_output(context);
        let environment_binding = args.environment.get_output(context);
        let file_system_locations_binding = args
            .file_system_locations
            .get_output(context);
        let logs_config_binding = args.logs_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_visibility_binding = args.project_visibility.get_output(context);
        let queued_timeout_binding = args.queued_timeout.get_output(context);
        let resource_access_role_binding = args.resource_access_role.get_output(context);
        let secondary_artifacts_binding = args.secondary_artifacts.get_output(context);
        let secondary_source_versions_binding = args
            .secondary_source_versions
            .get_output(context);
        let secondary_sources_binding = args.secondary_sources.get_output(context);
        let service_role_binding = args.service_role.get_output(context);
        let source_binding = args.source.get_output(context);
        let source_version_binding = args.source_version.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_config_binding = args.vpc_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codebuild/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "artifacts".into(),
                    value: artifacts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "badgeEnabled".into(),
                    value: badge_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildBatchConfig".into(),
                    value: build_batch_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildTimeout".into(),
                    value: build_timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cache".into(),
                    value: cache_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "concurrentBuildLimit".into(),
                    value: concurrent_build_limit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionKey".into(),
                    value: encryption_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: environment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileSystemLocations".into(),
                    value: file_system_locations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logsConfig".into(),
                    value: logs_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectVisibility".into(),
                    value: project_visibility_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queuedTimeout".into(),
                    value: queued_timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceAccessRole".into(),
                    value: resource_access_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondaryArtifacts".into(),
                    value: secondary_artifacts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondarySourceVersions".into(),
                    value: secondary_source_versions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondarySources".into(),
                    value: secondary_sources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceRole".into(),
                    value: service_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceVersion".into(),
                    value: source_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfig".into(),
                    value: vpc_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectResult {
            arn: o.get_field("arn"),
            artifacts: o.get_field("artifacts"),
            badge_enabled: o.get_field("badgeEnabled"),
            badge_url: o.get_field("badgeUrl"),
            build_batch_config: o.get_field("buildBatchConfig"),
            build_timeout: o.get_field("buildTimeout"),
            cache: o.get_field("cache"),
            concurrent_build_limit: o.get_field("concurrentBuildLimit"),
            description: o.get_field("description"),
            encryption_key: o.get_field("encryptionKey"),
            environment: o.get_field("environment"),
            file_system_locations: o.get_field("fileSystemLocations"),
            logs_config: o.get_field("logsConfig"),
            name: o.get_field("name"),
            project_visibility: o.get_field("projectVisibility"),
            public_project_alias: o.get_field("publicProjectAlias"),
            queued_timeout: o.get_field("queuedTimeout"),
            resource_access_role: o.get_field("resourceAccessRole"),
            secondary_artifacts: o.get_field("secondaryArtifacts"),
            secondary_source_versions: o.get_field("secondarySourceVersions"),
            secondary_sources: o.get_field("secondarySources"),
            service_role: o.get_field("serviceRole"),
            source: o.get_field("source"),
            source_version: o.get_field("sourceVersion"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_config: o.get_field("vpcConfig"),
        }
    }
}
