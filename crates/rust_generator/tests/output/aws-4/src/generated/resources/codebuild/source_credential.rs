/// Provides a CodeBuild Source Credentials Resource.
///
/// > **NOTE:** [Codebuild only allows a single credential per given server type in a given region](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_codebuild.GitHubSourceCredentials.html). Therefore, when you define `aws.codebuild.SourceCredential`, `aws.codebuild.Project` resource defined in the same module will use it.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = source_credential::create(
///         "example",
///         SourceCredentialArgs::builder()
///             .auth_type("PERSONAL_ACCESS_TOKEN")
///             .server_type("GITHUB")
///             .token("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Bitbucket Server Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = source_credential::create(
///         "example",
///         SourceCredentialArgs::builder()
///             .auth_type("BASIC_AUTH")
///             .server_type("BITBUCKET")
///             .token("example")
///             .user_name("test-user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeBuild Source Credential using the CodeBuild Source Credential arn. For example:
///
/// ```sh
/// $ pulumi import aws:codebuild/sourceCredential:SourceCredential example arn:aws:codebuild:us-west-2:123456789:token:github
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod source_credential {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceCredentialArgs {
        /// The type of authentication used to connect to a GitHub, GitHub Enterprise, or Bitbucket repository. An OAUTH connection is not supported by the API.
        #[builder(into)]
        pub auth_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The source provider used for this project.
        #[builder(into)]
        pub server_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// For `GitHub` or `GitHub Enterprise`, this is the personal access token. For `Bitbucket`, this is the app password.
        #[builder(into)]
        pub token: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Bitbucket username when the authType is `BASIC_AUTH`. This parameter is not valid for other types of source providers or connections.
        #[builder(into, default)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SourceCredentialResult {
        /// The ARN of Source Credential.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The type of authentication used to connect to a GitHub, GitHub Enterprise, or Bitbucket repository. An OAUTH connection is not supported by the API.
        pub auth_type: pulumi_gestalt_rust::Output<String>,
        /// The source provider used for this project.
        pub server_type: pulumi_gestalt_rust::Output<String>,
        /// For `GitHub` or `GitHub Enterprise`, this is the personal access token. For `Bitbucket`, this is the app password.
        pub token: pulumi_gestalt_rust::Output<String>,
        /// The Bitbucket username when the authType is `BASIC_AUTH`. This parameter is not valid for other types of source providers or connections.
        pub user_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SourceCredentialArgs,
    ) -> SourceCredentialResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auth_type_binding = args.auth_type.get_output(context);
        let server_type_binding = args.server_type.get_output(context);
        let token_binding = args.token.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codebuild/sourceCredential:SourceCredential".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authType".into(),
                    value: auth_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverType".into(),
                    value: server_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "token".into(),
                    value: token_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: user_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SourceCredentialResult {
            arn: o.get_field("arn"),
            auth_type: o.get_field("authType"),
            server_type: o.get_field("serverType"),
            token: o.get_field("token"),
            user_name: o.get_field("userName"),
        }
    }
}
