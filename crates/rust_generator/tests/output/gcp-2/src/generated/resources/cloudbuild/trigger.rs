/// Configuration for an automated build in response to source repository changes.
///
///
/// To get more information about Trigger, see:
///
/// * [API documentation](https://cloud.google.com/cloud-build/docs/api/reference/rest/v1/projects.triggers)
/// * How-to Guides
///     * [Automating builds using build triggers](https://cloud.google.com/cloud-build/docs/running-builds/automate-builds)
///
/// > **Note:** You can retrieve the email of the Cloud Build Service Account used in jobs by using the `gcp.projects.ServiceIdentity` resource.
///
/// ## Example Usage
///
/// ### Cloudbuild Trigger Filename
///
///
/// ```yaml
/// resources:
///   filename-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       location: us-central1
///       triggerTemplate:
///         branchName: main
///         repoName: my-repo
///       substitutions:
///         _FOO: bar
///         _BAZ: qux
///       filename: cloudbuild.yaml
/// ```
/// ### Cloudbuild Trigger Build
///
///
/// ```yaml
/// resources:
///   build-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: my-trigger
///       location: global
///       triggerTemplate:
///         branchName: main
///         repoName: my-repo
///       build:
///         steps:
///           - name: gcr.io/cloud-builders/gsutil
///             args:
///               - cp
///               - gs://mybucket/remotefile.zip
///               - localfile.zip
///             timeout: 120s
///             secretEnvs:
///               - MY_SECRET
///           - name: ubuntu
///             script: echo hello
///         source:
///           storageSource:
///             bucket: mybucket
///             object: source_code.tar.gz
///         tags:
///           - build
///           - newFeature
///         substitutions:
///           _FOO: bar
///           _BAZ: qux
///         queueTtl: 20s
///         logsBucket: gs://mybucket/logs
///         secrets:
///           - kmsKeyName: projects/myProject/locations/global/keyRings/keyring-name/cryptoKeys/key-name
///             secretEnv:
///               PASSWORD: ZW5jcnlwdGVkLXBhc3N3b3JkCg==
///         availableSecrets:
///           secretManagers:
///             - env: MY_SECRET
///               versionName: projects/myProject/secrets/mySecret/versions/latest
///         artifacts:
///           images:
///             - gcr.io/$PROJECT_ID/$REPO_NAME:$COMMIT_SHA
///           objects:
///             location: gs://bucket/path/to/somewhere/
///             paths:
///               - path
///           npmPackages:
///             - packagePath: package.json
///               repository: https://us-west1-npm.pkg.dev/myProject/quickstart-nodejs-repo
///           pythonPackages:
///             - paths:
///                 - dist/*
///               repository: https://us-west1-python.pkg.dev/myProject/quickstart-python-repo
///           mavenArtifacts:
///             - repository: https://us-west1-maven.pkg.dev/myProject/quickstart-java-repo
///               path: /workspace/my-app/target/my-app-1.0.SNAPSHOT.jar
///               artifactId: my-app
///               groupId: com.mycompany.app
///               version: '1.0'
///         options:
///           sourceProvenanceHashes:
///             - MD5
///           requestedVerifyOption: VERIFIED
///           machineType: N1_HIGHCPU_8
///           diskSizeGb: 100
///           substitutionOption: ALLOW_LOOSE
///           dynamicSubstitutions: true
///           logStreamingOption: STREAM_OFF
///           workerPool: pool
///           logging: LEGACY
///           envs:
///             - ekey = evalue
///           secretEnvs:
///             - secretenv = svalue
///           volumes:
///             - name: v1
///               path: v1
/// ```
/// ### Cloudbuild Trigger Service Account
///
///
/// ```yaml
/// resources:
///   service-account-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       triggerTemplate:
///         branchName: main
///         repoName: my-repo
///       serviceAccount: ${cloudbuildServiceAccount.id}
///       filename: cloudbuild.yaml
///     options:
///       dependsOn:
///         - ${actAs}
///         - ${logsWriter}
///   cloudbuildServiceAccount:
///     type: gcp:serviceaccount:Account
///     name: cloudbuild_service_account
///     properties:
///       accountId: cloud-sa
///   actAs:
///     type: gcp:projects:IAMMember
///     name: act_as
///     properties:
///       project: ${project.projectId}
///       role: roles/iam.serviceAccountUser
///       member: serviceAccount:${cloudbuildServiceAccount.email}
///   logsWriter:
///     type: gcp:projects:IAMMember
///     name: logs_writer
///     properties:
///       project: ${project.projectId}
///       role: roles/logging.logWriter
///       member: serviceAccount:${cloudbuildServiceAccount.email}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Cloudbuild Trigger Include Build Logs
///
///
/// ```yaml
/// resources:
///   include-build-logs-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       location: us-central1
///       name: include-build-logs-trigger
///       filename: cloudbuild.yaml
///       github:
///         owner: hashicorp
///         name: terraform-provider-google-beta
///         push:
///           branch: ^main$
///       includeBuildLogs: INCLUDE_BUILD_LOGS_WITH_STATUS
/// ```
/// ### Cloudbuild Trigger Pubsub Config
///
///
/// ```yaml
/// resources:
///   mytopic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: my-topic
///   pubsub-config-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       location: us-central1
///       name: pubsub-trigger
///       description: acceptance test example pubsub build trigger
///       pubsubConfig:
///         topic: ${mytopic.id}
///       sourceToBuild:
///         uri: https://hashicorp/terraform-provider-google-beta
///         ref: refs/heads/main
///         repoType: GITHUB
///       gitFileSource:
///         path: cloudbuild.yaml
///         uri: https://hashicorp/terraform-provider-google-beta
///         revision: refs/heads/main
///         repoType: GITHUB
///       substitutions:
///         _ACTION: $(body.message.data.action)
///       filter: _ACTION.matches('INSERT')
/// ```
/// ### Cloudbuild Trigger Webhook Config
///
///
/// ```yaml
/// resources:
///   webhookTriggerSecretKey:
///     type: gcp:secretmanager:Secret
///     name: webhook_trigger_secret_key
///     properties:
///       secretId: webhook-trigger-secret-key
///       replication:
///         userManaged:
///           replicas:
///             - location: us-central1
///   webhookTriggerSecretKeyData:
///     type: gcp:secretmanager:SecretVersion
///     name: webhook_trigger_secret_key_data
///     properties:
///       secret: ${webhookTriggerSecretKey.id}
///       secretData: secretkeygoeshere
///   policy:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       project: ${webhookTriggerSecretKey.project}
///       secretId: ${webhookTriggerSecretKey.secretId}
///       policyData: ${secretAccessor.policyData}
///   webhook-config-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: webhook-trigger
///       description: acceptance test example webhook build trigger
///       webhookConfig:
///         secret: ${webhookTriggerSecretKeyData.id}
///       sourceToBuild:
///         uri: https://hashicorp/terraform-provider-google-beta
///         ref: refs/heads/main
///         repoType: GITHUB
///       gitFileSource:
///         path: cloudbuild.yaml
///         uri: https://hashicorp/terraform-provider-google-beta
///         revision: refs/heads/main
///         repoType: GITHUB
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
///   secretAccessor:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - serviceAccount:service-${project.number}@gcp-sa-cloudbuild.iam.gserviceaccount.com
/// ```
/// ### Cloudbuild Trigger Manual
///
///
/// ```yaml
/// resources:
///   manual-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: manual-trigger
///       sourceToBuild:
///         uri: https://hashicorp/terraform-provider-google-beta
///         ref: refs/heads/main
///         repoType: GITHUB
///       gitFileSource:
///         path: cloudbuild.yaml
///         uri: https://hashicorp/terraform-provider-google-beta
///         revision: refs/heads/main
///         repoType: GITHUB
///       approvalConfig:
///         approvalRequired: true
/// ```
/// ### Cloudbuild Trigger Manual Github Enterprise
///
///
/// ```yaml
/// resources:
///   manual-ghe-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: ""
///       sourceToBuild:
///         uri: https://hashicorp/terraform-provider-google-beta
///         ref: refs/heads/main
///         repoType: GITHUB
///         githubEnterpriseConfig: projects/myProject/locations/global/githubEnterpriseConfigs/configID
///       gitFileSource:
///         path: cloudbuild.yaml
///         uri: https://hashicorp/terraform-provider-google-beta
///         revision: refs/heads/main
///         repoType: GITHUB
///         githubEnterpriseConfig: projects/myProject/locations/global/githubEnterpriseConfigs/configID
/// ```
/// ### Cloudbuild Trigger Manual Bitbucket Server
///
///
/// ```yaml
/// resources:
///   manual-bitbucket-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: terraform-manual-bbs-trigger
///       sourceToBuild:
///         uri: https://bbs.com/scm/stag/test-repo.git
///         ref: refs/heads/main
///         repoType: BITBUCKET_SERVER
///         bitbucketServerConfig: projects/myProject/locations/global/bitbucketServerConfigs/configID
///       gitFileSource:
///         path: cloudbuild.yaml
///         uri: https://bbs.com/scm/stag/test-repo.git
///         revision: refs/heads/main
///         repoType: BITBUCKET_SERVER
///         bitbucketServerConfig: projects/myProject/locations/global/bitbucketServerConfigs/configID
/// ```
/// ### Cloudbuild Trigger Repo
///
///
/// ```yaml
/// resources:
///   my-connection:
///     type: gcp:cloudbuildv2:Connection
///     properties:
///       location: us-central1
///       name: my-connection
///       githubConfig:
///         appInstallationId: 123123
///         authorizerCredential:
///           oauthTokenSecretVersion: projects/my-project/secrets/github-pat-secret/versions/latest
///   my-repository:
///     type: gcp:cloudbuildv2:Repository
///     properties:
///       name: my-repo
///       parentConnection: ${["my-connection"].id}
///       remoteUri: https://github.com/myuser/my-repo.git
///   repo-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       location: us-central1
///       repositoryEventConfig:
///         repository: ${["my-repository"].id}
///         push:
///           branch: feature-.*
///       filename: cloudbuild.yaml
/// ```
/// ### Cloudbuild Trigger Bitbucket Server Push
///
///
/// ```yaml
/// resources:
///   bbs-push-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: bbs-push-trigger
///       location: us-central1
///       bitbucketServerTriggerConfig:
///         repoSlug: bbs-push-trigger
///         projectKey: STAG
///         bitbucketServerConfigResource: projects/123456789/locations/us-central1/bitbucketServerConfigs/myBitbucketConfig
///         push:
///           tag: ^0.1.*
///           invertRegex: true
///       filename: cloudbuild.yaml
/// ```
/// ### Cloudbuild Trigger Bitbucket Server Pull Request
///
///
/// ```yaml
/// resources:
///   bbs-pull-request-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: ghe-trigger
///       location: us-central1
///       bitbucketServerTriggerConfig:
///         repoSlug: terraform-provider-google
///         projectKey: STAG
///         bitbucketServerConfigResource: projects/123456789/locations/us-central1/bitbucketServerConfigs/myBitbucketConfig
///         pullRequest:
///           branch: ^master$
///           invertRegex: false
///           commentControl: COMMENTS_ENABLED
///       filename: cloudbuild.yaml
/// ```
/// ### Cloudbuild Trigger Github Enterprise
///
///
/// ```yaml
/// resources:
///   ghe-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: ghe-trigger
///       location: us-central1
///       github:
///         owner: hashicorp
///         name: terraform-provider-google
///         push:
///           branch: ^main$
///         enterpriseConfigResourceName: projects/123456789/locations/us-central1/githubEnterpriseConfigs/configID
///       filename: cloudbuild.yaml
/// ```
/// ### Cloudbuild Trigger Allow Failure
///
///
/// ```yaml
/// resources:
///   allow-failure-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: my-trigger
///       location: global
///       triggerTemplate:
///         branchName: main
///         repoName: my-repo
///       build:
///         steps:
///           - name: ubuntu
///             args:
///               - -c
///               - exit 1
///             allowFailure: true
///         source:
///           storageSource:
///             bucket: mybucket
///             object: source_code.tar.gz
///         tags:
///           - build
///           - newFeature
///         substitutions:
///           _FOO: bar
///           _BAZ: qux
///         queueTtl: 20s
///         logsBucket: gs://mybucket/logs
///         secrets:
///           - kmsKeyName: projects/myProject/locations/global/keyRings/keyring-name/cryptoKeys/key-name
///             secretEnv:
///               PASSWORD: ZW5jcnlwdGVkLXBhc3N3b3JkCg==
///         availableSecrets:
///           secretManagers:
///             - env: MY_SECRET
///               versionName: projects/myProject/secrets/mySecret/versions/latest
///         artifacts:
///           images:
///             - gcr.io/$PROJECT_ID/$REPO_NAME:$COMMIT_SHA
///           objects:
///             location: gs://bucket/path/to/somewhere/
///             paths:
///               - path
///         options:
///           sourceProvenanceHashes:
///             - MD5
///           requestedVerifyOption: VERIFIED
///           machineType: N1_HIGHCPU_8
///           diskSizeGb: 100
///           substitutionOption: ALLOW_LOOSE
///           dynamicSubstitutions: true
///           logStreamingOption: STREAM_OFF
///           workerPool: pool
///           logging: LEGACY
///           envs:
///             - ekey = evalue
///           secretEnvs:
///             - secretenv = svalue
///           volumes:
///             - name: v1
///               path: v1
/// ```
/// ### Cloudbuild Trigger Allow Exit Codes
///
///
/// ```yaml
/// resources:
///   allow-exit-codes-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: my-trigger
///       location: global
///       triggerTemplate:
///         branchName: main
///         repoName: my-repo
///       build:
///         steps:
///           - name: ubuntu
///             args:
///               - -c
///               - exit 1
///             allowExitCodes:
///               - 1
///               - 3
///         source:
///           storageSource:
///             bucket: mybucket
///             object: source_code.tar.gz
///         tags:
///           - build
///           - newFeature
///         substitutions:
///           _FOO: bar
///           _BAZ: qux
///         queueTtl: 20s
///         logsBucket: gs://mybucket/logs
///         secrets:
///           - kmsKeyName: projects/myProject/locations/global/keyRings/keyring-name/cryptoKeys/key-name
///             secretEnv:
///               PASSWORD: ZW5jcnlwdGVkLXBhc3N3b3JkCg==
///         availableSecrets:
///           secretManagers:
///             - env: MY_SECRET
///               versionName: projects/myProject/secrets/mySecret/versions/latest
///         artifacts:
///           images:
///             - gcr.io/$PROJECT_ID/$REPO_NAME:$COMMIT_SHA
///           objects:
///             location: gs://bucket/path/to/somewhere/
///             paths:
///               - path
///         options:
///           sourceProvenanceHashes:
///             - MD5
///           requestedVerifyOption: VERIFIED
///           machineType: N1_HIGHCPU_8
///           diskSizeGb: 100
///           substitutionOption: ALLOW_LOOSE
///           dynamicSubstitutions: true
///           logStreamingOption: STREAM_OFF
///           workerPool: pool
///           logging: LEGACY
///           envs:
///             - ekey = evalue
///           secretEnvs:
///             - secretenv = svalue
///           volumes:
///             - name: v1
///               path: v1
/// ```
/// ### Cloudbuild Trigger Pubsub With Repo
///
///
/// ```yaml
/// resources:
///   my-connection:
///     type: gcp:cloudbuildv2:Connection
///     properties:
///       location: us-central1
///       name: my-connection
///       githubConfig:
///         appInstallationId: 123123
///         authorizerCredential:
///           oauthTokenSecretVersion: projects/my-project/secrets/github-pat-secret/versions/latest
///   my-repository:
///     type: gcp:cloudbuildv2:Repository
///     properties:
///       name: my-repo
///       parentConnection: ${["my-connection"].id}
///       remoteUri: https://github.com/myuser/my-repo.git
///   mytopic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: my-topic
///   pubsub-with-repo-trigger:
///     type: gcp:cloudbuild:Trigger
///     properties:
///       name: pubsub-with-repo-trigger
///       location: us-central1
///       pubsubConfig:
///         topic: ${mytopic.id}
///       sourceToBuild:
///         repository: ${["my-repository"].id}
///         ref: refs/heads/main
///         repoType: GITHUB
///       gitFileSource:
///         path: cloudbuild.yaml
///         repository: ${["my-repository"].id}
///         revision: refs/heads/main
///         repoType: GITHUB
/// ```
///
/// ## Import
///
/// Trigger can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/triggers/{{trigger_id}}`
///
/// * `projects/{{project}}/triggers/{{trigger_id}}`
///
/// * `{{project}}/{{trigger_id}}`
///
/// * `{{trigger_id}}`
///
/// When using the `pulumi import` command, Trigger can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/trigger:Trigger default projects/{{project}}/locations/{{location}}/triggers/{{trigger_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/trigger:Trigger default projects/{{project}}/triggers/{{trigger_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/trigger:Trigger default {{project}}/{{trigger_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/trigger:Trigger default {{trigger_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trigger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerArgs {
        /// Configuration for manual approval to start a build invocation of this BuildTrigger.
        /// Builds created by this trigger will require approval before they execute.
        /// Any user with a Cloud Build Approver role for the project can approve a build.
        /// Structure is documented below.
        #[builder(into, default)]
        pub approval_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerApprovalConfig>,
        >,
        /// BitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bitbucket_server_trigger_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerBitbucketServerTriggerConfig>,
        >,
        /// Contents of the build template. Either a filename or build template must be provided.
        /// Structure is documented below.
        #[builder(into, default)]
        pub build: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerBuild>,
        >,
        /// Human-readable description of the trigger.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the trigger is disabled or not. If true, the trigger will never result in a build.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Path, from the source root, to a file whose contents is used for the template.
        /// Either a filename or build template must be provided. Set this only when using trigger_template or github.
        /// When using Pub/Sub, Webhook or Manual set the file name using git_file_source instead.
        #[builder(into, default)]
        pub filename: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A Common Expression Language string. Used only with Pub/Sub and Webhook.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The file source describing the local or remote Build template.
        /// Structure is documented below.
        #[builder(into, default)]
        pub git_file_source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerGitFileSource>,
        >,
        /// Describes the configuration of a trigger that creates a build whenever a GitHub event is received.
        /// One of `trigger_template`, `github`, `pubsub_config` or `webhook_config` must be provided.
        /// Structure is documented below.
        #[builder(into, default)]
        pub github: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerGithub>,
        >,
        /// ignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match
        /// extended with support for `**`.
        /// If ignoredFiles and changed files are both empty, then they are not
        /// used to determine whether or not to trigger a build.
        /// If ignoredFiles is not empty, then we ignore any files that match any
        /// of the ignored_file globs. If the change has no files that are outside
        /// of the ignoredFiles globs, then we do not trigger a build.
        #[builder(into, default)]
        pub ignored_files: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Build logs will be sent back to GitHub as part of the checkrun
        /// result.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or
        /// INCLUDE_BUILD_LOGS_WITH_STATUS
        /// Possible values are: `INCLUDE_BUILD_LOGS_UNSPECIFIED`, `INCLUDE_BUILD_LOGS_WITH_STATUS`.
        #[builder(into, default)]
        pub include_build_logs: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match
        /// extended with support for `**`.
        /// If any of the files altered in the commit pass the ignoredFiles filter
        /// and includedFiles is empty, then as far as this filter is concerned, we
        /// should trigger the build.
        /// If any of the files altered in the commit pass the ignoredFiles filter
        /// and includedFiles is not empty, then we make sure that at least one of
        /// those files matches a includedFiles glob. If not, then we do not trigger
        /// a build.
        #[builder(into, default)]
        pub included_files: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.
        /// If not specified, "global" is used.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the trigger. Must be unique within the project.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// PubsubConfig describes the configuration of a trigger that creates
        /// a build whenever a Pub/Sub message is published.
        /// One of `trigger_template`, `github`, `pubsub_config` `webhook_config` or `source_to_build` must be provided.
        /// Structure is documented below.
        #[builder(into, default)]
        pub pubsub_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerPubsubConfig>,
        >,
        /// The configuration of a trigger that creates a build whenever an event from Repo API is received.
        /// Structure is documented below.
        #[builder(into, default)]
        pub repository_event_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerRepositoryEventConfig>,
        >,
        /// The service account used for all user-controlled operations including
        /// triggers.patch, triggers.run, builds.create, and builds.cancel.
        /// If no service account is set, then the standard Cloud Build service account
        /// ([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.
        /// Format: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The repo and ref of the repository from which to build.
        /// This field is used only for those triggers that do not respond to SCM events.
        /// Triggers that respond to such events build source at whatever commit caused the event.
        /// This field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers.
        /// One of `trigger_template`, `github`, `pubsub_config` `webhook_config` or `source_to_build` must be provided.
        /// Structure is documented below.
        #[builder(into, default)]
        pub source_to_build: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerSourceToBuild>,
        >,
        /// Substitutions data for Build resource.
        #[builder(into, default)]
        pub substitutions: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Tags for annotation of a BuildTrigger
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Template describing the types of source changes to trigger a build.
        /// Branch and tag names in trigger templates are interpreted as regular
        /// expressions. Any branch or tag change that matches that regular
        /// expression will trigger a build.
        /// One of `trigger_template`, `github`, `pubsub_config`, `webhook_config` or `source_to_build` must be provided.
        /// Structure is documented below.
        #[builder(into, default)]
        pub trigger_template: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerTriggerTemplate>,
        >,
        /// WebhookConfig describes the configuration of a trigger that creates
        /// a build whenever a webhook is sent to a trigger's webhook URL.
        /// One of `trigger_template`, `github`, `pubsub_config` `webhook_config` or `source_to_build` must be provided.
        /// Structure is documented below.
        #[builder(into, default)]
        pub webhook_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::TriggerWebhookConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct TriggerResult {
        /// Configuration for manual approval to start a build invocation of this BuildTrigger.
        /// Builds created by this trigger will require approval before they execute.
        /// Any user with a Cloud Build Approver role for the project can approve a build.
        /// Structure is documented below.
        pub approval_config: pulumi_gestalt_rust::Output<
            super::super::types::cloudbuild::TriggerApprovalConfig,
        >,
        /// BitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received.
        /// Structure is documented below.
        pub bitbucket_server_trigger_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::TriggerBitbucketServerTriggerConfig>,
        >,
        /// Contents of the build template. Either a filename or build template must be provided.
        /// Structure is documented below.
        pub build: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::TriggerBuild>,
        >,
        /// Time when the trigger was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Human-readable description of the trigger.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the trigger is disabled or not. If true, the trigger will never result in a build.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Path, from the source root, to a file whose contents is used for the template.
        /// Either a filename or build template must be provided. Set this only when using trigger_template or github.
        /// When using Pub/Sub, Webhook or Manual set the file name using git_file_source instead.
        pub filename: pulumi_gestalt_rust::Output<Option<String>>,
        /// A Common Expression Language string. Used only with Pub/Sub and Webhook.
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The file source describing the local or remote Build template.
        /// Structure is documented below.
        pub git_file_source: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::TriggerGitFileSource>,
        >,
        /// Describes the configuration of a trigger that creates a build whenever a GitHub event is received.
        /// One of `trigger_template`, `github`, `pubsub_config` or `webhook_config` must be provided.
        /// Structure is documented below.
        pub github: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::TriggerGithub>,
        >,
        /// ignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match
        /// extended with support for `**`.
        /// If ignoredFiles and changed files are both empty, then they are not
        /// used to determine whether or not to trigger a build.
        /// If ignoredFiles is not empty, then we ignore any files that match any
        /// of the ignored_file globs. If the change has no files that are outside
        /// of the ignoredFiles globs, then we do not trigger a build.
        pub ignored_files: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Build logs will be sent back to GitHub as part of the checkrun
        /// result.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or
        /// INCLUDE_BUILD_LOGS_WITH_STATUS
        /// Possible values are: `INCLUDE_BUILD_LOGS_UNSPECIFIED`, `INCLUDE_BUILD_LOGS_WITH_STATUS`.
        pub include_build_logs: pulumi_gestalt_rust::Output<Option<String>>,
        /// ignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match
        /// extended with support for `**`.
        /// If any of the files altered in the commit pass the ignoredFiles filter
        /// and includedFiles is empty, then as far as this filter is concerned, we
        /// should trigger the build.
        /// If any of the files altered in the commit pass the ignoredFiles filter
        /// and includedFiles is not empty, then we make sure that at least one of
        /// those files matches a includedFiles glob. If not, then we do not trigger
        /// a build.
        pub included_files: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.
        /// If not specified, "global" is used.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the trigger. Must be unique within the project.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// PubsubConfig describes the configuration of a trigger that creates
        /// a build whenever a Pub/Sub message is published.
        /// One of `trigger_template`, `github`, `pubsub_config` `webhook_config` or `source_to_build` must be provided.
        /// Structure is documented below.
        pub pubsub_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::TriggerPubsubConfig>,
        >,
        /// The configuration of a trigger that creates a build whenever an event from Repo API is received.
        /// Structure is documented below.
        pub repository_event_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::TriggerRepositoryEventConfig>,
        >,
        /// The service account used for all user-controlled operations including
        /// triggers.patch, triggers.run, builds.create, and builds.cancel.
        /// If no service account is set, then the standard Cloud Build service account
        /// ([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.
        /// Format: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}
        pub service_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// The repo and ref of the repository from which to build.
        /// This field is used only for those triggers that do not respond to SCM events.
        /// Triggers that respond to such events build source at whatever commit caused the event.
        /// This field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers.
        /// One of `trigger_template`, `github`, `pubsub_config` `webhook_config` or `source_to_build` must be provided.
        /// Structure is documented below.
        pub source_to_build: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::TriggerSourceToBuild>,
        >,
        /// Substitutions data for Build resource.
        pub substitutions: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Tags for annotation of a BuildTrigger
        pub tags: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The unique identifier for the trigger.
        pub trigger_id: pulumi_gestalt_rust::Output<String>,
        /// Template describing the types of source changes to trigger a build.
        /// Branch and tag names in trigger templates are interpreted as regular
        /// expressions. Any branch or tag change that matches that regular
        /// expression will trigger a build.
        /// One of `trigger_template`, `github`, `pubsub_config`, `webhook_config` or `source_to_build` must be provided.
        /// Structure is documented below.
        pub trigger_template: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::TriggerTriggerTemplate>,
        >,
        /// WebhookConfig describes the configuration of a trigger that creates
        /// a build whenever a webhook is sent to a trigger's webhook URL.
        /// One of `trigger_template`, `github`, `pubsub_config` `webhook_config` or `source_to_build` must be provided.
        /// Structure is documented below.
        pub webhook_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::TriggerWebhookConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerArgs,
    ) -> TriggerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let approval_config_binding = args.approval_config.get_output(context);
        let bitbucket_server_trigger_config_binding = args
            .bitbucket_server_trigger_config
            .get_output(context);
        let build_binding = args.build.get_output(context);
        let description_binding = args.description.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let filename_binding = args.filename.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let git_file_source_binding = args.git_file_source.get_output(context);
        let github_binding = args.github.get_output(context);
        let ignored_files_binding = args.ignored_files.get_output(context);
        let include_build_logs_binding = args.include_build_logs.get_output(context);
        let included_files_binding = args.included_files.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let pubsub_config_binding = args.pubsub_config.get_output(context);
        let repository_event_config_binding = args
            .repository_event_config
            .get_output(context);
        let service_account_binding = args.service_account.get_output(context);
        let source_to_build_binding = args.source_to_build.get_output(context);
        let substitutions_binding = args.substitutions.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let trigger_template_binding = args.trigger_template.get_output(context);
        let webhook_config_binding = args.webhook_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudbuild/trigger:Trigger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvalConfig".into(),
                    value: approval_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bitbucketServerTriggerConfig".into(),
                    value: bitbucket_server_trigger_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "build".into(),
                    value: build_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filename".into(),
                    value: filename_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gitFileSource".into(),
                    value: git_file_source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "github".into(),
                    value: github_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoredFiles".into(),
                    value: ignored_files_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeBuildLogs".into(),
                    value: include_build_logs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includedFiles".into(),
                    value: included_files_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pubsubConfig".into(),
                    value: pubsub_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryEventConfig".into(),
                    value: repository_event_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccount".into(),
                    value: service_account_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceToBuild".into(),
                    value: source_to_build_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "substitutions".into(),
                    value: substitutions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggerTemplate".into(),
                    value: trigger_template_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webhookConfig".into(),
                    value: webhook_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TriggerResult {
            approval_config: o.get_field("approvalConfig"),
            bitbucket_server_trigger_config: o.get_field("bitbucketServerTriggerConfig"),
            build: o.get_field("build"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            disabled: o.get_field("disabled"),
            filename: o.get_field("filename"),
            filter: o.get_field("filter"),
            git_file_source: o.get_field("gitFileSource"),
            github: o.get_field("github"),
            ignored_files: o.get_field("ignoredFiles"),
            include_build_logs: o.get_field("includeBuildLogs"),
            included_files: o.get_field("includedFiles"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pubsub_config: o.get_field("pubsubConfig"),
            repository_event_config: o.get_field("repositoryEventConfig"),
            service_account: o.get_field("serviceAccount"),
            source_to_build: o.get_field("sourceToBuild"),
            substitutions: o.get_field("substitutions"),
            tags: o.get_field("tags"),
            trigger_id: o.get_field("triggerId"),
            trigger_template: o.get_field("triggerTemplate"),
            webhook_config: o.get_field("webhookConfig"),
        }
    }
}
