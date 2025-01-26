/// Service acts as a top-level container that manages a set of configurations and revision templates which implement a network service. Service exists to provide a singular abstraction which can be access controlled, reasoned about, and which encapsulates software lifecycle decisions such as rollout policy and team resource ownership.
///
///
/// To get more information about Service, see:
///
/// * [API documentation](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/run/docs/)
///
/// ## Example Usage
///
/// ### Cloudrunv2 Service Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .deletion_protection(false)
///             .ingress("INGRESS_TRAFFIC_ALL")
///             .location("us-central1")
///             .name("cloudrun-service")
///             .template(
///                 ServiceTemplate::builder()
///                     .containers(
///                         vec![
///                             ServiceTemplateContainer::builder()
///                             .image("us-docker.pkg.dev/cloudrun/container/hello")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Service Limits
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrunv2:Service
///     properties:
///       name: cloudrun-service
///       location: us-central1
///       deletionProtection: false
///       ingress: INGRESS_TRAFFIC_ALL
///       template:
///         containers:
///           - image: us-docker.pkg.dev/cloudrun/container/hello
///             resources:
///               limits:
///                 cpu: '2'
///                 memory: 1024Mi
/// ```
/// ### Cloudrunv2 Service Sql
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrunv2:Service
///     properties:
///       name: cloudrun-service
///       location: us-central1
///       deletionProtection: false
///       ingress: INGRESS_TRAFFIC_ALL
///       template:
///         scaling:
///           maxInstanceCount: 2
///         volumes:
///           - name: cloudsql
///             cloudSqlInstance:
///               instances:
///                 - ${instance.connectionName}
///         containers:
///           - image: us-docker.pkg.dev/cloudrun/container/hello
///             envs:
///               - name: FOO
///                 value: bar
///               - name: SECRET_ENV_VAR
///                 valueSource:
///                   secretKeyRef:
///                     secret: ${secret.secretId}
///                     version: '1'
///             volumeMounts:
///               - name: cloudsql
///                 mountPath: /cloudsql
///       traffics:
///         - type: TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST
///           percent: 100
///     options:
///       dependsOn:
///         - ${["secret-version-data"]}
///   secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret-1
///       replication:
///         auto: {}
///   secret-version-data:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${secret.name}
///       secretData: secret-data
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${secret.id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:${project.number}-compute@developer.gserviceaccount.com
///     options:
///       dependsOn:
///         - ${secret}
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: cloudrun-sql
///       region: us-central1
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-f1-micro
///       deletionProtection: true
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Cloudrunv2 Service Vpcaccess
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connector = connector::create(
///         "connector",
///         ConnectorArgs::builder()
///             .machine_type("e2-standard-4")
///             .max_instances(3)
///             .min_instances(2)
///             .name("run-vpc")
///             .region("us-central1")
///             .subnet(ConnectorSubnet::builder().name("${customTest.name}").build_struct())
///             .build_struct(),
///     );
///     let customTest = subnetwork::create(
///         "customTest",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.2.0.0/28")
///             .name("run-subnetwork")
///             .network("${customTestNetwork.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let customTestNetwork = network::create(
///         "customTestNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("run-network")
///             .build_struct(),
///     );
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .deletion_protection(false)
///             .location("us-central1")
///             .name("cloudrun-service")
///             .template(
///                 ServiceTemplate::builder()
///                     .containers(
///                         vec![
///                             ServiceTemplateContainer::builder()
///                             .image("us-docker.pkg.dev/cloudrun/container/hello")
///                             .build_struct(),
///                         ],
///                     )
///                     .vpcAccess(
///                         ServiceTemplateVpcAccess::builder()
///                             .connector("${connector.id}")
///                             .egress("ALL_TRAFFIC")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Service Directvpc
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .deletion_protection(false)
///             .launch_stage("GA")
///             .location("us-central1")
///             .name("cloudrun-service")
///             .template(
///                 ServiceTemplate::builder()
///                     .containers(
///                         vec![
///                             ServiceTemplateContainer::builder()
///                             .image("us-docker.pkg.dev/cloudrun/container/hello")
///                             .build_struct(),
///                         ],
///                     )
///                     .vpcAccess(
///                         ServiceTemplateVpcAccess::builder()
///                             .networkInterfaces(
///                                 vec![
///                                     ServiceTemplateVpcAccessNetworkInterface::builder()
///                                     .network("default").subnetwork("default").tags(vec!["tag1",
///                                     "tag2", "tag3",]).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Service Gpu
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrunv2:Service
///     properties:
///       name: cloudrun-service
///       location: us-central1
///       deletionProtection: false
///       ingress: INGRESS_TRAFFIC_ALL
///       launchStage: BETA
///       template:
///         containers:
///           - image: us-docker.pkg.dev/cloudrun/container/hello
///             resources:
///               limits:
///                 cpu: '4'
///                 memory: 16Gi
///                 nvidia.com/gpu: '1'
///               startupCpuBoost: true
///         nodeSelector:
///           accelerator: nvidia-l4
///         scaling:
///           maxInstanceCount: 1
/// ```
/// ### Cloudrunv2 Service Probes
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .deletion_protection(false)
///             .location("us-central1")
///             .name("cloudrun-service")
///             .template(
///                 ServiceTemplate::builder()
///                     .containers(
///                         vec![
///                             ServiceTemplateContainer::builder()
///                             .image("us-docker.pkg.dev/cloudrun/container/hello")
///                             .livenessProbe(ServiceTemplateContainerLivenessProbe::builder()
///                             .httpGet(ServiceTemplateContainerLivenessProbeHttpGet::builder()
///                             .path("/").build_struct()).build_struct())
///                             .startupProbe(ServiceTemplateContainerStartupProbe::builder()
///                             .failureThreshold(1).initialDelaySeconds(0).periodSeconds(3)
///                             .tcpSocket(ServiceTemplateContainerStartupProbeTcpSocket::builder()
///                             .port(8080).build_struct()).timeoutSeconds(1).build_struct())
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Service Secret
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrunv2:Service
///     properties:
///       name: cloudrun-service
///       location: us-central1
///       deletionProtection: false
///       ingress: INGRESS_TRAFFIC_ALL
///       template:
///         volumes:
///           - name: a-volume
///             secret:
///               secret: ${secret.secretId}
///               defaultMode: 292
///               items:
///                 - version: '1'
///                   path: my-secret
///         containers:
///           - image: us-docker.pkg.dev/cloudrun/container/hello
///             volumeMounts:
///               - name: a-volume
///                 mountPath: /secrets
///     options:
///       dependsOn:
///         - ${["secret-version-data"]}
///   secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret-1
///       replication:
///         auto: {}
///   secret-version-data:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${secret.name}
///       secretData: secret-data
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${secret.id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:${project.number}-compute@developer.gserviceaccount.com
///     options:
///       dependsOn:
///         - ${secret}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Cloudrunv2 Service Multicontainer
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .deletion_protection(false)
///             .ingress("INGRESS_TRAFFIC_ALL")
///             .location("us-central1")
///             .name("cloudrun-service")
///             .template(
///                 ServiceTemplate::builder()
///                     .containers(
///                         vec![
///                             ServiceTemplateContainer::builder()
///                             .dependsOns(vec!["hello-2",])
///                             .image("us-docker.pkg.dev/cloudrun/container/hello")
///                             .name("hello-1")
///                             .ports(ServiceTemplateContainerPorts::builder()
///                             .containerPort(8080).build_struct())
///                             .volumeMounts(vec![ServiceTemplateContainerVolumeMount::builder()
///                             .mountPath("/mnt").name("empty-dir-volume").build_struct(),])
///                             .build_struct(), ServiceTemplateContainer::builder()
///                             .envs(vec![ServiceTemplateContainerEnv::builder()
///                             .name("PORT").value("8081").build_struct(),])
///                             .image("us-docker.pkg.dev/cloudrun/container/hello")
///                             .name("hello-2")
///                             .startupProbe(ServiceTemplateContainerStartupProbe::builder()
///                             .httpGet(ServiceTemplateContainerStartupProbeHttpGet::builder()
///                             .port(8081).build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .volumes(
///                         vec![
///                             ServiceTemplateVolume::builder()
///                             .emptyDir(ServiceTemplateVolumeEmptyDir::builder()
///                             .medium("MEMORY").sizeLimit("256Mi").build_struct())
///                             .name("empty-dir-volume").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Service Mount Gcs
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .deletion_protection(false)
///             .location("us-central1")
///             .name("cloudrun-service")
///             .template(
///                 ServiceTemplate::builder()
///                     .containers(
///                         vec![
///                             ServiceTemplateContainer::builder()
///                             .image("us-docker.pkg.dev/cloudrun/container/hello")
///                             .volumeMounts(vec![ServiceTemplateContainerVolumeMount::builder()
///                             .mountPath("/var/www").name("bucket").build_struct(),])
///                             .build_struct(),
///                         ],
///                     )
///                     .executionEnvironment("EXECUTION_ENVIRONMENT_GEN2")
///                     .volumes(
///                         vec![
///                             ServiceTemplateVolume::builder()
///                             .gcs(ServiceTemplateVolumeGcs::builder()
///                             .bucket("${defaultBucket.name}").readOnly(false)
///                             .build_struct()).name("bucket").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let defaultBucket = bucket::create(
///         "defaultBucket",
///         BucketArgs::builder().location("US").name("cloudrun-service").build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Service Mount Nfs
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .deletion_protection(false)
///             .ingress("INGRESS_TRAFFIC_ALL")
///             .location("us-central1")
///             .name("cloudrun-service")
///             .template(
///                 ServiceTemplate::builder()
///                     .containers(
///                         vec![
///                             ServiceTemplateContainer::builder()
///                             .image("us-docker.pkg.dev/cloudrun/container/hello:latest")
///                             .volumeMounts(vec![ServiceTemplateContainerVolumeMount::builder()
///                             .mountPath("/mnt/nfs/filestore").name("nfs")
///                             .build_struct(),]).build_struct(),
///                         ],
///                     )
///                     .executionEnvironment("EXECUTION_ENVIRONMENT_GEN2")
///                     .volumes(
///                         vec![
///                             ServiceTemplateVolume::builder().name("nfs")
///                             .nfs(ServiceTemplateVolumeNfs::builder().path("/share1")
///                             .readOnly(false)
///                             .server("${defaultInstance.networks[0].ipAddresses[0]}")
///                             .build_struct()).build_struct(),
///                         ],
///                     )
///                     .vpcAccess(
///                         ServiceTemplateVpcAccess::builder()
///                             .networkInterfaces(
///                                 vec![
///                                     ServiceTemplateVpcAccessNetworkInterface::builder()
///                                     .network("default").subnetwork("default").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let defaultInstance = instance::create(
///         "defaultInstance",
///         InstanceArgs::builder()
///             .file_shares(
///                 InstanceFileShares::builder()
///                     .capacityGb(1024)
///                     .name("share1")
///                     .build_struct(),
///             )
///             .location("us-central1-b")
///             .name("cloudrun-service")
///             .networks(
///                 vec![
///                     InstanceNetwork::builder().modes(vec!["MODE_IPV4",])
///                     .network("default").build_struct(),
///                 ],
///             )
///             .tier("BASIC_HDD")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Service Mesh
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrunv2:Service
///     properties:
///       name: cloudrun-service
///       deletionProtection: false
///       location: us-central1
///       launchStage: BETA
///       template:
///         containers:
///           - image: us-docker.pkg.dev/cloudrun/container/hello
///         serviceMesh:
///           mesh: ${mesh.id}
///     options:
///       dependsOn:
///         - ${waitForMesh}
///   waitForMesh:
///     type: time:sleep
///     name: wait_for_mesh
///     properties:
///       createDuration: 1m
///     options:
///       dependsOn:
///         - ${mesh}
///   mesh:
///     type: gcp:networkservices:Mesh
///     properties:
///       name: network-services-mesh
/// ```
/// ### Cloudrunv2 Service Invokeriam
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .deletion_protection(false)
///             .description(
///                 "The serving URL of this service will not perform any IAM check when invoked",
///             )
///             .ingress("INGRESS_TRAFFIC_ALL")
///             .invoker_iam_disabled(true)
///             .location("us-central1")
///             .name("cloudrun-service")
///             .template(
///                 ServiceTemplate::builder()
///                     .containers(
///                         vec![
///                             ServiceTemplateContainer::builder()
///                             .image("us-docker.pkg.dev/cloudrun/container/hello")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Service can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/services/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Service can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/service:Service default projects/{{project}}/locations/{{location}}/services/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/service:Service default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/service:Service default {{location}}/{{name}}
/// ```
///
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and
        /// should be preserved when modifying objects. Cloud Run API v2 does not support annotations with 'run.googleapis.com',
        /// 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new
        /// resources. All system annotations in v1 now have a corresponding field in v2 Service. This field follows Kubernetes
        /// annotations' namespacing, limits, and rules. **Note**: This field is non-authoritative, and will only manage the
        /// annotations present in your configuration. Please refer to the field 'effective_annotations' for all of the annotations
        /// present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Settings for the Binary Authorization feature.
        #[builder(into, default)]
        pub binary_authorization: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudrunv2::ServiceBinaryAuthorization>,
        >,
        /// Arbitrary identifier for the API client.
        #[builder(into, default)]
        pub client: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Arbitrary version identifier for the API client.
        #[builder(into, default)]
        pub client_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a
        /// string. The custom audiences are encoded in the token and used to authenticate requests. For more information, see
        /// https://cloud.google.com/run/docs/configuring/custom-audiences.
        #[builder(into, default)]
        pub custom_audiences: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Disables public resolution of the default URI of this service.
        #[builder(into, default)]
        pub default_uri_disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// User-provided description of the Service. This field currently has a 512-character limit.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Provides the ingress settings for this Service. On output, returns the currently observed ingress settings, or
        /// INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: ["INGRESS_TRAFFIC_ALL",
        /// "INGRESS_TRAFFIC_INTERNAL_ONLY", "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER"]
        #[builder(into, default)]
        pub ingress: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Disables IAM permission check for run.routes.invoke for callers of this service. This feature is available by invitation
        /// only. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check.
        #[builder(into, default)]
        pub invoker_iam_disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with
        /// Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment,
        /// state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or
        /// https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with
        /// 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they
        /// will be rejected. All system labels in v1 now have a corresponding field in v2 Service. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The launch stage as defined by [Google Cloud Platform Launch
        /// Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA. If no value is
        /// specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that
        /// stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as
        /// input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values:
        /// ["UNIMPLEMENTED", "PRELAUNCH", "EARLY_ACCESS", "ALPHA", "BETA", "GA", "DEPRECATED"]
        #[builder(into, default)]
        pub launch_stage: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The location of the cloud run service
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the Service.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Scaling settings that apply to the whole service
        #[builder(into, default)]
        pub scaling: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudrunv2::ServiceScaling>,
        >,
        /// The template used to create revisions for this Service.
        /// Structure is documented below.
        #[builder(into)]
        pub template: pulumi_wasm_rust::InputOrOutput<
            super::super::types::cloudrunv2::ServiceTemplate,
        >,
        /// Specifies how to distribute traffic over a collection of Revisions belonging to the Service. If traffic is empty or not
        /// provided, defaults to 100% traffic to the latest Ready Revision.
        #[builder(into, default)]
        pub traffics: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudrunv2::ServiceTraffic>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and
        /// should be preserved when modifying objects. Cloud Run API v2 does not support annotations with 'run.googleapis.com',
        /// 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new
        /// resources. All system annotations in v1 now have a corresponding field in v2 Service. This field follows Kubernetes
        /// annotations' namespacing, limits, and rules. **Note**: This field is non-authoritative, and will only manage the
        /// annotations present in your configuration. Please refer to the field 'effective_annotations' for all of the annotations
        /// present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Settings for the Binary Authorization feature.
        pub binary_authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudrunv2::ServiceBinaryAuthorization>,
        >,
        /// Arbitrary identifier for the API client.
        pub client: pulumi_wasm_rust::Output<Option<String>>,
        /// Arbitrary version identifier for the API client.
        pub client_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run.
        /// Structure is documented below.
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudrunv2::ServiceCondition>,
        >,
        /// The creation time.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Email address of the authenticated creator.
        pub creator: pulumi_wasm_rust::Output<String>,
        /// One or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a
        /// string. The custom audiences are encoded in the token and used to authenticate requests. For more information, see
        /// https://cloud.google.com/run/docs/configuring/custom-audiences.
        pub custom_audiences: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Disables public resolution of the default URI of this service.
        pub default_uri_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The deletion time.
        pub delete_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// User-provided description of the Service. This field currently has a 512-character limit.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// For a deleted resource, the time after which it will be permanently deleted.
        pub expire_time: pulumi_wasm_rust::Output<String>,
        /// A number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer.
        pub generation: pulumi_wasm_rust::Output<String>,
        /// Provides the ingress settings for this Service. On output, returns the currently observed ingress settings, or
        /// INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: ["INGRESS_TRAFFIC_ALL",
        /// "INGRESS_TRAFFIC_INTERNAL_ONLY", "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER"]
        pub ingress: pulumi_wasm_rust::Output<String>,
        /// Disables IAM permission check for run.routes.invoke for callers of this service. This feature is available by invitation
        /// only. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check.
        pub invoker_iam_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with
        /// Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment,
        /// state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or
        /// https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with
        /// 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they
        /// will be rejected. All system labels in v1 now have a corresponding field in v2 Service. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Email address of the last authenticated modifier.
        pub last_modifier: pulumi_wasm_rust::Output<String>,
        /// Name of the last created revision. See comments in reconciling for additional information on reconciliation process in Cloud Run.
        pub latest_created_revision: pulumi_wasm_rust::Output<String>,
        /// Name of the latest revision that is serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run.
        pub latest_ready_revision: pulumi_wasm_rust::Output<String>,
        /// The launch stage as defined by [Google Cloud Platform Launch
        /// Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA. If no value is
        /// specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that
        /// stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as
        /// input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values:
        /// ["UNIMPLEMENTED", "PRELAUNCH", "EARLY_ACCESS", "ALPHA", "BETA", "GA", "DEPRECATED"]
        pub launch_stage: pulumi_wasm_rust::Output<String>,
        /// The location of the cloud run service
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the Service.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The generation of this Service currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer.
        pub observed_generation: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Returns true if the Service is currently being acted upon by the system to bring it into the desired state.
        /// When a new Service is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Service to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration, latest_ready_revison, trafficStatuses, and uri will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Service, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.
        /// If reconciliation succeeded, the following fields will match: traffic and trafficStatuses, observedGeneration and generation, latestReadyRevision and latestCreatedRevision.
        /// If reconciliation failed, trafficStatuses, observedGeneration, and latestReadyRevision will have the state of the last serving revision, or empty for newly created Services. Additional information on the failure can be found in terminalCondition and conditions.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// Scaling settings that apply to the whole service
        pub scaling: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudrunv2::ServiceScaling>,
        >,
        /// The template used to create revisions for this Service.
        /// Structure is documented below.
        pub template: pulumi_wasm_rust::Output<
            super::super::types::cloudrunv2::ServiceTemplate,
        >,
        /// The Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run.
        /// Structure is documented below.
        pub terminal_conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudrunv2::ServiceTerminalCondition>,
        >,
        /// Detailed status information for corresponding traffic targets. See comments in reconciling for additional information on reconciliation process in Cloud Run.
        /// Structure is documented below.
        pub traffic_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudrunv2::ServiceTrafficStatus>,
        >,
        /// Specifies how to distribute traffic over a collection of Revisions belonging to the Service. If traffic is empty or not
        /// provided, defaults to 100% traffic to the latest Ready Revision.
        pub traffics: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudrunv2::ServiceTraffic>,
        >,
        /// Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// The last-modified time.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// (Output)
        /// Displays the target URI.
        pub uri: pulumi_wasm_rust::Output<String>,
        /// All URLs serving traffic for this Service.
        pub urls: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let binary_authorization_binding = args
            .binary_authorization
            .get_output(context)
            .get_inner();
        let client_binding = args.client.get_output(context).get_inner();
        let client_version_binding = args.client_version.get_output(context).get_inner();
        let custom_audiences_binding = args
            .custom_audiences
            .get_output(context)
            .get_inner();
        let default_uri_disabled_binding = args
            .default_uri_disabled
            .get_output(context)
            .get_inner();
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let ingress_binding = args.ingress.get_output(context).get_inner();
        let invoker_iam_disabled_binding = args
            .invoker_iam_disabled
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let launch_stage_binding = args.launch_stage.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let scaling_binding = args.scaling.get_output(context).get_inner();
        let template_binding = args.template.get_output(context).get_inner();
        let traffics_binding = args.traffics.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudrunv2/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "binaryAuthorization".into(),
                    value: &binary_authorization_binding,
                },
                register_interface::ObjectField {
                    name: "client".into(),
                    value: &client_binding,
                },
                register_interface::ObjectField {
                    name: "clientVersion".into(),
                    value: &client_version_binding,
                },
                register_interface::ObjectField {
                    name: "customAudiences".into(),
                    value: &custom_audiences_binding,
                },
                register_interface::ObjectField {
                    name: "defaultUriDisabled".into(),
                    value: &default_uri_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ingress".into(),
                    value: &ingress_binding,
                },
                register_interface::ObjectField {
                    name: "invokerIamDisabled".into(),
                    value: &invoker_iam_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "launchStage".into(),
                    value: &launch_stage_binding,
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
                register_interface::ObjectField {
                    name: "scaling".into(),
                    value: &scaling_binding,
                },
                register_interface::ObjectField {
                    name: "template".into(),
                    value: &template_binding,
                },
                register_interface::ObjectField {
                    name: "traffics".into(),
                    value: &traffics_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            binary_authorization: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("binaryAuthorization"),
            ),
            client: pulumi_wasm_rust::__private::into_domain(o.extract_field("client")),
            client_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientVersion"),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("conditions"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            creator: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creator"),
            ),
            custom_audiences: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customAudiences"),
            ),
            default_uri_disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultUriDisabled"),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deleteTime"),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expireTime"),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("generation"),
            ),
            ingress: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ingress"),
            ),
            invoker_iam_disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("invokerIamDisabled"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            last_modifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModifier"),
            ),
            latest_created_revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestCreatedRevision"),
            ),
            latest_ready_revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestReadyRevision"),
            ),
            launch_stage: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("launchStage"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            observed_generation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("observedGeneration"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            scaling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scaling"),
            ),
            template: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("template"),
            ),
            terminal_conditions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("terminalConditions"),
            ),
            traffic_statuses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trafficStatuses"),
            ),
            traffics: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("traffics"),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(o.extract_field("uri")),
            urls: pulumi_wasm_rust::__private::into_domain(o.extract_field("urls")),
        }
    }
}
