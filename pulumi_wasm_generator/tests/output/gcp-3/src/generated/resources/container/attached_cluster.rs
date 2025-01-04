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
pub mod attached_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
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
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration related to the cluster RBAC settings.
        #[builder(into, default)]
        pub authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::container::AttachedClusterAuthorization>,
        >,
        /// Binary Authorization configuration.
        #[builder(into, default)]
        pub binary_authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::container::AttachedClusterBinaryAuthorization>,
        >,
        /// Policy to determine what flags to send on delete. Possible values: DELETE, DELETE_IGNORE_ERRORS
        #[builder(into, default)]
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A human readable description of this attached cluster. Cannot be longer than 255 UTF-8 encoded bytes.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Kubernetes distribution of the underlying attached cluster. Supported values:
        /// "eks", "aks", "generic". The generic distribution provides the ability to register
        /// or migrate any CNCF conformant cluster.
        #[builder(into)]
        pub distribution: pulumi_wasm_rust::Output<String>,
        /// Fleet configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub fleet: pulumi_wasm_rust::Output<
            super::super::types::container::AttachedClusterFleet,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Logging configuration.
        #[builder(into, default)]
        pub logging_config: pulumi_wasm_rust::Output<
            Option<super::super::types::container::AttachedClusterLoggingConfig>,
        >,
        /// Monitoring configuration.
        #[builder(into, default)]
        pub monitoring_config: pulumi_wasm_rust::Output<
            Option<super::super::types::container::AttachedClusterMonitoringConfig>,
        >,
        /// The name of this resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
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
        pub oidc_config: pulumi_wasm_rust::Output<
            super::super::types::container::AttachedClusterOidcConfig,
        >,
        /// The platform version for the cluster (e.g. `1.23.0-gke.1`).
        #[builder(into)]
        pub platform_version: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Support for proxy configuration.
        #[builder(into, default)]
        pub proxy_config: pulumi_wasm_rust::Output<
            Option<super::super::types::container::AttachedClusterProxyConfig>,
        >,
        /// Enable/Disable Security Posture API features for the cluster.
        #[builder(into, default)]
        pub security_posture_config: pulumi_wasm_rust::Output<
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
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration related to the cluster RBAC settings.
        pub authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::container::AttachedClusterAuthorization>,
        >,
        /// Binary Authorization configuration.
        pub binary_authorization: pulumi_wasm_rust::Output<
            super::super::types::container::AttachedClusterBinaryAuthorization,
        >,
        /// Output only. The region where this cluster runs.
        /// For EKS clusters, this is an AWS region. For AKS clusters,
        /// this is an Azure region.
        pub cluster_region: pulumi_wasm_rust::Output<String>,
        /// Output only. The time at which this cluster was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Policy to determine what flags to send on delete. Possible values: DELETE, DELETE_IGNORE_ERRORS
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A human readable description of this attached cluster. Cannot be longer than 255 UTF-8 encoded bytes.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Kubernetes distribution of the underlying attached cluster. Supported values:
        /// "eks", "aks", "generic". The generic distribution provides the ability to register
        /// or migrate any CNCF conformant cluster.
        pub distribution: pulumi_wasm_rust::Output<String>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A set of errors found in the cluster.
        /// Structure is documented below.
        pub errors: pulumi_wasm_rust::Output<
            Vec<super::super::types::container::AttachedClusterError>,
        >,
        /// Fleet configuration.
        /// Structure is documented below.
        pub fleet: pulumi_wasm_rust::Output<
            super::super::types::container::AttachedClusterFleet,
        >,
        /// The Kubernetes version of the cluster.
        pub kubernetes_version: pulumi_wasm_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Logging configuration.
        pub logging_config: pulumi_wasm_rust::Output<
            Option<super::super::types::container::AttachedClusterLoggingConfig>,
        >,
        /// Monitoring configuration.
        pub monitoring_config: pulumi_wasm_rust::Output<
            super::super::types::container::AttachedClusterMonitoringConfig,
        >,
        /// The name of this resource.
        pub name: pulumi_wasm_rust::Output<String>,
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
        pub oidc_config: pulumi_wasm_rust::Output<
            super::super::types::container::AttachedClusterOidcConfig,
        >,
        /// The platform version for the cluster (e.g. `1.23.0-gke.1`).
        pub platform_version: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Support for proxy configuration.
        pub proxy_config: pulumi_wasm_rust::Output<
            Option<super::super::types::container::AttachedClusterProxyConfig>,
        >,
        /// If set, there are currently changes in flight to the cluster.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// Enable/Disable Security Posture API features for the cluster.
        pub security_posture_config: pulumi_wasm_rust::Output<
            super::super::types::container::AttachedClusterSecurityPostureConfig,
        >,
        /// The current state of the cluster. Possible values:
        /// STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR,
        /// DEGRADED
        pub state: pulumi_wasm_rust::Output<String>,
        /// A globally unique identifier for the cluster.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// The time at which this cluster was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Workload Identity settings.
        /// Structure is documented below.
        pub workload_identity_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::container::AttachedClusterWorkloadIdentityConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AttachedClusterArgs) -> AttachedClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let authorization_binding = args.authorization.get_inner();
        let binary_authorization_binding = args.binary_authorization.get_inner();
        let deletion_policy_binding = args.deletion_policy.get_inner();
        let description_binding = args.description.get_inner();
        let distribution_binding = args.distribution.get_inner();
        let fleet_binding = args.fleet.get_inner();
        let location_binding = args.location.get_inner();
        let logging_config_binding = args.logging_config.get_inner();
        let monitoring_config_binding = args.monitoring_config.get_inner();
        let name_binding = args.name.get_inner();
        let oidc_config_binding = args.oidc_config.get_inner();
        let platform_version_binding = args.platform_version.get_inner();
        let project_binding = args.project.get_inner();
        let proxy_config_binding = args.proxy_config.get_inner();
        let security_posture_config_binding = args.security_posture_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:container/attachedCluster:AttachedCluster".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "authorization".into(),
                },
                register_interface::ResultField {
                    name: "binaryAuthorization".into(),
                },
                register_interface::ResultField {
                    name: "clusterRegion".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "distribution".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "errors".into(),
                },
                register_interface::ResultField {
                    name: "fleet".into(),
                },
                register_interface::ResultField {
                    name: "kubernetesVersion".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfig".into(),
                },
                register_interface::ResultField {
                    name: "monitoringConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "oidcConfig".into(),
                },
                register_interface::ResultField {
                    name: "platformVersion".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "proxyConfig".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "securityPostureConfig".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "workloadIdentityConfigs".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AttachedClusterResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            authorization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorization").unwrap(),
            ),
            binary_authorization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("binaryAuthorization").unwrap(),
            ),
            cluster_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterRegion").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionPolicy").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            distribution: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distribution").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            errors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errors").unwrap(),
            ),
            fleet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleet").unwrap(),
            ),
            kubernetes_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesVersion").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            logging_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfig").unwrap(),
            ),
            monitoring_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoringConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            oidc_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oidcConfig").unwrap(),
            ),
            platform_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformVersion").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            proxy_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyConfig").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            security_posture_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityPostureConfig").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            workload_identity_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workloadIdentityConfigs").unwrap(),
            ),
        }
    }
}
