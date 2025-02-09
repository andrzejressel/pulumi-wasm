/// A repository for storing artifacts
///
///
/// To get more information about Repository, see:
///
/// * [API documentation](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/artifact-registry/docs/overview)
///
/// ## Example Usage
///
/// ### Artifact Registry Repository Basic
///
///
/// ```yaml
/// resources:
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: my-repository
///       description: example docker repository
///       format: DOCKER
/// ```
/// ### Artifact Registry Repository Multi Region
///
///
/// ```yaml
/// resources:
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       repositoryId: my-repository
///       description: example docker repository
///       location: us
///       format: DOCKER
/// ```
/// ### Artifact Registry Repository Docker
///
///
/// ```yaml
/// resources:
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: my-repository
///       description: example docker repository
///       format: DOCKER
///       dockerConfig:
///         immutableTags: true
/// ```
/// ### Artifact Registry Repository Cmek
///
///
/// ```yaml
/// resources:
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: my-repository
///       description: example docker repository with cmek
///       format: DOCKER
///       kmsKeyName: kms-key
///     options:
///       dependsOn:
///         - ${cryptoKey}
///   cryptoKey:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key
///     properties:
///       cryptoKeyId: kms-key
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-artifactregistry.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Artifact Registry Repository Virtual
///
///
/// ```yaml
/// resources:
///   my-repo-upstream-1:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: my-repository-upstream-1
///       description: example docker repository (upstream source) 1
///       format: DOCKER
///   my-repo-upstream-2:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: my-repository-upstream-2
///       description: example docker repository (upstream source) 2
///       format: DOCKER
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: my-repository
///       description: example virtual docker repository
///       format: DOCKER
///       mode: VIRTUAL_REPOSITORY
///       virtualRepositoryConfig:
///         upstreamPolicies:
///           - id: my-repository-upstream-1
///             repository: ${["my-repo-upstream-1"].id}
///             priority: 20
///           - id: my-repository-upstream-2
///             repository: ${["my-repo-upstream-2"].id}
///             priority: 10
/// ```
/// ### Artifact Registry Repository Remote
///
///
/// ```yaml
/// resources:
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: my-repository
///       description: example remote docker repository
///       format: DOCKER
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: docker hub
///         dockerRepository:
///           publicRepository: DOCKER_HUB
/// ```
/// ### Artifact Registry Repository Remote Apt
///
///
/// ```yaml
/// resources:
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: debian-buster
///       description: example remote apt repository
///       format: APT
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: Debian buster remote repository
///         aptRepository:
///           publicRepository:
///             repositoryBase: DEBIAN
///             repositoryPath: debian/dists/buster
/// ```
/// ### Artifact Registry Repository Remote Yum
///
///
/// ```yaml
/// resources:
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: rocky-9
///       description: example remote yum repository
///       format: YUM
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: Rocky 9 remote repository
///         yumRepository:
///           publicRepository:
///             repositoryBase: ROCKY
///             repositoryPath: pub/rocky/9/BaseOS/x86_64/os
/// ```
/// ### Artifact Registry Repository Cleanup
///
///
/// ```yaml
/// resources:
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: my-repository
///       description: example docker repository with cleanup policies
///       format: DOCKER
///       cleanupPolicyDryRun: false
///       cleanupPolicies:
///         - id: delete-prerelease
///           action: DELETE
///           condition:
///             tagState: TAGGED
///             tagPrefixes:
///               - alpha
///               - v0
///             olderThan: 2592000s
///         - id: keep-tagged-release
///           action: KEEP
///           condition:
///             tagState: TAGGED
///             tagPrefixes:
///               - release
///             packageNamePrefixes:
///               - webapp
///               - mobile
///         - id: keep-minimum-versions
///           action: KEEP
///           mostRecentVersions:
///             packageNamePrefixes:
///               - webapp
///               - mobile
///               - sandbox
///             keepCount: 5
/// ```
/// ### Artifact Registry Repository Remote Dockerhub Auth
///
///
/// ```yaml
/// resources:
///   example-remote-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: example-secret
///       replication:
///         auto: {}
///   example-remote-secretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: example-remote-secret_version
///     properties:
///       secret: ${["example-remote-secret"].id}
///       secretData: remote-password
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${["example-remote-secret"].id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:service-${project.number}@gcp-sa-artifactregistry.iam.gserviceaccount.com
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: example-dockerhub-remote
///       description: example remote dockerhub repository with credentials
///       format: DOCKER
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: docker hub with custom credentials
///         disableUpstreamValidation: true
///         dockerRepository:
///           publicRepository: DOCKER_HUB
///         upstreamCredentials:
///           usernamePasswordCredentials:
///             username: remote-username
///             passwordSecretVersion: ${["example-remote-secretVersion"].name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Artifact Registry Repository Remote Docker Custom With Auth
///
///
/// ```yaml
/// resources:
///   example-remote-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: example-secret
///       replication:
///         auto: {}
///   example-remote-secretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: example-remote-secret_version
///     properties:
///       secret: ${["example-remote-secret"].id}
///       secretData: remote-password
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${["example-remote-secret"].id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:service-${project.number}@gcp-sa-artifactregistry.iam.gserviceaccount.com
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: example-docker-custom-remote
///       description: example remote custom docker repository with credentials
///       format: DOCKER
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: custom docker remote with credentials
///         disableUpstreamValidation: true
///         dockerRepository:
///           customRepository:
///             uri: https://registry-1.docker.io
///         upstreamCredentials:
///           usernamePasswordCredentials:
///             username: remote-username
///             passwordSecretVersion: ${["example-remote-secretVersion"].name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Artifact Registry Repository Remote Maven Custom With Auth
///
///
/// ```yaml
/// resources:
///   example-remote-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: example-secret
///       replication:
///         auto: {}
///   example-remote-secretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: example-remote-secret_version
///     properties:
///       secret: ${["example-remote-secret"].id}
///       secretData: remote-password
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${["example-remote-secret"].id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:service-${project.number}@gcp-sa-artifactregistry.iam.gserviceaccount.com
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: example-maven-custom-remote
///       description: example remote custom maven repository with credentials
///       format: MAVEN
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: custom maven remote with credentials
///         disableUpstreamValidation: true
///         mavenRepository:
///           customRepository:
///             uri: https://my.maven.registry
///         upstreamCredentials:
///           usernamePasswordCredentials:
///             username: remote-username
///             passwordSecretVersion: ${["example-remote-secretVersion"].name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Artifact Registry Repository Remote Npm Custom With Auth
///
///
/// ```yaml
/// resources:
///   example-remote-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: example-secret
///       replication:
///         auto: {}
///   example-remote-secretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: example-remote-secret_version
///     properties:
///       secret: ${["example-remote-secret"].id}
///       secretData: remote-password
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${["example-remote-secret"].id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:service-${project.number}@gcp-sa-artifactregistry.iam.gserviceaccount.com
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: example-npm-custom-remote
///       description: example remote custom npm repository with credentials
///       format: NPM
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: custom npm with credentials
///         disableUpstreamValidation: true
///         npmRepository:
///           customRepository:
///             uri: https://my.npm.registry
///         upstreamCredentials:
///           usernamePasswordCredentials:
///             username: remote-username
///             passwordSecretVersion: ${["example-remote-secretVersion"].name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Artifact Registry Repository Remote Python Custom With Auth
///
///
/// ```yaml
/// resources:
///   example-remote-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: example-secret
///       replication:
///         auto: {}
///   example-remote-secretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: example-remote-secret_version
///     properties:
///       secret: ${["example-remote-secret"].id}
///       secretData: remote-password
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${["example-remote-secret"].id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:service-${project.number}@gcp-sa-artifactregistry.iam.gserviceaccount.com
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: example-python-custom-remote
///       description: example remote custom python repository with credentials
///       format: PYTHON
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: custom npm with credentials
///         disableUpstreamValidation: true
///         pythonRepository:
///           customRepository:
///             uri: https://my.python.registry
///         upstreamCredentials:
///           usernamePasswordCredentials:
///             username: remote-username
///             passwordSecretVersion: ${["example-remote-secretVersion"].name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Artifact Registry Repository Remote Common Repository With Docker
///
///
/// ```yaml
/// resources:
///   upstreamRepo:
///     type: gcp:artifactregistry:Repository
///     name: upstream_repo
///     properties:
///       location: us-central1
///       repositoryId: example-upstream-repo
///       description: example upstream repository
///       format: DOCKER
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: example-common-remote
///       description: example remote common repository with docker upstream
///       format: DOCKER
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: pull-through cache of another Artifact Registry repository
///         commonRepository:
///           uri: ${upstreamRepo.id}
/// ```
/// ### Artifact Registry Repository Remote Common Repository With Artifact Registry Uri
///
///
/// ```yaml
/// resources:
///   upstreamRepo:
///     type: gcp:artifactregistry:Repository
///     name: upstream_repo
///     properties:
///       location: us-central1
///       repositoryId: example-upstream-repo
///       description: example upstream repository
///       format: DOCKER
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: example-common-remote
///       description: example remote common repository with docker upstream
///       format: DOCKER
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: pull-through cache of another Artifact Registry repository by URL
///         commonRepository:
///           uri: https://us-central1-docker.pkg.dev//example-upstream-repo
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Artifact Registry Repository Remote Common Repository With Custom Upstream
///
///
/// ```yaml
/// resources:
///   example-remote-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: example-secret
///       replication:
///         auto: {}
///   example-remote-secretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: example-remote-secret_version
///     properties:
///       secret: ${["example-remote-secret"].id}
///       secretData: remote-password
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${["example-remote-secret"].id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:service-${project.number}@gcp-sa-artifactregistry.iam.gserviceaccount.com
///   my-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: example-docker-custom-remote
///       description: example remote custom docker repository with credentials
///       format: DOCKER
///       mode: REMOTE_REPOSITORY
///       remoteRepositoryConfig:
///         description: custom common docker remote with credentials
///         disableUpstreamValidation: true
///         commonRepository:
///           uri: https://registry-1.docker.io
///         upstreamCredentials:
///           usernamePasswordCredentials:
///             username: remote-username
///             passwordSecretVersion: ${["example-remote-secretVersion"].name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Repository can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}`
///
/// * `{{project}}/{{location}}/{{repository_id}}`
///
/// * `{{location}}/{{repository_id}}`
///
/// When using the `pulumi import` command, Repository can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/repository:Repository default projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/repository:Repository default {{project}}/{{location}}/{{repository_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/repository:Repository default {{location}}/{{repository_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Cleanup policies for this repository. Cleanup policies indicate when
        /// certain package versions can be automatically deleted.
        /// Map keys are policy IDs supplied by users during policy creation. They must
        /// unique within a repository and be under 128 characters in length.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cleanup_policies: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::artifactregistry::RepositoryCleanupPolicy>>,
        >,
        /// If true, the cleanup pipeline is prevented from deleting versions in this
        /// repository.
        #[builder(into, default)]
        pub cleanup_policy_dry_run: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The user-provided description of the repository.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Docker repository config contains repository level configuration for the repositories of docker type.
        /// Structure is documented below.
        #[builder(into, default)]
        pub docker_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::artifactregistry::RepositoryDockerConfig>,
        >,
        /// The format of packages that are stored in the repository. Supported formats
        /// can be found [here](https://cloud.google.com/artifact-registry/docs/supported-formats).
        /// You can only create alpha formats if you are a member of the
        /// [alpha user group](https://cloud.google.com/artifact-registry/docs/supported-formats#alpha-access).
        #[builder(into)]
        pub format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Cloud KMS resource name of the customer managed encryption key that’s
        /// used to encrypt the contents of the Repository. Has the form:
        /// `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`.
        /// This value may not be changed after the Repository has been created.
        #[builder(into, default)]
        pub kms_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels with user-defined metadata.
        /// This field may contain up to 64 entries. Label keys and values may be no
        /// longer than 63 characters. Label keys must begin with a lowercase letter
        /// and may only contain lowercase letters, numeric characters, underscores,
        /// and dashes.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the repository's location. In addition to specific regions,
        /// special values for multi-region locations are `asia`, `europe`, and `us`.
        /// See [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),
        /// or use the
        /// gcp.artifactregistry.getLocations
        /// data source for possible values.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// MavenRepositoryConfig is maven related repository details.
        /// Provides additional configuration details for repositories of the maven
        /// format type.
        /// Structure is documented below.
        #[builder(into, default)]
        pub maven_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::artifactregistry::RepositoryMavenConfig>,
        >,
        /// The mode configures the repository to serve artifacts from different sources.
        /// Default value is `STANDARD_REPOSITORY`.
        /// Possible values are: `STANDARD_REPOSITORY`, `VIRTUAL_REPOSITORY`, `REMOTE_REPOSITORY`.
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration specific for a Remote Repository.
        /// Structure is documented below.
        #[builder(into, default)]
        pub remote_repository_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::artifactregistry::RepositoryRemoteRepositoryConfig,
            >,
        >,
        /// The last part of the repository name, for example:
        /// "repo1"
        ///
        ///
        /// - - -
        #[builder(into)]
        pub repository_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration specific for a Virtual Repository.
        /// Structure is documented below.
        #[builder(into, default)]
        pub virtual_repository_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::artifactregistry::RepositoryVirtualRepositoryConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Cleanup policies for this repository. Cleanup policies indicate when
        /// certain package versions can be automatically deleted.
        /// Map keys are policy IDs supplied by users during policy creation. They must
        /// unique within a repository and be under 128 characters in length.
        /// Structure is documented below.
        pub cleanup_policies: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::artifactregistry::RepositoryCleanupPolicy>>,
        >,
        /// If true, the cleanup pipeline is prevented from deleting versions in this
        /// repository.
        pub cleanup_policy_dry_run: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The time when the repository was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The user-provided description of the repository.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Docker repository config contains repository level configuration for the repositories of docker type.
        /// Structure is documented below.
        pub docker_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::artifactregistry::RepositoryDockerConfig>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The format of packages that are stored in the repository. Supported formats
        /// can be found [here](https://cloud.google.com/artifact-registry/docs/supported-formats).
        /// You can only create alpha formats if you are a member of the
        /// [alpha user group](https://cloud.google.com/artifact-registry/docs/supported-formats#alpha-access).
        pub format: pulumi_gestalt_rust::Output<String>,
        /// The Cloud KMS resource name of the customer managed encryption key that’s
        /// used to encrypt the contents of the Repository. Has the form:
        /// `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`.
        /// This value may not be changed after the Repository has been created.
        pub kms_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Labels with user-defined metadata.
        /// This field may contain up to 64 entries. Label keys and values may be no
        /// longer than 63 characters. Label keys must begin with a lowercase letter
        /// and may only contain lowercase letters, numeric characters, underscores,
        /// and dashes.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the repository's location. In addition to specific regions,
        /// special values for multi-region locations are `asia`, `europe`, and `us`.
        /// See [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),
        /// or use the
        /// gcp.artifactregistry.getLocations
        /// data source for possible values.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// MavenRepositoryConfig is maven related repository details.
        /// Provides additional configuration details for repositories of the maven
        /// format type.
        /// Structure is documented below.
        pub maven_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::artifactregistry::RepositoryMavenConfig>,
        >,
        /// The mode configures the repository to serve artifacts from different sources.
        /// Default value is `STANDARD_REPOSITORY`.
        /// Possible values are: `STANDARD_REPOSITORY`, `VIRTUAL_REPOSITORY`, `REMOTE_REPOSITORY`.
        pub mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the repository, for example:
        /// "repo1"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration specific for a Remote Repository.
        /// Structure is documented below.
        pub remote_repository_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::artifactregistry::RepositoryRemoteRepositoryConfig,
            >,
        >,
        /// The last part of the repository name, for example:
        /// "repo1"
        ///
        ///
        /// - - -
        pub repository_id: pulumi_gestalt_rust::Output<String>,
        /// The time when the repository was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Configuration specific for a Virtual Repository.
        /// Structure is documented below.
        pub virtual_repository_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::artifactregistry::RepositoryVirtualRepositoryConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryArgs,
    ) -> RepositoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cleanup_policies_binding = args.cleanup_policies.get_output(context);
        let cleanup_policy_dry_run_binding = args
            .cleanup_policy_dry_run
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let docker_config_binding = args.docker_config.get_output(context);
        let format_binding = args.format.get_output(context);
        let kms_key_name_binding = args.kms_key_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let maven_config_binding = args.maven_config.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let project_binding = args.project.get_output(context);
        let remote_repository_config_binding = args
            .remote_repository_config
            .get_output(context);
        let repository_id_binding = args.repository_id.get_output(context);
        let virtual_repository_config_binding = args
            .virtual_repository_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:artifactregistry/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cleanupPolicies".into(),
                    value: cleanup_policies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cleanupPolicyDryRun".into(),
                    value: cleanup_policy_dry_run_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dockerConfig".into(),
                    value: docker_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "format".into(),
                    value: format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyName".into(),
                    value: kms_key_name_binding.get_id(),
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
                    name: "mavenConfig".into(),
                    value: maven_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteRepositoryConfig".into(),
                    value: remote_repository_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryId".into(),
                    value: repository_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualRepositoryConfig".into(),
                    value: virtual_repository_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryResult {
            cleanup_policies: o.get_field("cleanupPolicies"),
            cleanup_policy_dry_run: o.get_field("cleanupPolicyDryRun"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            docker_config: o.get_field("dockerConfig"),
            effective_labels: o.get_field("effectiveLabels"),
            format: o.get_field("format"),
            kms_key_name: o.get_field("kmsKeyName"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            maven_config: o.get_field("mavenConfig"),
            mode: o.get_field("mode"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            remote_repository_config: o.get_field("remoteRepositoryConfig"),
            repository_id: o.get_field("repositoryId"),
            update_time: o.get_field("updateTime"),
            virtual_repository_config: o.get_field("virtualRepositoryConfig"),
        }
    }
}
