/// A Cloud Function that contains user computation executed in response to an event.
///
///
/// To get more information about function, see:
///
/// * [API documentation](https://cloud.google.com/functions/docs/reference/rest/v2beta/projects.locations.functions)
///
/// ## Example Usage
///
/// ### Cloudfunctions2 Basic
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: function-v2
///       location: us-central1
///       description: a new function
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloHttp
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///       serviceConfig:
///         maxInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
/// variables:
///   project: my-project-name
/// ```
/// ### Cloudfunctions2 Full
///
///
/// ```yaml
/// resources:
///   account:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: gcf-sa
///       displayName: Test Service Account
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: functions2-topic
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: gcf-function
///       location: us-central1
///       description: a new function
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloPubSub
///         environmentVariables:
///           BUILD_CONFIG_TEST: build_test
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///       serviceConfig:
///         maxInstanceCount: 3
///         minInstanceCount: 1
///         availableMemory: 4Gi
///         timeoutSeconds: 60
///         maxInstanceRequestConcurrency: 80
///         availableCpu: '4'
///         environmentVariables:
///           SERVICE_CONFIG_TEST: config_test
///           SERVICE_CONFIG_DIFF_TEST: ${account.email}
///         ingressSettings: ALLOW_INTERNAL_ONLY
///         allTrafficOnLatestRevision: true
///         serviceAccountEmail: ${account.email}
///       eventTrigger:
///         triggerRegion: us-central1
///         eventType: google.cloud.pubsub.topic.v1.messagePublished
///         pubsubTopic: ${topic.id}
///         retryPolicy: RETRY_POLICY_RETRY
/// variables:
///   project: my-project-name
/// ```
/// ### Cloudfunctions2 Scheduler Auth
///
///
/// ```yaml
/// resources:
///   account:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: gcf-sa
///       displayName: Test Service Account
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: gcf-function
///       location: us-central1
///       description: a new function
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloHttp
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///       serviceConfig:
///         minInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
///         serviceAccountEmail: ${account.email}
///   invoker:
///     type: gcp:cloudfunctionsv2:FunctionIamMember
///     properties:
///       project: ${function.project}
///       location: ${function.location}
///       cloudFunction: ${function.name}
///       role: roles/cloudfunctions.invoker
///       member: serviceAccount:${account.email}
///   cloudRunInvoker:
///     type: gcp:cloudrun:IamMember
///     name: cloud_run_invoker
///     properties:
///       project: ${function.project}
///       location: ${function.location}
///       service: ${function.name}
///       role: roles/run.invoker
///       member: serviceAccount:${account.email}
///   invokeCloudFunction:
///     type: gcp:cloudscheduler:Job
///     name: invoke_cloud_function
///     properties:
///       name: invoke-gcf-function
///       description: Schedule the HTTPS trigger for cloud function
///       schedule: 0 0 * * *
///       project: ${function.project}
///       region: ${function.location}
///       httpTarget:
///         uri: ${function.serviceConfig.uri}
///         httpMethod: POST
///         oidcToken:
///           audience: ${function.serviceConfig.uri}/
///           serviceAccountEmail: ${account.email}
/// variables:
///   project: my-project-name
/// ```
/// ### Cloudfunctions2 Basic Gcs
///
///
/// ```yaml
/// resources:
///   source-bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: gcf-source-bucket
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${["source-bucket"].name}
///       source:
///         fn::FileAsset: function-source.zip
///   trigger-bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: gcf-trigger-bucket
///       location: us-central1
///       uniformBucketLevelAccess: true
///   # To use GCS CloudEvent triggers, the GCS service account requires the Pub/Sub Publisher(roles/pubsub.publisher) IAM role in the specified project.
///   # (See https://cloud.google.com/eventarc/docs/run/quickstart-storage#before-you-begin)
///   gcs-pubsub-publishing:
///     type: gcp:projects:IAMMember
///     properties:
///       project: my-project-name
///       role: roles/pubsub.publisher
///       member: serviceAccount:${gcsAccount.emailAddress}
///   account:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: gcf-sa
///       displayName: Test Service Account - used for both the cloud function and eventarc trigger in the test
///   # Permissions on the service account used by the function and Eventarc trigger
///   invoking:
///     type: gcp:projects:IAMMember
///     properties:
///       project: my-project-name
///       role: roles/run.invoker
///       member: serviceAccount:${account.email}
///     options:
///       dependsOn:
///         - ${["gcs-pubsub-publishing"]}
///   event-receiving:
///     type: gcp:projects:IAMMember
///     properties:
///       project: my-project-name
///       role: roles/eventarc.eventReceiver
///       member: serviceAccount:${account.email}
///     options:
///       dependsOn:
///         - ${invoking}
///   artifactregistry-reader:
///     type: gcp:projects:IAMMember
///     properties:
///       project: my-project-name
///       role: roles/artifactregistry.reader
///       member: serviceAccount:${account.email}
///     options:
///       dependsOn:
///         - ${["event-receiving"]}
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: gcf-function
///       location: us-central1
///       description: a new function
///       buildConfig:
///         runtime: nodejs12
///         entryPoint: entryPoint
///         environmentVariables:
///           BUILD_CONFIG_TEST: build_test
///         source:
///           storageSource:
///             bucket: ${["source-bucket"].name}
///             object: ${object.name}
///       serviceConfig:
///         maxInstanceCount: 3
///         minInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
///         environmentVariables:
///           SERVICE_CONFIG_TEST: config_test
///         ingressSettings: ALLOW_INTERNAL_ONLY
///         allTrafficOnLatestRevision: true
///         serviceAccountEmail: ${account.email}
///       eventTrigger:
///         eventType: google.cloud.storage.object.v1.finalized
///         retryPolicy: RETRY_POLICY_RETRY
///         serviceAccountEmail: ${account.email}
///         eventFilters:
///           - attribute: bucket
///             value: ${["trigger-bucket"].name}
///     options:
///       dependsOn:
///         - ${["event-receiving"]}
///         - ${["artifactregistry-reader"]}
/// variables:
///   gcsAccount:
///     fn::invoke:
///       function: gcp:storage:getProjectServiceAccount
///       arguments: {}
/// ```
/// ### Cloudfunctions2 Basic Auditlogs
///
///
/// ```yaml
/// resources:
///   # This example follows the examples shown in this Google Cloud Community blog post
///   # https://medium.com/google-cloud/applying-a-path-pattern-when-filtering-in-eventarc-f06b937b4c34
///   # and the docs:
///   # https://cloud.google.com/eventarc/docs/path-patterns
///   source-bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: gcf-source-bucket
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${["source-bucket"].name}
///       source:
///         fn::FileAsset: function-source.zip
///   account:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: gcf-sa
///       displayName: Test Service Account - used for both the cloud function and eventarc trigger in the test
///   # Note: The right way of listening for Cloud Storage events is to use a Cloud Storage trigger.
///   # Here we use Audit Logs to monitor the bucket so path patterns can be used in the example of
///   # google_cloudfunctions2_function below (Audit Log events have path pattern support)
///   audit-log-bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: gcf-auditlog-bucket
///       location: us-central1
///       uniformBucketLevelAccess: true
///   # Permissions on the service account used by the function and Eventarc trigger
///   invoking:
///     type: gcp:projects:IAMMember
///     properties:
///       project: my-project-name
///       role: roles/run.invoker
///       member: serviceAccount:${account.email}
///   event-receiving:
///     type: gcp:projects:IAMMember
///     properties:
///       project: my-project-name
///       role: roles/eventarc.eventReceiver
///       member: serviceAccount:${account.email}
///     options:
///       dependsOn:
///         - ${invoking}
///   artifactregistry-reader:
///     type: gcp:projects:IAMMember
///     properties:
///       project: my-project-name
///       role: roles/artifactregistry.reader
///       member: serviceAccount:${account.email}
///     options:
///       dependsOn:
///         - ${["event-receiving"]}
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: gcf-function
///       location: us-central1
///       description: a new function
///       buildConfig:
///         runtime: nodejs12
///         entryPoint: entryPoint
///         environmentVariables:
///           BUILD_CONFIG_TEST: build_test
///         source:
///           storageSource:
///             bucket: ${["source-bucket"].name}
///             object: ${object.name}
///       serviceConfig:
///         maxInstanceCount: 3
///         minInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
///         environmentVariables:
///           SERVICE_CONFIG_TEST: config_test
///         ingressSettings: ALLOW_INTERNAL_ONLY
///         allTrafficOnLatestRevision: true
///         serviceAccountEmail: ${account.email}
///       eventTrigger:
///         triggerRegion: us-central1
///         eventType: google.cloud.audit.log.v1.written
///         retryPolicy: RETRY_POLICY_RETRY
///         serviceAccountEmail: ${account.email}
///         eventFilters:
///           - attribute: serviceName
///             value: storage.googleapis.com
///           - attribute: methodName
///             value: storage.objects.create
///           - attribute: resourceName
///             value: /projects/_/buckets/${["audit-log-bucket"].name}/objects/*.txt
///             operator: match-path-pattern
///     options:
///       dependsOn:
///         - ${["event-receiving"]}
///         - ${["artifactregistry-reader"]}
/// ```
/// ### Cloudfunctions2 Basic Builder
///
///
/// ```yaml
/// resources:
///   account:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: gcf-sa
///       displayName: Test Service Account
///   logWriter:
///     type: gcp:projects:IAMMember
///     name: log_writer
///     properties:
///       project: ${account.project}
///       role: roles/logging.logWriter
///       member: serviceAccount:${account.email}
///   artifactRegistryWriter:
///     type: gcp:projects:IAMMember
///     name: artifact_registry_writer
///     properties:
///       project: ${account.project}
///       role: roles/artifactregistry.writer
///       member: serviceAccount:${account.email}
///   storageObjectAdmin:
///     type: gcp:projects:IAMMember
///     name: storage_object_admin
///     properties:
///       project: ${account.project}
///       role: roles/storage.objectAdmin
///       member: serviceAccount:${account.email}
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   # builder permissions need to stablize before it can pull the source zip
///   wait60s:
///     type: time:sleep
///     name: wait_60s
///     properties:
///       createDuration: 60s
///     options:
///       dependsOn:
///         - ${logWriter}
///         - ${artifactRegistryWriter}
///         - ${storageObjectAdmin}
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: function-v2
///       location: us-central1
///       description: a new function
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloHttp
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///         serviceAccount: ${account.id}
///       serviceConfig:
///         maxInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
///     options:
///       dependsOn:
///         - ${wait60s}
/// variables:
///   project: my-project-name
/// ```
/// ### Cloudfunctions2 Secret Env
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: function-secret
///       location: us-central1
///       description: a new function
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloHttp
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///       serviceConfig:
///         maxInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
///         secretEnvironmentVariables:
///           - key: TEST
///             projectId: ${project}
///             secret: ${secret.secretId}
///             version: latest
///     options:
///       dependsOn:
///         - ${secretSecretVersion}
///   secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret
///       replication:
///         userManaged:
///           replicas:
///             - location: us-central1
///   secretSecretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: secret
///     properties:
///       secret: ${secret.name}
///       secretData: secret
///       enabled: true
/// variables:
///   project: my-project-name
/// ```
/// ### Cloudfunctions2 Secret Volume
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: function-secret
///       location: us-central1
///       description: a new function
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloHttp
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///       serviceConfig:
///         maxInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
///         secretVolumes:
///           - mountPath: /etc/secrets
///             projectId: ${project}
///             secret: ${secret.secretId}
///     options:
///       dependsOn:
///         - ${secretSecretVersion}
///   secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret
///       replication:
///         userManaged:
///           replicas:
///             - location: us-central1
///   secretSecretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: secret
///     properties:
///       secret: ${secret.name}
///       secretData: secret
///       enabled: true
/// variables:
///   project: my-project-name
/// ```
/// ### Cloudfunctions2 Private Workerpool
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   pool:
///     type: gcp:cloudbuild:WorkerPool
///     properties:
///       name: workerpool
///       location: us-central1
///       workerConfig:
///         diskSizeGb: 100
///         machineType: e2-standard-8
///         noExternalIp: false
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: function-workerpool
///       location: us-central1
///       description: a new function
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloHttp
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///         workerPool: ${pool.id}
///       serviceConfig:
///         maxInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
/// variables:
///   project: my-project-name
/// ```
/// ### Cloudfunctions2 Cmek Docs
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   eaSa:
///     type: gcp:projects:ServiceIdentity
///     name: ea_sa
///     properties:
///       project: ${projectGetProject.projectId}
///       service: eventarc.googleapis.com
///   unencoded-ar-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       repositoryId: ar-repo
///       location: us-central1
///       format: DOCKER
///   binding:
///     type: gcp:artifactregistry:RepositoryIamBinding
///     properties:
///       location: ${["encoded-ar-repo"].location}
///       repository: ${["encoded-ar-repo"].name}
///       role: roles/artifactregistry.admin
///       members:
///         - serviceAccount:service-${projectGetProject.number}@gcf-admin-robot.iam.gserviceaccount.com
///   gcfCmekKeyuser:
///     type: gcp:kms:CryptoKeyIAMBinding
///     name: gcf_cmek_keyuser
///     properties:
///       cryptoKeyId: cmek-key
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       members:
///         - serviceAccount:service-${projectGetProject.number}@gcf-admin-robot.iam.gserviceaccount.com
///         - serviceAccount:service-${projectGetProject.number}@gcp-sa-artifactregistry.iam.gserviceaccount.com
///         - serviceAccount:service-${projectGetProject.number}@gs-project-accounts.iam.gserviceaccount.com
///         - serviceAccount:service-${projectGetProject.number}@serverless-robot-prod.iam.gserviceaccount.com
///         - ${eaSa.member}
///     options:
///       dependsOn:
///         - ${eaSa}
///   encoded-ar-repo:
///     type: gcp:artifactregistry:Repository
///     properties:
///       location: us-central1
///       repositoryId: cmek-repo
///       format: DOCKER
///       kmsKeyName: cmek-key
///     options:
///       dependsOn:
///         - ${gcfCmekKeyuser}
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: function-cmek
///       location: us-central1
///       description: CMEK function
///       kmsKeyName: cmek-key
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloHttp
///         dockerRepository: ${["encoded-ar-repo"].id}
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///       serviceConfig:
///         maxInstanceCount: 1
///         availableMemory: 256M
///         timeoutSeconds: 60
///     options:
///       dependsOn:
///         - ${gcfCmekKeyuser}
/// variables:
///   project: my-project-name
///   projectGetProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Cloudfunctions2 Abiu
///
///
/// ```yaml
/// resources:
///   account:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: gcf-sa
///       displayName: Test Service Account
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: functions2-topic
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: gcf-function
///       location: europe-west6
///       description: a new function
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloPubSub
///         environmentVariables:
///           BUILD_CONFIG_TEST: build_test
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///         automaticUpdatePolicy: {}
///       serviceConfig:
///         maxInstanceCount: 3
///         minInstanceCount: 1
///         availableMemory: 4Gi
///         timeoutSeconds: 60
///         maxInstanceRequestConcurrency: 80
///         availableCpu: '4'
///         environmentVariables:
///           SERVICE_CONFIG_TEST: config_test
///         ingressSettings: ALLOW_INTERNAL_ONLY
///         allTrafficOnLatestRevision: true
///         serviceAccountEmail: ${account.email}
///       eventTrigger:
///         triggerRegion: us-central1
///         eventType: google.cloud.pubsub.topic.v1.messagePublished
///         pubsubTopic: ${topic.id}
///         retryPolicy: RETRY_POLICY_RETRY
/// variables:
///   project: my-project-name
/// ```
/// ### Cloudfunctions2 Abiu On Deploy
///
///
/// ```yaml
/// resources:
///   account:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: gcf-sa
///       displayName: Test Service Account
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: functions2-topic
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-gcf-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   function:
///     type: gcp:cloudfunctionsv2:Function
///     properties:
///       name: gcf-function
///       location: europe-west6
///       description: a new function
///       buildConfig:
///         runtime: nodejs16
///         entryPoint: helloPubSub
///         environmentVariables:
///           BUILD_CONFIG_TEST: build_test
///         source:
///           storageSource:
///             bucket: ${bucket.name}
///             object: ${object.name}
///         onDeployUpdatePolicy: {}
///       serviceConfig:
///         maxInstanceCount: 3
///         minInstanceCount: 1
///         availableMemory: 4Gi
///         timeoutSeconds: 60
///         maxInstanceRequestConcurrency: 80
///         availableCpu: '4'
///         environmentVariables:
///           SERVICE_CONFIG_TEST: config_test
///         ingressSettings: ALLOW_INTERNAL_ONLY
///         allTrafficOnLatestRevision: true
///         serviceAccountEmail: ${account.email}
///       eventTrigger:
///         triggerRegion: us-central1
///         eventType: google.cloud.pubsub.topic.v1.messagePublished
///         pubsubTopic: ${topic.id}
///         retryPolicy: RETRY_POLICY_RETRY
/// variables:
///   project: my-project-name
/// ```
///
/// ## Import
///
/// function can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/functions/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, function can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudfunctionsv2/function:Function default projects/{{project}}/locations/{{location}}/functions/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudfunctionsv2/function:Function default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudfunctionsv2/function:Function default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionArgs {
        /// Describes the Build step of the function that builds a container
        /// from the given source.
        /// Structure is documented below.
        #[builder(into, default)]
        pub build_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudfunctionsv2::FunctionBuildConfig>,
        >,
        /// User-provided description of a function.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An Eventarc trigger managed by Google Cloud Functions that fires events in
        /// response to a condition in another service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub event_trigger: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudfunctionsv2::FunctionEventTrigger>,
        >,
        /// Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.
        /// It must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}.
        #[builder(into, default)]
        pub kms_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of key/value label pairs associated with this Cloud Function.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of this cloud function.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A user-defined name of the function. Function names must
        /// be unique globally and match pattern `projects/*/locations/*/functions/*`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Describes the Service being deployed.
        /// Structure is documented below.
        #[builder(into, default)]
        pub service_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudfunctionsv2::FunctionServiceConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct FunctionResult {
        /// Describes the Build step of the function that builds a container
        /// from the given source.
        /// Structure is documented below.
        pub build_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudfunctionsv2::FunctionBuildConfig>,
        >,
        /// User-provided description of a function.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The environment the function is hosted on.
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// An Eventarc trigger managed by Google Cloud Functions that fires events in
        /// response to a condition in another service.
        /// Structure is documented below.
        pub event_trigger: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudfunctionsv2::FunctionEventTrigger>,
        >,
        /// Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.
        /// It must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}.
        pub kms_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A set of key/value label pairs associated with this Cloud Function.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of this cloud function.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A user-defined name of the function. Function names must
        /// be unique globally and match pattern `projects/*/locations/*/functions/*`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Describes the Service being deployed.
        /// Structure is documented below.
        pub service_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudfunctionsv2::FunctionServiceConfig>,
        >,
        /// Describes the current state of the function.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The last update timestamp of a Cloud Function.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. The deployed url for the function.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FunctionArgs,
    ) -> FunctionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let build_config_binding = args.build_config.get_output(context);
        let description_binding = args.description.get_output(context);
        let event_trigger_binding = args.event_trigger.get_output(context);
        let kms_key_name_binding = args.kms_key_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_config_binding = args.service_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudfunctionsv2/function:Function".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildConfig".into(),
                    value: build_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventTrigger".into(),
                    value: event_trigger_binding.get_id(),
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
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceConfig".into(),
                    value: service_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FunctionResult {
            build_config: o.get_field("buildConfig"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            environment: o.get_field("environment"),
            event_trigger: o.get_field("eventTrigger"),
            kms_key_name: o.get_field("kmsKeyName"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            service_config: o.get_field("serviceConfig"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
            url: o.get_field("url"),
        }
    }
}
