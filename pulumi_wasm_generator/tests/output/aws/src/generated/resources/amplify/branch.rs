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
///           Function: std:base64encode
///           Arguments:
///             input: username:password
///           Return: result
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
///       Function: aws:iam:getPolicyDocument
///       Arguments:
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
pub mod branch {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BranchArgs {
        /// Unique ID for an Amplify app.
        #[builder(into)]
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// ARN for a backend environment that is part of an Amplify app.
        #[builder(into, default)]
        pub backend_environment_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Basic authorization credentials for the branch.
        #[builder(into, default)]
        pub basic_auth_credentials: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for the branch.
        #[builder(into)]
        pub branch_name: pulumi_wasm_rust::Output<String>,
        /// Description for the branch.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Display name for a branch. This is used as the default domain prefix.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables auto building for the branch.
        #[builder(into, default)]
        pub enable_auto_build: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables basic authorization for the branch.
        #[builder(into, default)]
        pub enable_basic_auth: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables notifications for the branch.
        #[builder(into, default)]
        pub enable_notification: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables performance mode for the branch.
        #[builder(into, default)]
        pub enable_performance_mode: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables pull request previews for this branch.
        #[builder(into, default)]
        pub enable_pull_request_preview: pulumi_wasm_rust::Output<Option<bool>>,
        /// Environment variables for the branch.
        #[builder(into, default)]
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Framework for the branch.
        #[builder(into, default)]
        pub framework: pulumi_wasm_rust::Output<Option<String>>,
        /// Amplify environment name for the pull request.
        #[builder(into, default)]
        pub pull_request_environment_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes the current stage for the branch. Valid values: `PRODUCTION`, `BETA`, `DEVELOPMENT`, `EXPERIMENTAL`, `PULL_REQUEST`.
        #[builder(into, default)]
        pub stage: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Content Time To Live (TTL) for the website in seconds.
        #[builder(into, default)]
        pub ttl: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BranchResult {
        /// Unique ID for an Amplify app.
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// ARN for the branch.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A list of custom resources that are linked to this branch.
        pub associated_resources: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN for a backend environment that is part of an Amplify app.
        pub backend_environment_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Basic authorization credentials for the branch.
        pub basic_auth_credentials: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for the branch.
        pub branch_name: pulumi_wasm_rust::Output<String>,
        /// Custom domains for the branch.
        pub custom_domains: pulumi_wasm_rust::Output<Vec<String>>,
        /// Description for the branch.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Destination branch if the branch is a pull request branch.
        pub destination_branch: pulumi_wasm_rust::Output<String>,
        /// Display name for a branch. This is used as the default domain prefix.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Enables auto building for the branch.
        pub enable_auto_build: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables basic authorization for the branch.
        pub enable_basic_auth: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables notifications for the branch.
        pub enable_notification: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables performance mode for the branch.
        pub enable_performance_mode: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables pull request previews for this branch.
        pub enable_pull_request_preview: pulumi_wasm_rust::Output<Option<bool>>,
        /// Environment variables for the branch.
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Framework for the branch.
        pub framework: pulumi_wasm_rust::Output<Option<String>>,
        /// Amplify environment name for the pull request.
        pub pull_request_environment_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Source branch if the branch is a pull request branch.
        pub source_branch: pulumi_wasm_rust::Output<String>,
        /// Describes the current stage for the branch. Valid values: `PRODUCTION`, `BETA`, `DEVELOPMENT`, `EXPERIMENTAL`, `PULL_REQUEST`.
        pub stage: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Content Time To Live (TTL) for the website in seconds.
        pub ttl: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BranchArgs) -> BranchResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_inner();
        let backend_environment_arn_binding = args.backend_environment_arn.get_inner();
        let basic_auth_credentials_binding = args.basic_auth_credentials.get_inner();
        let branch_name_binding = args.branch_name.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let enable_auto_build_binding = args.enable_auto_build.get_inner();
        let enable_basic_auth_binding = args.enable_basic_auth.get_inner();
        let enable_notification_binding = args.enable_notification.get_inner();
        let enable_performance_mode_binding = args.enable_performance_mode.get_inner();
        let enable_pull_request_preview_binding = args
            .enable_pull_request_preview
            .get_inner();
        let environment_variables_binding = args.environment_variables.get_inner();
        let framework_binding = args.framework.get_inner();
        let pull_request_environment_name_binding = args
            .pull_request_environment_name
            .get_inner();
        let stage_binding = args.stage.get_inner();
        let tags_binding = args.tags.get_inner();
        let ttl_binding = args.ttl.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amplify/branch:Branch".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "backendEnvironmentArn".into(),
                    value: &backend_environment_arn_binding,
                },
                register_interface::ObjectField {
                    name: "basicAuthCredentials".into(),
                    value: &basic_auth_credentials_binding,
                },
                register_interface::ObjectField {
                    name: "branchName".into(),
                    value: &branch_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enableAutoBuild".into(),
                    value: &enable_auto_build_binding,
                },
                register_interface::ObjectField {
                    name: "enableBasicAuth".into(),
                    value: &enable_basic_auth_binding,
                },
                register_interface::ObjectField {
                    name: "enableNotification".into(),
                    value: &enable_notification_binding,
                },
                register_interface::ObjectField {
                    name: "enablePerformanceMode".into(),
                    value: &enable_performance_mode_binding,
                },
                register_interface::ObjectField {
                    name: "enablePullRequestPreview".into(),
                    value: &enable_pull_request_preview_binding,
                },
                register_interface::ObjectField {
                    name: "environmentVariables".into(),
                    value: &environment_variables_binding,
                },
                register_interface::ObjectField {
                    name: "framework".into(),
                    value: &framework_binding,
                },
                register_interface::ObjectField {
                    name: "pullRequestEnvironmentName".into(),
                    value: &pull_request_environment_name_binding,
                },
                register_interface::ObjectField {
                    name: "stage".into(),
                    value: &stage_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associatedResources".into(),
                },
                register_interface::ResultField {
                    name: "backendEnvironmentArn".into(),
                },
                register_interface::ResultField {
                    name: "basicAuthCredentials".into(),
                },
                register_interface::ResultField {
                    name: "branchName".into(),
                },
                register_interface::ResultField {
                    name: "customDomains".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destinationBranch".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "enableAutoBuild".into(),
                },
                register_interface::ResultField {
                    name: "enableBasicAuth".into(),
                },
                register_interface::ResultField {
                    name: "enableNotification".into(),
                },
                register_interface::ResultField {
                    name: "enablePerformanceMode".into(),
                },
                register_interface::ResultField {
                    name: "enablePullRequestPreview".into(),
                },
                register_interface::ResultField {
                    name: "environmentVariables".into(),
                },
                register_interface::ResultField {
                    name: "framework".into(),
                },
                register_interface::ResultField {
                    name: "pullRequestEnvironmentName".into(),
                },
                register_interface::ResultField {
                    name: "sourceBranch".into(),
                },
                register_interface::ResultField {
                    name: "stage".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BranchResult {
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            associated_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associatedResources").unwrap(),
            ),
            backend_environment_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendEnvironmentArn").unwrap(),
            ),
            basic_auth_credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("basicAuthCredentials").unwrap(),
            ),
            branch_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("branchName").unwrap(),
            ),
            custom_domains: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomains").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destination_branch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationBranch").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            enable_auto_build: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableAutoBuild").unwrap(),
            ),
            enable_basic_auth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableBasicAuth").unwrap(),
            ),
            enable_notification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableNotification").unwrap(),
            ),
            enable_performance_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enablePerformanceMode").unwrap(),
            ),
            enable_pull_request_preview: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enablePullRequestPreview").unwrap(),
            ),
            environment_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentVariables").unwrap(),
            ),
            framework: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("framework").unwrap(),
            ),
            pull_request_environment_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pullRequestEnvironmentName").unwrap(),
            ),
            source_branch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceBranch").unwrap(),
            ),
            stage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stage").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ttl").unwrap()),
        }
    }
}