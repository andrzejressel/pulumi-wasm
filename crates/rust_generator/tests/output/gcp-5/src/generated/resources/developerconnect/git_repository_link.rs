/// ## Example Usage
///
/// ### Developer Connect Git Repository Link Github Doc
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
///     type: gcp:developerconnect:Connection
///     properties:
///       location: us-central1
///       connectionId: my-connection
///       githubConfig:
///         githubApp: DEVELOPER_CONNECT
///         appInstallationId: 123123
///         authorizerCredential:
///           oauthTokenSecretVersion: ${["github-token-secret-version"].id}
///   my-repository:
///     type: gcp:developerconnect:GitRepositoryLink
///     properties:
///       location: us-central1
///       gitRepositoryLinkId: my-repo
///       parentConnection: ${["my-connection"].connectionId}
///       remoteUri: https://github.com/myuser/myrepo.git
/// variables:
///   p4sa-secretAccessor:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - serviceAccount:service-123456789@gcp-sa-devconnect.iam.gserviceaccount.com
/// ```
///
/// ## Import
///
/// GitRepositoryLink can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/connections/{{parent_connection}}/gitRepositoryLinks/{{git_repository_link_id}}`
///
/// * `{{project}}/{{location}}/{{parent_connection}}/{{git_repository_link_id}}`
///
/// * `{{location}}/{{parent_connection}}/{{git_repository_link_id}}`
///
/// When using the `pulumi import` command, GitRepositoryLink can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:developerconnect/gitRepositoryLink:GitRepositoryLink default projects/{{project}}/locations/{{location}}/connections/{{parent_connection}}/gitRepositoryLinks/{{git_repository_link_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:developerconnect/gitRepositoryLink:GitRepositoryLink default {{project}}/{{location}}/{{parent_connection}}/{{git_repository_link_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:developerconnect/gitRepositoryLink:GitRepositoryLink default {{location}}/{{parent_connection}}/{{git_repository_link_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod git_repository_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GitRepositoryLinkArgs {
        /// Optional. Allows clients to store small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Required. Git Clone URI.
        #[builder(into)]
        pub clone_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. This checksum is computed by the server based on the value of other
        /// fields, and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. The ID to use for the repository, which will become the final component of
        /// the repository's resource name. This ID should be unique in the connection.
        /// Allows alphanumeric characters and any of -._~%!$&'()*+,;=@.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub git_repository_link_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `developerconnect.googleapis.com/GitRepositoryLink`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `developerconnect.googleapis.com/GitRepositoryLink`.
        #[builder(into)]
        pub parent_connection: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GitRepositoryLinkResult {
        /// Optional. Allows clients to store small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Required. Git Clone URI.
        pub clone_uri: pulumi_gestalt_rust::Output<String>,
        /// Output only. [Output only] Create timestamp
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. [Output only] Delete timestamp
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. This checksum is computed by the server based on the value of other
        /// fields, and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<Option<String>>,
        /// Required. The ID to use for the repository, which will become the final component of
        /// the repository's resource name. This ID should be unique in the connection.
        /// Allows alphanumeric characters and any of -._~%!$&'()*+,;=@.
        ///
        ///
        /// - - -
        pub git_repository_link_id: pulumi_gestalt_rust::Output<String>,
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `developerconnect.googleapis.com/GitRepositoryLink`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. Resource name of the repository, in the format
        /// `projects/*/locations/*/connections/*/gitRepositoryLinks/*`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `developerconnect.googleapis.com/GitRepositoryLink`.
        pub parent_connection: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Set to true when the connection is being set up or updated in the
        /// background.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Output only. A system-assigned unique identifier for a the GitRepositoryLink.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. [Output only] Update timestamp
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GitRepositoryLinkArgs,
    ) -> GitRepositoryLinkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let clone_uri_binding = args.clone_uri.get_output(context);
        let etag_binding = args.etag.get_output(context);
        let git_repository_link_id_binding = args
            .git_repository_link_id
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let parent_connection_binding = args.parent_connection.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:developerconnect/gitRepositoryLink:GitRepositoryLink".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloneUri".into(),
                    value: clone_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "etag".into(),
                    value: etag_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gitRepositoryLinkId".into(),
                    value: git_repository_link_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentConnection".into(),
                    value: parent_connection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GitRepositoryLinkResult {
            annotations: o.get_field("annotations"),
            clone_uri: o.get_field("cloneUri"),
            create_time: o.get_field("createTime"),
            delete_time: o.get_field("deleteTime"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            git_repository_link_id: o.get_field("gitRepositoryLinkId"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent_connection: o.get_field("parentConnection"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
