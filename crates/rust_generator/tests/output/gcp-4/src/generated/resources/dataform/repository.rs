/// ## Example Usage
///
/// ### Dataform Repository
///
///
/// ```yaml
/// resources:
///   secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: my-secret
///       replication:
///         auto: {}
///   secretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: secret_version
///     properties:
///       secret: ${secret.id}
///       secretData: secret-data
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: example-key-ring
///       location: us-central1
///   exampleKey:
///     type: gcp:kms:CryptoKey
///     name: example_key
///     properties:
///       name: example-crypto-key-name
///       keyRing: ${keyring.id}
///   cryptoKeyBinding:
///     type: gcp:kms:CryptoKeyIAMBinding
///     name: crypto_key_binding
///     properties:
///       cryptoKeyId: ${exampleKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       members:
///         - serviceAccount:service-${project.number}@gcp-sa-dataform.iam.gserviceaccount.com
///   dataformRepository:
///     type: gcp:dataform:Repository
///     name: dataform_repository
///     properties:
///       name: dataform_repository
///       displayName: dataform_repository
///       npmrcEnvironmentVariablesSecretVersion: ${secretVersion.id}
///       kmsKeyName: ${exampleKey.id}
///       labels:
///         label_foo1: label-bar1
///       gitRemoteSettings:
///         url: https://github.com/OWNER/REPOSITORY.git
///         defaultBranch: main
///         authenticationTokenSecretVersion: ${secretVersion.id}
///       workspaceCompilationOverrides:
///         defaultDatabase: database
///         schemaSuffix: _suffix
///         tablePrefix: prefix_
///     options:
///       dependsOn:
///         - ${cryptoKeyBinding}
/// ```
///
/// ## Import
///
/// Repository can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/repositories/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Repository can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataform/repository:Repository default projects/{{project}}/locations/{{region}}/repositories/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repository:Repository default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repository:Repository default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repository:Repository default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Optional. The repository's user-friendly name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. If set, configures this repository to be linked to a Git remote.
        /// Structure is documented below.
        #[builder(into, default)]
        pub git_remote_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataform::RepositoryGitRemoteSettings>,
        >,
        /// Optional. The reference to a KMS encryption key. If provided, it will be used to encrypt user data in the repository and all child resources.
        /// It is not possible to add or update the encryption key after the repository is created. Example projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]
        #[builder(into, default)]
        pub kms_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Repository user labels.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The repository's name.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. The name of the Secret Manager secret version to be used to interpolate variables into the .npmrc file for package installation operations. Must be in the format projects/*/secrets/*/versions/*. The file itself must be in a JSON format.
        #[builder(into, default)]
        pub npmrc_environment_variables_secret_version: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the region
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The service account to run workflow invocations under.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set, fields of workspaceCompilationOverrides override the default compilation settings that are specified in dataform.json when creating workspace-scoped compilation results.
        /// Structure is documented below.
        #[builder(into, default)]
        pub workspace_compilation_overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataform::RepositoryWorkspaceCompilationOverrides,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Optional. The repository's user-friendly name.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. If set, configures this repository to be linked to a Git remote.
        /// Structure is documented below.
        pub git_remote_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataform::RepositoryGitRemoteSettings>,
        >,
        /// Optional. The reference to a KMS encryption key. If provided, it will be used to encrypt user data in the repository and all child resources.
        /// It is not possible to add or update the encryption key after the repository is created. Example projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]
        pub kms_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Repository user labels.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The repository's name.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Optional. The name of the Secret Manager secret version to be used to interpolate variables into the .npmrc file for package installation operations. Must be in the format projects/*/secrets/*/versions/*. The file itself must be in a JSON format.
        pub npmrc_environment_variables_secret_version: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A reference to the region
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The service account to run workflow invocations under.
        pub service_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// If set, fields of workspaceCompilationOverrides override the default compilation settings that are specified in dataform.json when creating workspace-scoped compilation results.
        /// Structure is documented below.
        pub workspace_compilation_overrides: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataform::RepositoryWorkspaceCompilationOverrides,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RepositoryArgs,
    ) -> RepositoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let git_remote_settings_binding = args
            .git_remote_settings
            .get_output(context)
            .get_inner();
        let kms_key_name_binding = args.kms_key_name.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let npmrc_environment_variables_secret_version_binding = args
            .npmrc_environment_variables_secret_version
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let service_account_binding = args
            .service_account
            .get_output(context)
            .get_inner();
        let workspace_compilation_overrides_binding = args
            .workspace_compilation_overrides
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataform/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "gitRemoteSettings".into(),
                    value: &git_remote_settings_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyName".into(),
                    value: &kms_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "npmrcEnvironmentVariablesSecretVersion".into(),
                    value: &npmrc_environment_variables_secret_version_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceCompilationOverrides".into(),
                    value: &workspace_compilation_overrides_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            git_remote_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gitRemoteSettings"),
            ),
            kms_key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            npmrc_environment_variables_secret_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("npmrcEnvironmentVariablesSecretVersion"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccount"),
            ),
            workspace_compilation_overrides: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceCompilationOverrides"),
            ),
        }
    }
}
