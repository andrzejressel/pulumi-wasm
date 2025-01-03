/// A repository associated to a parent connection.
///
///
/// To get more information about Repository, see:
///
/// * [API documentation](https://cloud.google.com/build/docs/api/reference/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/build/docs)
///
/// ## Example Usage
///
/// ### Cloudbuildv2 Repository Ghe Doc
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
///   my-repository:
///     type: gcp:cloudbuildv2:Repository
///     properties:
///       name: my-terraform-ghe-repo
///       location: us-central1
///       parentConnection: ${["my-connection"].name}
///       remoteUri: https://ghe.com/hashicorp/terraform-provider-google.git
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
/// ### Cloudbuildv2 Repository Github Doc
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
///   my-repository:
///     type: gcp:cloudbuildv2:Repository
///     properties:
///       location: us-central1
///       name: my-repo
///       parentConnection: ${["my-connection"].name}
///       remoteUri: https://github.com/myuser/myrepo.git
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
/// Repository can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/connections/{{parent_connection}}/repositories/{{name}}`
///
/// * `{{project}}/{{location}}/{{parent_connection}}/{{name}}`
///
/// * `{{location}}/{{parent_connection}}/{{name}}`
///
/// When using the `pulumi import` command, Repository can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/repository:Repository default projects/{{project}}/locations/{{location}}/connections/{{parent_connection}}/repositories/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/repository:Repository default {{project}}/{{location}}/{{parent_connection}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/repository:Repository default {{location}}/{{parent_connection}}/{{name}}
/// ```
///
pub mod repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Allows clients to store small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the repository.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The connection for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent_connection: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Git Clone HTTPS URI.
        #[builder(into)]
        pub remote_uri: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Allows clients to store small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Output only. Server assigned timestamp for when the connection was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the repository.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The connection for the resource
        ///
        ///
        /// - - -
        pub parent_connection: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Required. Git Clone HTTPS URI.
        pub remote_uri: pulumi_wasm_rust::Output<String>,
        /// Output only. Server assigned timestamp for when the connection was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RepositoryArgs) -> RepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let parent_connection_binding = args.parent_connection.get_inner();
        let project_binding = args.project.get_inner();
        let remote_uri_binding = args.remote_uri.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudbuildv2/repository:Repository".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
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
                    name: "parentConnection".into(),
                    value: &parent_connection_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "remoteUri".into(),
                    value: &remote_uri_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parentConnection".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "remoteUri".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent_connection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentConnection").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            remote_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteUri").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
