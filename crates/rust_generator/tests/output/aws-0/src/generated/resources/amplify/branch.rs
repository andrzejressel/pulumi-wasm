/// Provides an Amplify Branch resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:amplify:App
///     properties:
///       name: app
///   master:
///     type: aws:amplify:Branch
///     properties:
///       appId: ${example.id}
///       branchName: master
///       framework: React
///       stage: PRODUCTION
///       environmentVariables:
///         REACT_APP_API_SERVER: https://api.example.com
/// ```
///
/// ### Basic Authentication
///
/// ```yaml
/// resources:
///   example:
///     type: aws:amplify:App
///     properties:
///       name: app
///   master:
///     type: aws:amplify:Branch
///     properties:
///       appId: ${example.id}
///       branchName: master
///       enableBasicAuth: true
///       basicAuthCredentials:
///         fn::invoke:
///           function: std:base64encode
///           arguments:
///             input: username:password
///           return: result
/// ```
///
/// ### Notifications
///
/// Amplify Console uses EventBridge (formerly known as CloudWatch Events) and SNS for email notifications.  To implement the same functionality, you need to set `enable_notification` in a `aws.amplify.Branch` resource, as well as creating an EventBridge Rule, an SNS topic, and SNS subscriptions.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:amplify:App
///     properties:
///       name: app
///   master:
///     type: aws:amplify:Branch
///     properties:
///       appId: ${example.id}
///       branchName: master
///       enableNotification: true
///   # EventBridge Rule for Amplify notifications
///   amplifyAppMasterEventRule:
///     type: aws:cloudwatch:EventRule
///     name: amplify_app_master
///     properties:
///       name: amplify-${app.id}-${master.branchName}-branch-notification
///       description: 'AWS Amplify build notifications for :  App: ${app.id} Branch: ${master.branchName}'
///       eventPattern:
///         fn::toJSON:
///           detail:
///             appId:
///               - ${example.id}
///             branchName:
///               - ${master.branchName}
///             jobStatus:
///               - SUCCEED
///               - FAILED
///               - STARTED
///           detail-type:
///             - Amplify Deployment Status Change
///           source:
///             - aws.amplify
///   amplifyAppMasterEventTarget:
///     type: aws:cloudwatch:EventTarget
///     name: amplify_app_master
///     properties:
///       rule: ${amplifyAppMasterEventRule.name}
///       targetId: ${master.branchName}
///       arn: ${amplifyAppMasterTopic.arn}
///       inputTransformer:
///         inputPaths:
///           jobId: $.detail.jobId
///           appId: $.detail.appId
///           region: $.region
///           branch: $.detail.branchName
///           status: $.detail.jobStatus
///         inputTemplate: '"Build notification from the AWS Amplify Console for app: https://<branch>.<appId>.amplifyapp.com/. Your build status is <status>. Go to https://console.aws.amazon.com/amplify/home?region=<region>#<appId>/<branch>/<jobId> to view details on your build. "'
///   # SNS Topic for Amplify notifications
///   amplifyAppMasterTopic:
///     type: aws:sns:Topic
///     name: amplify_app_master
///     properties:
///       name: amplify-${app.id}_${master.branchName}
///   amplifyAppMasterTopicPolicy:
///     type: aws:sns:TopicPolicy
///     name: amplify_app_master
///     properties:
///       arn: ${amplifyAppMasterTopic.arn}
///       policy: ${amplifyAppMaster.json}
///   this:
///     type: aws:sns:TopicSubscription
///     properties:
///       topic: ${amplifyAppMasterTopic.arn}
///       protocol: email
///       endpoint: user@acme.com
/// variables:
///   amplifyAppMaster:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: Allow_Publish_Events ${master.arn}
///             effect: Allow
///             actions:
///               - SNS:Publish
///             principals:
///               - type: Service
///                 identifiers:
///                   - events.amazonaws.com
///             resources:
///               - ${amplifyAppMasterTopic.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amplify branch using `app_id` and `branch_name`. For example:
///
/// ```sh
/// $ pulumi import aws:amplify/branch:Branch master d2ypk4k47z8u6/master
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod branch {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BranchArgs {
        /// Unique ID for an Amplify app.
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN for a backend environment that is part of an Amplify app.
        #[builder(into, default)]
        pub backend_environment_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Basic authorization credentials for the branch.
        #[builder(into, default)]
        pub basic_auth_credentials: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name for the branch.
        #[builder(into)]
        pub branch_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description for the branch.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Display name for a branch. This is used as the default domain prefix.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables auto building for the branch.
        #[builder(into, default)]
        pub enable_auto_build: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enables basic authorization for the branch.
        #[builder(into, default)]
        pub enable_basic_auth: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enables notifications for the branch.
        #[builder(into, default)]
        pub enable_notification: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enables performance mode for the branch.
        #[builder(into, default)]
        pub enable_performance_mode: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enables pull request previews for this branch.
        #[builder(into, default)]
        pub enable_pull_request_preview: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Environment variables for the branch.
        #[builder(into, default)]
        pub environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Framework for the branch.
        #[builder(into, default)]
        pub framework: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amplify environment name for the pull request.
        #[builder(into, default)]
        pub pull_request_environment_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Describes the current stage for the branch. Valid values: `PRODUCTION`, `BETA`, `DEVELOPMENT`, `EXPERIMENTAL`, `PULL_REQUEST`.
        #[builder(into, default)]
        pub stage: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Content Time To Live (TTL) for the website in seconds.
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BranchResult {
        /// Unique ID for an Amplify app.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the branch.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A list of custom resources that are linked to this branch.
        pub associated_resources: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ARN for a backend environment that is part of an Amplify app.
        pub backend_environment_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Basic authorization credentials for the branch.
        pub basic_auth_credentials: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name for the branch.
        pub branch_name: pulumi_gestalt_rust::Output<String>,
        /// Custom domains for the branch.
        pub custom_domains: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Description for the branch.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Destination branch if the branch is a pull request branch.
        pub destination_branch: pulumi_gestalt_rust::Output<String>,
        /// Display name for a branch. This is used as the default domain prefix.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Enables auto building for the branch.
        pub enable_auto_build: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Enables basic authorization for the branch.
        pub enable_basic_auth: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Enables notifications for the branch.
        pub enable_notification: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Enables performance mode for the branch.
        pub enable_performance_mode: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Enables pull request previews for this branch.
        pub enable_pull_request_preview: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Environment variables for the branch.
        pub environment_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Framework for the branch.
        pub framework: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amplify environment name for the pull request.
        pub pull_request_environment_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Source branch if the branch is a pull request branch.
        pub source_branch: pulumi_gestalt_rust::Output<String>,
        /// Describes the current stage for the branch. Valid values: `PRODUCTION`, `BETA`, `DEVELOPMENT`, `EXPERIMENTAL`, `PULL_REQUEST`.
        pub stage: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Content Time To Live (TTL) for the website in seconds.
        pub ttl: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BranchArgs,
    ) -> BranchResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_id_binding = args.app_id.get_output(context);
        let backend_environment_arn_binding = args
            .backend_environment_arn
            .get_output(context);
        let basic_auth_credentials_binding = args
            .basic_auth_credentials
            .get_output(context);
        let branch_name_binding = args.branch_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enable_auto_build_binding = args.enable_auto_build.get_output(context);
        let enable_basic_auth_binding = args.enable_basic_auth.get_output(context);
        let enable_notification_binding = args.enable_notification.get_output(context);
        let enable_performance_mode_binding = args
            .enable_performance_mode
            .get_output(context);
        let enable_pull_request_preview_binding = args
            .enable_pull_request_preview
            .get_output(context);
        let environment_variables_binding = args
            .environment_variables
            .get_output(context);
        let framework_binding = args.framework.get_output(context);
        let pull_request_environment_name_binding = args
            .pull_request_environment_name
            .get_output(context);
        let stage_binding = args.stage.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:amplify/branch:Branch".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appId".into(),
                    value: app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendEnvironmentArn".into(),
                    value: backend_environment_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "basicAuthCredentials".into(),
                    value: basic_auth_credentials_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "branchName".into(),
                    value: branch_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableAutoBuild".into(),
                    value: enable_auto_build_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableBasicAuth".into(),
                    value: enable_basic_auth_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableNotification".into(),
                    value: enable_notification_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enablePerformanceMode".into(),
                    value: enable_performance_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enablePullRequestPreview".into(),
                    value: enable_pull_request_preview_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentVariables".into(),
                    value: environment_variables_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "framework".into(),
                    value: framework_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pullRequestEnvironmentName".into(),
                    value: pull_request_environment_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stage".into(),
                    value: stage_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: ttl_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BranchResult {
            app_id: o.get_field("appId"),
            arn: o.get_field("arn"),
            associated_resources: o.get_field("associatedResources"),
            backend_environment_arn: o.get_field("backendEnvironmentArn"),
            basic_auth_credentials: o.get_field("basicAuthCredentials"),
            branch_name: o.get_field("branchName"),
            custom_domains: o.get_field("customDomains"),
            description: o.get_field("description"),
            destination_branch: o.get_field("destinationBranch"),
            display_name: o.get_field("displayName"),
            enable_auto_build: o.get_field("enableAutoBuild"),
            enable_basic_auth: o.get_field("enableBasicAuth"),
            enable_notification: o.get_field("enableNotification"),
            enable_performance_mode: o.get_field("enablePerformanceMode"),
            enable_pull_request_preview: o.get_field("enablePullRequestPreview"),
            environment_variables: o.get_field("environmentVariables"),
            framework: o.get_field("framework"),
            pull_request_environment_name: o.get_field("pullRequestEnvironmentName"),
            source_branch: o.get_field("sourceBranch"),
            stage: o.get_field("stage"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            ttl: o.get_field("ttl"),
        }
    }
}
