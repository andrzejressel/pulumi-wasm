/// Provides an Amplify App resource, a fullstack serverless app hosted on the [AWS Amplify Console](https://docs.aws.amazon.com/amplify/latest/userguide/welcome.html).
///
/// > **Note:** When you create/update an Amplify App from the provider, you may end up with the error "BadRequestException: You should at least provide one valid token" because of authentication issues. See the section "Repository with Tokens" below.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:amplify:App
///     properties:
///       name: example
///       repository: https://github.com/example/app
///       buildSpec: |
///         version: 0.1
///         frontend:
///           phases:
///             preBuild:
///               commands:
///                 - yarn install
///             build:
///               commands:
///                 - yarn run build
///           artifacts:
///             baseDirectory: build
///             files:
///               - '**/*'
///           cache:
///             paths:
///               - node_modules/**/*
///       customRules:
///         - source: /<*>
///           status: '404'
///           target: /index.html
///       environmentVariables:
///         ENV: test
/// ```
///
/// ### Repository with Tokens
///
/// If you create a new Amplify App with the `repository` argument, you also need to set `oauth_token` or `access_token` for authentication. For GitHub, get a [personal access token](https://help.github.com/en/github/authenticating-to-github/creating-a-personal-access-token-for-the-command-line) and set `access_token` as follows:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app::create(
///         "example",
///         AppArgs::builder()
///             .access_token("...")
///             .name("example")
///             .repository("https://github.com/example/app")
///             .build_struct(),
///     );
/// }
/// ```
///
/// You can omit `access_token` if you import an existing Amplify App created by the Amplify Console (using OAuth for authentication).
///
/// ### Auto Branch Creation
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app::create(
///         "example",
///         AppArgs::builder()
///             .auto_branch_creation_config(
///                 AppAutoBranchCreationConfig::builder()
///                     .enableAutoBuild(true)
///                     .build_struct(),
///             )
///             .auto_branch_creation_patterns(vec!["*", "*/**",])
///             .enable_auto_branch_creation(true)
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic Authorization
///
/// ```yaml
/// resources:
///   example:
///     type: aws:amplify:App
///     properties:
///       name: example
///       enableBasicAuth: true
///       basicAuthCredentials:
///         fn::invoke:
///           function: std:base64encode
///           arguments:
///             input: username1:password1
///           return: result
/// ```
///
/// ### Rewrites and Redirects
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app::create(
///         "example",
///         AppArgs::builder()
///             .custom_rules(
///                 vec![
///                     AppCustomRule::builder().source("/api/<*>").status("200")
///                     .target("https://api.example.com/api/<*>").build_struct(),
///                     AppCustomRule::builder()
///                     .source("</^[^.]+$|\\.(?!(css|gif|ico|jpg|js|png|txt|svg|woff|ttf|map|json)$)([^.]+$)/>")
///                     .status("200").target("/index.html").build_struct(),
///                 ],
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Custom Image
///
/// ```yaml
/// resources:
///   example:
///     type: aws:amplify:App
///     properties:
///       name: example
///       environmentVariables:
///         _CUSTOM_IMAGE: node:16
/// ```
///
/// ### Custom Headers
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app::create(
///         "example",
///         AppArgs::builder()
///             .custom_headers(
///                 "customHeaders:\n  - pattern: '**'\n    headers:\n      - key: 'Strict-Transport-Security'\n        value: 'max-age=31536000; includeSubDomains'\n      - key: 'X-Frame-Options'\n        value: 'SAMEORIGIN'\n      - key: 'X-XSS-Protection'\n        value: '1; mode=block'\n      - key: 'X-Content-Type-Options'\n        value: 'nosniff'\n      - key: 'Content-Security-Policy'\n        value: \"default-src 'self'\"",
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amplify App using Amplify App ID (appId). For example:
///
/// ```sh
/// $ pulumi import aws:amplify/app:App example d2ypk4k47z8u6
/// ```
/// App ID can be obtained from App ARN (e.g., `arn:aws:amplify:us-east-1:12345678:apps/d2ypk4k47z8u6`).
///
pub mod app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppArgs {
        /// Personal access token for a third-party source control system for an Amplify app. This token must have write access to the relevant repo to create a webhook and a read-only deploy key for the Amplify project. The token is not stored, so after applying this attribute can be removed and the setup token deleted.
        #[builder(into, default)]
        pub access_token: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Automated branch creation configuration for an Amplify app. See `auto_branch_creation_config` Block for details.
        #[builder(into, default)]
        pub auto_branch_creation_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::amplify::AppAutoBranchCreationConfig>,
        >,
        /// Automated branch creation glob patterns for an Amplify app.
        #[builder(into, default)]
        pub auto_branch_creation_patterns: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Credentials for basic authorization for an Amplify app.
        #[builder(into, default)]
        pub basic_auth_credentials: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The [build specification](https://docs.aws.amazon.com/amplify/latest/userguide/build-settings.html) (build spec) for an Amplify app.
        #[builder(into, default)]
        pub build_spec: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Cache configuration for the Amplify app. See `cache_config` Block for details.
        #[builder(into, default)]
        pub cache_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::amplify::AppCacheConfig>,
        >,
        /// The [custom HTTP headers](https://docs.aws.amazon.com/amplify/latest/userguide/custom-headers.html) for an Amplify app.
        #[builder(into, default)]
        pub custom_headers: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Custom rewrite and redirect rules for an Amplify app. See `custom_rule` Block for details.
        #[builder(into, default)]
        pub custom_rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::amplify::AppCustomRule>>,
        >,
        /// Description for an Amplify app.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Enables automated branch creation for an Amplify app.
        #[builder(into, default)]
        pub enable_auto_branch_creation: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Enables basic authorization for an Amplify app. This will apply to all branches that are part of this app.
        #[builder(into, default)]
        pub enable_basic_auth: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Enables auto-building of branches for the Amplify App.
        #[builder(into, default)]
        pub enable_branch_auto_build: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Automatically disconnects a branch in the Amplify Console when you delete a branch from your Git repository.
        #[builder(into, default)]
        pub enable_branch_auto_deletion: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Environment variables map for an Amplify app.
        #[builder(into, default)]
        pub environment_variables: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// AWS Identity and Access Management (IAM) service role for an Amplify app.
        #[builder(into, default)]
        pub iam_service_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name for an Amplify app.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// OAuth token for a third-party source control system for an Amplify app. The OAuth token is used to create a webhook and a read-only deploy key. The OAuth token is not stored.
        #[builder(into, default)]
        pub oauth_token: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Platform or framework for an Amplify app. Valid values: `WEB`, `WEB_COMPUTE`. Default value: `WEB`.
        #[builder(into, default)]
        pub platform: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Repository for an Amplify app.
        #[builder(into, default)]
        pub repository: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppResult {
        /// Personal access token for a third-party source control system for an Amplify app. This token must have write access to the relevant repo to create a webhook and a read-only deploy key for the Amplify project. The token is not stored, so after applying this attribute can be removed and the setup token deleted.
        pub access_token: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the Amplify app.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Automated branch creation configuration for an Amplify app. See `auto_branch_creation_config` Block for details.
        pub auto_branch_creation_config: pulumi_wasm_rust::Output<
            super::super::types::amplify::AppAutoBranchCreationConfig,
        >,
        /// Automated branch creation glob patterns for an Amplify app.
        pub auto_branch_creation_patterns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Credentials for basic authorization for an Amplify app.
        pub basic_auth_credentials: pulumi_wasm_rust::Output<Option<String>>,
        /// The [build specification](https://docs.aws.amazon.com/amplify/latest/userguide/build-settings.html) (build spec) for an Amplify app.
        pub build_spec: pulumi_wasm_rust::Output<String>,
        /// Cache configuration for the Amplify app. See `cache_config` Block for details.
        pub cache_config: pulumi_wasm_rust::Output<
            super::super::types::amplify::AppCacheConfig,
        >,
        /// The [custom HTTP headers](https://docs.aws.amazon.com/amplify/latest/userguide/custom-headers.html) for an Amplify app.
        pub custom_headers: pulumi_wasm_rust::Output<String>,
        /// Custom rewrite and redirect rules for an Amplify app. See `custom_rule` Block for details.
        pub custom_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::amplify::AppCustomRule>>,
        >,
        /// Default domain for the Amplify app.
        pub default_domain: pulumi_wasm_rust::Output<String>,
        /// Description for an Amplify app.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables automated branch creation for an Amplify app.
        pub enable_auto_branch_creation: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables basic authorization for an Amplify app. This will apply to all branches that are part of this app.
        pub enable_basic_auth: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables auto-building of branches for the Amplify App.
        pub enable_branch_auto_build: pulumi_wasm_rust::Output<Option<bool>>,
        /// Automatically disconnects a branch in the Amplify Console when you delete a branch from your Git repository.
        pub enable_branch_auto_deletion: pulumi_wasm_rust::Output<Option<bool>>,
        /// Environment variables map for an Amplify app.
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// AWS Identity and Access Management (IAM) service role for an Amplify app.
        pub iam_service_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for an Amplify app.
        pub name: pulumi_wasm_rust::Output<String>,
        /// OAuth token for a third-party source control system for an Amplify app. The OAuth token is used to create a webhook and a read-only deploy key. The OAuth token is not stored.
        pub oauth_token: pulumi_wasm_rust::Output<Option<String>>,
        /// Platform or framework for an Amplify app. Valid values: `WEB`, `WEB_COMPUTE`. Default value: `WEB`.
        pub platform: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes the information about a production branch for an Amplify app. A `production_branch` block is documented below.
        pub production_branches: pulumi_wasm_rust::Output<
            Vec<super::super::types::amplify::AppProductionBranch>,
        >,
        /// Repository for an Amplify app.
        pub repository: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AppArgs,
    ) -> AppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_token_binding = args.access_token.get_output(context).get_inner();
        let auto_branch_creation_config_binding = args
            .auto_branch_creation_config
            .get_output(context)
            .get_inner();
        let auto_branch_creation_patterns_binding = args
            .auto_branch_creation_patterns
            .get_output(context)
            .get_inner();
        let basic_auth_credentials_binding = args
            .basic_auth_credentials
            .get_output(context)
            .get_inner();
        let build_spec_binding = args.build_spec.get_output(context).get_inner();
        let cache_config_binding = args.cache_config.get_output(context).get_inner();
        let custom_headers_binding = args.custom_headers.get_output(context).get_inner();
        let custom_rules_binding = args.custom_rules.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enable_auto_branch_creation_binding = args
            .enable_auto_branch_creation
            .get_output(context)
            .get_inner();
        let enable_basic_auth_binding = args
            .enable_basic_auth
            .get_output(context)
            .get_inner();
        let enable_branch_auto_build_binding = args
            .enable_branch_auto_build
            .get_output(context)
            .get_inner();
        let enable_branch_auto_deletion_binding = args
            .enable_branch_auto_deletion
            .get_output(context)
            .get_inner();
        let environment_variables_binding = args
            .environment_variables
            .get_output(context)
            .get_inner();
        let iam_service_role_arn_binding = args
            .iam_service_role_arn
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let oauth_token_binding = args.oauth_token.get_output(context).get_inner();
        let platform_binding = args.platform.get_output(context).get_inner();
        let repository_binding = args.repository.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amplify/app:App".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessToken".into(),
                    value: &access_token_binding,
                },
                register_interface::ObjectField {
                    name: "autoBranchCreationConfig".into(),
                    value: &auto_branch_creation_config_binding,
                },
                register_interface::ObjectField {
                    name: "autoBranchCreationPatterns".into(),
                    value: &auto_branch_creation_patterns_binding,
                },
                register_interface::ObjectField {
                    name: "basicAuthCredentials".into(),
                    value: &basic_auth_credentials_binding,
                },
                register_interface::ObjectField {
                    name: "buildSpec".into(),
                    value: &build_spec_binding,
                },
                register_interface::ObjectField {
                    name: "cacheConfig".into(),
                    value: &cache_config_binding,
                },
                register_interface::ObjectField {
                    name: "customHeaders".into(),
                    value: &custom_headers_binding,
                },
                register_interface::ObjectField {
                    name: "customRules".into(),
                    value: &custom_rules_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enableAutoBranchCreation".into(),
                    value: &enable_auto_branch_creation_binding,
                },
                register_interface::ObjectField {
                    name: "enableBasicAuth".into(),
                    value: &enable_basic_auth_binding,
                },
                register_interface::ObjectField {
                    name: "enableBranchAutoBuild".into(),
                    value: &enable_branch_auto_build_binding,
                },
                register_interface::ObjectField {
                    name: "enableBranchAutoDeletion".into(),
                    value: &enable_branch_auto_deletion_binding,
                },
                register_interface::ObjectField {
                    name: "environmentVariables".into(),
                    value: &environment_variables_binding,
                },
                register_interface::ObjectField {
                    name: "iamServiceRoleArn".into(),
                    value: &iam_service_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "oauthToken".into(),
                    value: &oauth_token_binding,
                },
                register_interface::ObjectField {
                    name: "platform".into(),
                    value: &platform_binding,
                },
                register_interface::ObjectField {
                    name: "repository".into(),
                    value: &repository_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessToken".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoBranchCreationConfig".into(),
                },
                register_interface::ResultField {
                    name: "autoBranchCreationPatterns".into(),
                },
                register_interface::ResultField {
                    name: "basicAuthCredentials".into(),
                },
                register_interface::ResultField {
                    name: "buildSpec".into(),
                },
                register_interface::ResultField {
                    name: "cacheConfig".into(),
                },
                register_interface::ResultField {
                    name: "customHeaders".into(),
                },
                register_interface::ResultField {
                    name: "customRules".into(),
                },
                register_interface::ResultField {
                    name: "defaultDomain".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enableAutoBranchCreation".into(),
                },
                register_interface::ResultField {
                    name: "enableBasicAuth".into(),
                },
                register_interface::ResultField {
                    name: "enableBranchAutoBuild".into(),
                },
                register_interface::ResultField {
                    name: "enableBranchAutoDeletion".into(),
                },
                register_interface::ResultField {
                    name: "environmentVariables".into(),
                },
                register_interface::ResultField {
                    name: "iamServiceRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "oauthToken".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "productionBranches".into(),
                },
                register_interface::ResultField {
                    name: "repository".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppResult {
            access_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessToken").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_branch_creation_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoBranchCreationConfig").unwrap(),
            ),
            auto_branch_creation_patterns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoBranchCreationPatterns").unwrap(),
            ),
            basic_auth_credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("basicAuthCredentials").unwrap(),
            ),
            build_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buildSpec").unwrap(),
            ),
            cache_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheConfig").unwrap(),
            ),
            custom_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customHeaders").unwrap(),
            ),
            custom_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customRules").unwrap(),
            ),
            default_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultDomain").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enable_auto_branch_creation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableAutoBranchCreation").unwrap(),
            ),
            enable_basic_auth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableBasicAuth").unwrap(),
            ),
            enable_branch_auto_build: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableBranchAutoBuild").unwrap(),
            ),
            enable_branch_auto_deletion: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableBranchAutoDeletion").unwrap(),
            ),
            environment_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentVariables").unwrap(),
            ),
            iam_service_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamServiceRoleArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            oauth_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oauthToken").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            production_branches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productionBranches").unwrap(),
            ),
            repository: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repository").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
