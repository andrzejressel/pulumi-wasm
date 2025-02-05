/// Feature represents the settings and status of any Hub Feature.
///
///
/// To get more information about Feature, see:
///
/// * [API documentation](https://cloud.google.com/anthos/fleet-management/docs/reference/rest/v1/projects.locations.features)
/// * How-to Guides
///     * [Registering a Cluster](https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster#register_cluster)
///
/// ## Example Usage
///
/// ### Gkehub Feature Multi Cluster Ingress
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cluster = cluster::create(
///         "cluster",
///         ClusterArgs::builder()
///             .initial_node_count(1)
///             .location("us-central1-a")
///             .name("my-cluster")
///             .build_struct(),
///     );
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .location("global")
///             .name("multiclusteringress")
///             .spec(
///                 FeatureSpec::builder()
///                     .multiclusteringress(
///                         FeatureSpecMulticlusteringress::builder()
///                             .configMembership("${membership.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let membership = membership::create(
///         "membership",
///         MembershipArgs::builder()
///             .endpoint(
///                 MembershipEndpoint::builder()
///                     .gkeCluster(
///                         MembershipEndpointGkeCluster::builder()
///                             .resourceLink("//container.googleapis.com/${cluster.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .membership_id("my-membership")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkehub Feature Multi Cluster Service Discovery
///
///
/// ```yaml
/// resources:
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: multiclusterservicediscovery
///       location: global
///       labels:
///         foo: bar
/// ```
/// ### Gkehub Feature Anthos Service Mesh
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder().location("global").name("servicemesh").build_struct(),
///     );
/// }
/// ```
/// ### Enable Fleet Observability For Default Logs With Copy
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .location("global")
///             .name("fleetobservability")
///             .spec(
///                 FeatureSpec::builder()
///                     .fleetobservability(
///                         FeatureSpecFleetobservability::builder()
///                             .loggingConfig(
///                                 FeatureSpecFleetobservabilityLoggingConfig::builder()
///                                     .defaultConfig(
///                                         FeatureSpecFleetobservabilityLoggingConfigDefaultConfig::builder()
///                                             .mode("COPY")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Enable Fleet Observability For Scope Logs With Move
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .location("global")
///             .name("fleetobservability")
///             .spec(
///                 FeatureSpec::builder()
///                     .fleetobservability(
///                         FeatureSpecFleetobservability::builder()
///                             .loggingConfig(
///                                 FeatureSpecFleetobservabilityLoggingConfig::builder()
///                                     .fleetScopeLogsConfig(
///                                         FeatureSpecFleetobservabilityLoggingConfigFleetScopeLogsConfig::builder()
///                                             .mode("MOVE")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Enable Fleet Observability For Both Default And Scope Logs
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .location("global")
///             .name("fleetobservability")
///             .spec(
///                 FeatureSpec::builder()
///                     .fleetobservability(
///                         FeatureSpecFleetobservability::builder()
///                             .loggingConfig(
///                                 FeatureSpecFleetobservabilityLoggingConfig::builder()
///                                     .defaultConfig(
///                                         FeatureSpecFleetobservabilityLoggingConfigDefaultConfig::builder()
///                                             .mode("COPY")
///                                             .build_struct(),
///                                     )
///                                     .fleetScopeLogsConfig(
///                                         FeatureSpecFleetobservabilityLoggingConfigFleetScopeLogsConfig::builder()
///                                             .mode("MOVE")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Enable Fleet Default Member Config Service Mesh
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .fleet_default_member_config(
///                 FeatureFleetDefaultMemberConfig::builder()
///                     .mesh(
///                         FeatureFleetDefaultMemberConfigMesh::builder()
///                             .management("MANAGEMENT_AUTOMATIC")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("global")
///             .name("servicemesh")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Enable Fleet Default Member Config Configmanagement
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .fleet_default_member_config(
///                 FeatureFleetDefaultMemberConfig::builder()
///                     .configmanagement(
///                         FeatureFleetDefaultMemberConfigConfigmanagement::builder()
///                             .configSync(
///                                 FeatureFleetDefaultMemberConfigConfigmanagementConfigSync::builder()
///                                     .git(
///                                         FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncGit::builder()
///                                             .syncRepo("https://github.com/hashicorp/terraform")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("global")
///             .name("configmanagement")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Enable Fleet Default Member Config Policycontroller
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .fleet_default_member_config(
///                 FeatureFleetDefaultMemberConfig::builder()
///                     .policycontroller(
///                         FeatureFleetDefaultMemberConfigPolicycontroller::builder()
///                             .policyControllerHubConfig(
///                                 FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfig::builder()
///                                     .auditIntervalSeconds(30)
///                                     .exemptableNamespaces(vec!["foo",])
///                                     .installSpec("INSTALL_SPEC_ENABLED")
///                                     .policyContent(
///                                         FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContent::builder()
///                                             .bundles(
///                                                 vec![
///                                                     FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentBundle::builder()
///                                                     .bundle("policy-essentials-v2022")
///                                                     .exemptedNamespaces(vec!["foo", "bar",]).build_struct(),
///                                                 ],
///                                             )
///                                             .templateLibrary(
///                                                 FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentTemplateLibrary::builder()
///                                                     .installation("ALL")
///                                                     .build_struct(),
///                                             )
///                                             .build_struct(),
///                                     )
///                                     .referentialRulesEnabled(true)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("global")
///             .name("policycontroller")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Enable Fleet Default Member Config Policycontroller Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .fleet_default_member_config(
///                 FeatureFleetDefaultMemberConfig::builder()
///                     .policycontroller(
///                         FeatureFleetDefaultMemberConfigPolicycontroller::builder()
///                             .policyControllerHubConfig(
///                                 FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfig::builder()
///                                     .constraintViolationLimit(50)
///                                     .deploymentConfigs(
///                                         vec![
///                                             FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfig::builder()
///                                             .component("admission").podAffinity("ANTI_AFFINITY")
///                                             .replicaCount(2).build_struct(),
///                                             FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfig::builder()
///                                             .component("audit")
///                                             .containerResources(FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResources::builder()
///                                             .limits(FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResourcesLimits::builder()
///                                             .cpu("1.5").memory("1Gi").build_struct())
///                                             .requests(FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResourcesRequests::builder()
///                                             .cpu("150m").memory("500Mi").build_struct()).build_struct())
///                                             .podTolerations(vec![FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigPodToleration::builder()
///                                             .effect("NoSchedule").key("key1").operator("Equal")
///                                             .value("value1").build_struct(),]).build_struct(),
///                                         ],
///                                     )
///                                     .installSpec("INSTALL_SPEC_SUSPENDED")
///                                     .logDeniesEnabled(true)
///                                     .monitoring(
///                                         FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigMonitoring::builder()
///                                             .backends(vec!["PROMETHEUS",])
///                                             .build_struct(),
///                                     )
///                                     .mutationEnabled(true)
///                                     .policyContent(
///                                         FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContent::builder()
///                                             .bundles(
///                                                 vec![
///                                                     FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentBundle::builder()
///                                                     .bundle("pci-dss-v3.2.1").exemptedNamespaces(vec!["baz",
///                                                     "bar",]).build_struct(),
///                                                     FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentBundle::builder()
///                                                     .bundle("nist-sp-800-190").exemptedNamespaces(vec![])
///                                                     .build_struct(),
///                                                 ],
///                                             )
///                                             .templateLibrary(
///                                                 FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentTemplateLibrary::builder()
///                                                     .installation("ALL")
///                                                     .build_struct(),
///                                             )
///                                             .build_struct(),
///                                     )
///                                     .referentialRulesEnabled(true)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("global")
///             .name("policycontroller")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Enable Fleet Default Member Config Policycontroller Minimal
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .fleet_default_member_config(
///                 FeatureFleetDefaultMemberConfig::builder()
///                     .policycontroller(
///                         FeatureFleetDefaultMemberConfigPolicycontroller::builder()
///                             .policyControllerHubConfig(
///                                 FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfig::builder()
///                                     .constraintViolationLimit(50)
///                                     .deploymentConfigs(
///                                         vec![
///                                             FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfig::builder()
///                                             .component("admission").build_struct(),
///                                         ],
///                                     )
///                                     .installSpec("INSTALL_SPEC_ENABLED")
///                                     .logDeniesEnabled(true)
///                                     .monitoring(
///                                         FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigMonitoring::builder()
///                                             .build_struct(),
///                                     )
///                                     .mutationEnabled(true)
///                                     .policyContent(
///                                         FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContent::builder()
///                                             .build_struct(),
///                                     )
///                                     .referentialRulesEnabled(true)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("global")
///             .name("policycontroller")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkehub Feature Clusterupgrade
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder()
///             .location("global")
///             .name("clusterupgrade")
///             .spec(
///                 FeatureSpec::builder()
///                     .clusterupgrade(
///                         FeatureSpecClusterupgrade::builder()
///                             .postConditions(
///                                 FeatureSpecClusterupgradePostConditions::builder()
///                                     .soaking("60s")
///                                     .build_struct(),
///                             )
///                             .upstreamFleets(vec![])
///                             .build_struct(),
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
/// Feature can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/features/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Feature can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/feature:Feature default projects/{{project}}/locations/{{location}}/features/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/feature:Feature default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/feature:Feature default {{location}}/{{name}}
/// ```
///
pub mod feature {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FeatureArgs {
        /// Optional. Fleet Default Membership Configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub fleet_default_member_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfig>,
        >,
        /// GCP labels for this Feature.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The full, unique name of this Feature resource
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. Hub-wide Feature configuration. If this Feature does not support any Hub-wide configuration, this field may be unused.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spec: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::gkehub::FeatureSpec>,
        >,
    }
    #[allow(dead_code)]
    pub struct FeatureResult {
        /// Output only. When the Feature resource was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Output only. When the Feature resource was deleted.
        pub delete_time: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Fleet Default Membership Configuration.
        /// Structure is documented below.
        pub fleet_default_member_config: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfig>,
        >,
        /// GCP labels for this Feature.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The full, unique name of this Feature resource
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// State of the Feature resource itself.
        /// Structure is documented below.
        pub resource_states: pulumi_wasm_rust::Output<
            Vec<super::super::types::gkehub::FeatureResourceState>,
        >,
        /// Optional. Hub-wide Feature configuration. If this Feature does not support any Hub-wide configuration, this field may be unused.
        /// Structure is documented below.
        pub spec: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::FeatureSpec>,
        >,
        /// (Output)
        /// Output only. The "running state" of the Feature in this Hub.
        /// Structure is documented below.
        pub states: pulumi_wasm_rust::Output<
            Vec<super::super::types::gkehub::FeatureState>,
        >,
        /// (Output)
        /// The time this status and any related Feature-specific details were updated. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z"
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FeatureArgs,
    ) -> FeatureResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let fleet_default_member_config_binding = args
            .fleet_default_member_config
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let spec_binding = args.spec.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkehub/feature:Feature".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fleetDefaultMemberConfig".into(),
                    value: &fleet_default_member_config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
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
                    name: "spec".into(),
                    value: &spec_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FeatureResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deleteTime"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            fleet_default_member_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fleetDefaultMemberConfig"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            resource_states: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceStates"),
            ),
            spec: pulumi_wasm_rust::__private::into_domain(o.extract_field("spec")),
            states: pulumi_wasm_rust::__private::into_domain(o.extract_field("states")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
