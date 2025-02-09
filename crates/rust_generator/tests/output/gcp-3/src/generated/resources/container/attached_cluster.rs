/// An Anthos cluster running on customer owned infrastructure.
///
///
/// To get more information about Cluster, see:
///
/// * [API documentation](https://cloud.google.com/anthos/clusters/docs/multi-cloud/reference/rest)
/// * How-to Guides
///     * [API reference](https://cloud.google.com/anthos/clusters/docs/multi-cloud/reference/rest/v1/projects.locations.attachedClusters)
///     * [Multicloud overview](https://cloud.google.com/anthos/clusters/docs/multi-cloud)
///
/// ## Example Usage
///
/// ### Container Attached Cluster Basic
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AttachedCluster
///     properties:
///       name: basic
///       location: us-west1
///       project: ${project.projectId}
///       description: Test cluster
///       distribution: aks
///       oidcConfig:
///         issuerUrl: https://oidc.issuer.url
///       platformVersion: ${versions.validVersions[0]}
///       fleet:
///         project: projects/${project.number}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
///   versions:
///     fn::invoke:
///       function: gcp:container:getAttachedVersions
///       arguments:
///         location: us-west1
///         project: ${project.projectId}
/// ```
/// ### Container Attached Cluster Full
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AttachedCluster
///     properties:
///       name: basic
///       project: ${project.projectId}
///       location: us-west1
///       description: Test cluster
///       distribution: aks
///       annotations:
///         label-one: value-one
///       authorization:
///         adminUsers:
///           - user1@example.com
///           - user2@example.com
///         adminGroups:
///           - group1@example.com
///           - group2@example.com
///       oidcConfig:
///         issuerUrl: https://oidc.issuer.url
///         jwks:
///           fn::invoke:
///             function: std:base64encode
///             arguments:
///               input: '{"keys":[{"use":"sig","kty":"RSA","kid":"testid","alg":"RS256","n":"somedata","e":"AQAB"}]}'
///             return: result
///       platformVersion: ${versions.validVersions[0]}
///       fleet:
///         project: projects/${project.number}
///       loggingConfig:
///         componentConfig:
///           enableComponents:
///             - SYSTEM_COMPONENTS
///             - WORKLOADS
///       monitoringConfig:
///         managedPrometheusConfig:
///           enabled: true
///       binaryAuthorization:
///         evaluationMode: PROJECT_SINGLETON_POLICY_ENFORCE
///       proxyConfig:
///         kubernetesSecret:
///           name: proxy-config
///           namespace: default
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
///   versions:
///     fn::invoke:
///       function: gcp:container:getAttachedVersions
///       arguments:
///         location: us-west1
///         project: ${project.projectId}
/// ```
/// ### Container Attached Cluster Ignore Errors
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AttachedCluster
///     properties:
///       name: basic
///       location: us-west1
///       project: ${project.projectId}
///       description: Test cluster
///       distribution: aks
///       oidcConfig:
///         issuerUrl: https://oidc.issuer.url
///       platformVersion: ${versions.validVersions[0]}
///       fleet:
///         project: projects/${project.number}
///       deletionPolicy: DELETE_IGNORE_ERRORS
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
///   versions:
///     fn::invoke:
///       function: gcp:container:getAttachedVersions
///       arguments:
///         location: us-west1
///         project: ${project.projectId}
/// ```
///
/// ## Import
///
/// Cluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/attachedClusters/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/attachedCluster:AttachedCluster default projects/{{project}}/locations/{{location}}/attachedClusters/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/attachedCluster:AttachedCluster default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/attachedCluster:AttachedCluster default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod attached_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachedClusterArgs {
        /// Optional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of
        /// all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration related to the cluster RBAC settings.
        #[builder(into, default)]
        pub authorization: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AttachedClusterAuthorization>,
        >,
        /// Binary Authorization configuration.
        #[builder(into, default)]
        pub binary_authorization: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AttachedClusterBinaryAuthorization>,
        >,
        /// Policy to determine what flags to send on delete. Possible values: DELETE, DELETE_IGNORE_ERRORS
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A human readable description of this attached cluster. Cannot be longer than 255 UTF-8 encoded bytes.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Kubernetes distribution of the underlying attached cluster. Supported values:
        /// "eks", "aks", "generic". The generic distribution provides the ability to register
        /// or migrate any CNCF conformant cluster.
        #[builder(into)]
        pub distribution: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Fleet configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub fleet: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AttachedClusterFleet,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Logging configuration.
        #[builder(into, default)]
        pub logging_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AttachedClusterLoggingConfig>,
        >,
        /// Monitoring configuration.
        #[builder(into, default)]
        pub monitoring_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AttachedClusterMonitoringConfig>,
        >,
        /// The name of this resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// OIDC discovery information of the target cluster.
        /// Kubernetes Service Account (KSA) tokens are JWT tokens signed by the cluster
        /// API server. This fields indicates how GCP services
        /// validate KSA tokens in order to allow system workloads (such as GKE Connect
        /// and telemetry agents) to authenticate back to GCP.
        /// Both clusters with public and private issuer URLs are supported.
        /// Clusters with public issuers only need to specify the `issuer_url` field
        /// while clusters with private issuers need to provide both
        /// `issuer_url` and `jwks`.
        /// Structure is documented below.
        #[builder(into)]
        pub oidc_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AttachedClusterOidcConfig,
        >,
        /// The platform version for the cluster (e.g. `1.23.0-gke.1`).
        #[builder(into)]
        pub platform_version: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Support for proxy configuration.
        #[builder(into, default)]
        pub proxy_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AttachedClusterProxyConfig>,
        >,
        /// Enable/Disable Security Posture API features for the cluster.
        #[builder(into, default)]
        pub security_posture_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AttachedClusterSecurityPostureConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct AttachedClusterResult {
        /// Optional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of
        /// all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration related to the cluster RBAC settings.
        pub authorization: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::AttachedClusterAuthorization>,
        >,
        /// Binary Authorization configuration.
        pub binary_authorization: pulumi_gestalt_rust::Output<
            super::super::types::container::AttachedClusterBinaryAuthorization,
        >,
        /// Output only. The region where this cluster runs.
        /// For EKS clusters, this is an AWS region. For AKS clusters,
        /// this is an Azure region.
        pub cluster_region: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time at which this cluster was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Policy to determine what flags to send on delete. Possible values: DELETE, DELETE_IGNORE_ERRORS
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// A human readable description of this attached cluster. Cannot be longer than 255 UTF-8 encoded bytes.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Kubernetes distribution of the underlying attached cluster. Supported values:
        /// "eks", "aks", "generic". The generic distribution provides the ability to register
        /// or migrate any CNCF conformant cluster.
        pub distribution: pulumi_gestalt_rust::Output<String>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A set of errors found in the cluster.
        /// Structure is documented below.
        pub errors: pulumi_gestalt_rust::Output<
            Vec<super::super::types::container::AttachedClusterError>,
        >,
        /// Fleet configuration.
        /// Structure is documented below.
        pub fleet: pulumi_gestalt_rust::Output<
            super::super::types::container::AttachedClusterFleet,
        >,
        /// The Kubernetes version of the cluster.
        pub kubernetes_version: pulumi_gestalt_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Logging configuration.
        pub logging_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::AttachedClusterLoggingConfig>,
        >,
        /// Monitoring configuration.
        pub monitoring_config: pulumi_gestalt_rust::Output<
            super::super::types::container::AttachedClusterMonitoringConfig,
        >,
        /// The name of this resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// OIDC discovery information of the target cluster.
        /// Kubernetes Service Account (KSA) tokens are JWT tokens signed by the cluster
        /// API server. This fields indicates how GCP services
        /// validate KSA tokens in order to allow system workloads (such as GKE Connect
        /// and telemetry agents) to authenticate back to GCP.
        /// Both clusters with public and private issuer URLs are supported.
        /// Clusters with public issuers only need to specify the `issuer_url` field
        /// while clusters with private issuers need to provide both
        /// `issuer_url` and `jwks`.
        /// Structure is documented below.
        pub oidc_config: pulumi_gestalt_rust::Output<
            super::super::types::container::AttachedClusterOidcConfig,
        >,
        /// The platform version for the cluster (e.g. `1.23.0-gke.1`).
        pub platform_version: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Support for proxy configuration.
        pub proxy_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::AttachedClusterProxyConfig>,
        >,
        /// If set, there are currently changes in flight to the cluster.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Enable/Disable Security Posture API features for the cluster.
        pub security_posture_config: pulumi_gestalt_rust::Output<
            super::super::types::container::AttachedClusterSecurityPostureConfig,
        >,
        /// The current state of the cluster. Possible values:
        /// STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR,
        /// DEGRADED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A globally unique identifier for the cluster.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time at which this cluster was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Workload Identity settings.
        /// Structure is documented below.
        pub workload_identity_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::container::AttachedClusterWorkloadIdentityConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AttachedClusterArgs,
    ) -> AttachedClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let authorization_binding_1 = args.authorization.get_output(context);
        let authorization_binding = authorization_binding_1.get_inner();
        let binary_authorization_binding_1 = args
            .binary_authorization
            .get_output(context);
        let binary_authorization_binding = binary_authorization_binding_1.get_inner();
        let deletion_policy_binding_1 = args.deletion_policy.get_output(context);
        let deletion_policy_binding = deletion_policy_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let distribution_binding_1 = args.distribution.get_output(context);
        let distribution_binding = distribution_binding_1.get_inner();
        let fleet_binding_1 = args.fleet.get_output(context);
        let fleet_binding = fleet_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let logging_config_binding_1 = args.logging_config.get_output(context);
        let logging_config_binding = logging_config_binding_1.get_inner();
        let monitoring_config_binding_1 = args.monitoring_config.get_output(context);
        let monitoring_config_binding = monitoring_config_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let oidc_config_binding_1 = args.oidc_config.get_output(context);
        let oidc_config_binding = oidc_config_binding_1.get_inner();
        let platform_version_binding_1 = args.platform_version.get_output(context);
        let platform_version_binding = platform_version_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let proxy_config_binding_1 = args.proxy_config.get_output(context);
        let proxy_config_binding = proxy_config_binding_1.get_inner();
        let security_posture_config_binding_1 = args
            .security_posture_config
            .get_output(context);
        let security_posture_config_binding = security_posture_config_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:container/attachedCluster:AttachedCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "authorization".into(),
                    value: &authorization_binding,
                },
                register_interface::ObjectField {
                    name: "binaryAuthorization".into(),
                    value: &binary_authorization_binding,
                },
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "distribution".into(),
                    value: &distribution_binding,
                },
                register_interface::ObjectField {
                    name: "fleet".into(),
                    value: &fleet_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfig".into(),
                    value: &logging_config_binding,
                },
                register_interface::ObjectField {
                    name: "monitoringConfig".into(),
                    value: &monitoring_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "oidcConfig".into(),
                    value: &oidc_config_binding,
                },
                register_interface::ObjectField {
                    name: "platformVersion".into(),
                    value: &platform_version_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "proxyConfig".into(),
                    value: &proxy_config_binding,
                },
                register_interface::ObjectField {
                    name: "securityPostureConfig".into(),
                    value: &security_posture_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AttachedClusterResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            authorization: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorization"),
            ),
            binary_authorization: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("binaryAuthorization"),
            ),
            cluster_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterRegion"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            deletion_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionPolicy"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            distribution: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("distribution"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            errors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("errors"),
            ),
            fleet: pulumi_gestalt_rust::__private::into_domain(o.extract_field("fleet")),
            kubernetes_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubernetesVersion"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            logging_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingConfig"),
            ),
            monitoring_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringConfig"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            oidc_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("oidcConfig"),
            ),
            platform_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformVersion"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            proxy_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("proxyConfig"),
            ),
            reconciling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            security_posture_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityPostureConfig"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            workload_identity_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadIdentityConfigs"),
            ),
        }
    }
}
