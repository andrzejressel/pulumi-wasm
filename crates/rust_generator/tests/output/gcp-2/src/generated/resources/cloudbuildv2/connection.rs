/// A connection to a SCM like GitHub, GitHub Enterprise, Bitbucket Data Center/Cloud or GitLab.
///
///
/// To get more information about Connection, see:
///
/// * [API documentation](https://cloud.google.com/build/docs/api/reference/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/build/docs)
///
/// ## Example Usage
///
/// ### Cloudbuildv2 Connection
///
///
/// ```yaml
/// resources:
///   my-connection:
///     type: gcp:cloudbuildv2:Connection
///     properties:
///       location: us-central1
///       name: tf-test-connection
///       githubConfig:
///         appInstallationId: 0
///         authorizerCredential:
///           oauthTokenSecretVersion: projects/gcb-terraform-creds/secrets/github-pat/versions/1
/// ```
/// ### Cloudbuildv2 Connection Ghe
///
///
/// ```yaml
/// resources:
///   private-key-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: ghe-pk-secret
///       replication:
///         auto: {}
///   private-key-secret-version:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["private-key-secret"].id}
///       secretData:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: private-key.pem
///           return: result
///   webhook-secret-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: github-token-secret
///       replication:
///         auto: {}
///   webhook-secret-secret-version:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["webhook-secret-secret"].id}
///       secretData: <webhook-secret-data>
///   policy-pk:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       secretId: ${["private-key-secret"].secretId}
///       policyData: ${["p4sa-secretAccessor"].policyData}
///   policy-whs:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       secretId: ${["webhook-secret-secret"].secretId}
///       policyData: ${["p4sa-secretAccessor"].policyData}
///   my-connection:
///     type: gcp:cloudbuildv2:Connection
///     properties:
///       location: us-central1
///       name: my-terraform-ghe-connection
///       githubEnterpriseConfig:
///         hostUri: https://ghe.com
///         privateKeySecretVersion: ${["private-key-secret-version"].id}
///         webhookSecretSecretVersion: ${["webhook-secret-secret-version"].id}
///         appId: 200
///         appSlug: gcb-app
///         appInstallationId: 300
///     options:
///       dependsOn:
///         - ${["policy-pk"]}
///         - ${["policy-whs"]}
/// variables:
///   p4sa-secretAccessor:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - serviceAccount:service-123456789@gcp-sa-cloudbuild.iam.gserviceaccount.com
/// ```
/// ### Cloudbuildv2 Connection Github
///
///
/// ```yaml
/// resources:
///   github-token-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: github-token-secret
///       replication:
///         auto: {}
///   github-token-secret-version:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["github-token-secret"].id}
///       secretData:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: my-github-token.txt
///           return: result
///   policy:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       secretId: ${["github-token-secret"].secretId}
///       policyData: ${["p4sa-secretAccessor"].policyData}
///   my-connection:
///     type: gcp:cloudbuildv2:Connection
///     properties:
///       location: us-central1
///       name: my-connection
///       githubConfig:
///         appInstallationId: 123123
///         authorizerCredential:
///           oauthTokenSecretVersion: ${["github-token-secret-version"].id}
/// variables:
///   p4sa-secretAccessor:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - serviceAccount:service-123456789@gcp-sa-cloudbuild.iam.gserviceaccount.com
/// ```
///
/// ## Import
///
/// Connection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/connections/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Connection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/connection:Connection default projects/{{project}}/locations/{{location}}/connections/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/connection:Connection default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/connection:Connection default {{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/connection:Connection default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// Allows clients to store small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration for connections to Bitbucket Cloud.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bitbucket_cloud_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuildv2::ConnectionBitbucketCloudConfig>,
        >,
        /// Configuration for connections to Bitbucket Data Center.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bitbucket_data_center_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cloudbuildv2::ConnectionBitbucketDataCenterConfig,
            >,
        >,
        /// If disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration for connections to github.com.
        /// Structure is documented below.
        #[builder(into, default)]
        pub github_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuildv2::ConnectionGithubConfig>,
        >,
        /// Configuration for connections to an instance of GitHub Enterprise.
        /// Structure is documented below.
        #[builder(into, default)]
        pub github_enterprise_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuildv2::ConnectionGithubEnterpriseConfig>,
        >,
        /// Configuration for connections to gitlab.com or an instance of GitLab Enterprise.
        /// Structure is documented below.
        #[builder(into, default)]
        pub gitlab_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuildv2::ConnectionGitlabConfig>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Immutable. The resource name of the connection.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// Allows clients to store small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration for connections to Bitbucket Cloud.
        /// Structure is documented below.
        pub bitbucket_cloud_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuildv2::ConnectionBitbucketCloudConfig>,
        >,
        /// Configuration for connections to Bitbucket Data Center.
        /// Structure is documented below.
        pub bitbucket_data_center_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cloudbuildv2::ConnectionBitbucketDataCenterConfig,
            >,
        >,
        /// Output only. Server assigned timestamp for when the connection was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// If disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Configuration for connections to github.com.
        /// Structure is documented below.
        pub github_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuildv2::ConnectionGithubConfig>,
        >,
        /// Configuration for connections to an instance of GitHub Enterprise.
        /// Structure is documented below.
        pub github_enterprise_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuildv2::ConnectionGithubEnterpriseConfig>,
        >,
        /// Configuration for connections to gitlab.com or an instance of GitLab Enterprise.
        /// Structure is documented below.
        pub gitlab_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuildv2::ConnectionGitlabConfig>,
        >,
        /// Output only. Installation state of the Connection.
        /// Structure is documented below.
        pub installation_states: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudbuildv2::ConnectionInstallationState>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The resource name of the connection.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. Set to true when the connection is being set up or updated in the background.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Output only. Server assigned timestamp for when the connection was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ConnectionArgs,
    ) -> ConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let bitbucket_cloud_config_binding = args
            .bitbucket_cloud_config
            .get_output(context)
            .get_inner();
        let bitbucket_data_center_config_binding = args
            .bitbucket_data_center_config
            .get_output(context)
            .get_inner();
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let github_config_binding = args.github_config.get_output(context).get_inner();
        let github_enterprise_config_binding = args
            .github_enterprise_config
            .get_output(context)
            .get_inner();
        let gitlab_config_binding = args.gitlab_config.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudbuildv2/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "bitbucketCloudConfig".into(),
                    value: &bitbucket_cloud_config_binding,
                },
                register_interface::ObjectField {
                    name: "bitbucketDataCenterConfig".into(),
                    value: &bitbucket_data_center_config_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "githubConfig".into(),
                    value: &github_config_binding,
                },
                register_interface::ObjectField {
                    name: "githubEnterpriseConfig".into(),
                    value: &github_enterprise_config_binding,
                },
                register_interface::ObjectField {
                    name: "gitlabConfig".into(),
                    value: &gitlab_config_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConnectionResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            bitbucket_cloud_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bitbucketCloudConfig"),
            ),
            bitbucket_data_center_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bitbucketDataCenterConfig"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            github_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("githubConfig"),
            ),
            github_enterprise_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("githubEnterpriseConfig"),
            ),
            gitlab_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gitlabConfig"),
            ),
            installation_states: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("installationStates"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            reconciling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
