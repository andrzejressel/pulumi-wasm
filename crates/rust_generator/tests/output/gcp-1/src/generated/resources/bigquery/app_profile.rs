/// App profile is a configuration object describing how Cloud Bigtable should treat traffic from a particular end user application.
///
///
/// To get more information about AppProfile, see:
///
/// * [API documentation](https://cloud.google.com/bigtable/docs/reference/admin/rest/v2/projects.instances.appProfiles)
///
/// ## Example Usage
///
/// ### Bigtable App Profile Anycluster
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ap = app_profile::create(
///         "ap",
///         AppProfileArgs::builder()
///             .app_profile_id("bt-profile")
///             .ignore_warnings(true)
///             .instance("${instance.name}")
///             .multi_cluster_routing_use_any(true)
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .clusters(
///                 vec![
///                     InstanceCluster::builder().clusterId("cluster-1").numNodes(3)
///                     .storageType("HDD").zone("us-central1-a").build_struct(),
///                     InstanceCluster::builder().clusterId("cluster-2").numNodes(3)
///                     .storageType("HDD").zone("us-central1-b").build_struct(),
///                     InstanceCluster::builder().clusterId("cluster-3").numNodes(3)
///                     .storageType("HDD").zone("us-central1-c").build_struct(),
///                 ],
///             )
///             .deletion_protection(true)
///             .name("bt-instance")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigtable App Profile Singlecluster
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ap = app_profile::create(
///         "ap",
///         AppProfileArgs::builder()
///             .app_profile_id("bt-profile")
///             .ignore_warnings(true)
///             .instance("${instance.name}")
///             .single_cluster_routing(
///                 AppProfileSingleClusterRouting::builder()
///                     .allowTransactionalWrites(true)
///                     .clusterId("cluster-1")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .clusters(
///                 vec![
///                     InstanceCluster::builder().clusterId("cluster-1").numNodes(3)
///                     .storageType("HDD").zone("us-central1-b").build_struct(),
///                 ],
///             )
///             .deletion_protection(true)
///             .name("bt-instance")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigtable App Profile Multicluster
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ap = app_profile::create(
///         "ap",
///         AppProfileArgs::builder()
///             .app_profile_id("bt-profile")
///             .ignore_warnings(true)
///             .instance("${instance.name}")
///             .multi_cluster_routing_cluster_ids(vec!["cluster-1", "cluster-2",])
///             .multi_cluster_routing_use_any(true)
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .clusters(
///                 vec![
///                     InstanceCluster::builder().clusterId("cluster-1").numNodes(3)
///                     .storageType("HDD").zone("us-central1-a").build_struct(),
///                     InstanceCluster::builder().clusterId("cluster-2").numNodes(3)
///                     .storageType("HDD").zone("us-central1-b").build_struct(),
///                     InstanceCluster::builder().clusterId("cluster-3").numNodes(3)
///                     .storageType("HDD").zone("us-central1-c").build_struct(),
///                 ],
///             )
///             .deletion_protection(true)
///             .name("bt-instance")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigtable App Profile Priority
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ap = app_profile::create(
///         "ap",
///         AppProfileArgs::builder()
///             .app_profile_id("bt-profile")
///             .ignore_warnings(true)
///             .instance("${instance.name}")
///             .single_cluster_routing(
///                 AppProfileSingleClusterRouting::builder()
///                     .allowTransactionalWrites(true)
///                     .clusterId("cluster-1")
///                     .build_struct(),
///             )
///             .standard_isolation(
///                 AppProfileStandardIsolation::builder()
///                     .priority("PRIORITY_LOW")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .clusters(
///                 vec![
///                     InstanceCluster::builder().clusterId("cluster-1").numNodes(3)
///                     .storageType("HDD").zone("us-central1-b").build_struct(),
///                 ],
///             )
///             .deletion_protection(true)
///             .name("bt-instance")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AppProfile can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/instances/{{instance}}/appProfiles/{{app_profile_id}}`
///
/// * `{{project}}/{{instance}}/{{app_profile_id}}`
///
/// * `{{instance}}/{{app_profile_id}}`
///
/// When using the `pulumi import` command, AppProfile can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/appProfile:AppProfile default projects/{{project}}/instances/{{instance}}/appProfiles/{{app_profile_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/appProfile:AppProfile default {{project}}/{{instance}}/{{app_profile_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/appProfile:AppProfile default {{instance}}/{{app_profile_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppProfileArgs {
        /// The unique name of the app profile in the form `[_a-zA-Z0-9][-_.a-zA-Z0-9]*`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies that this app profile is intended for read-only usage via the Data Boost feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub data_boost_isolation_read_only: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::AppProfileDataBoostIsolationReadOnly>,
        >,
        /// Long form description of the use case for this app profile.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, ignore safety checks when deleting/updating the app profile.
        #[builder(into, default)]
        pub ignore_warnings: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the instance to create the app profile within.
        #[builder(into, default)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all
        /// clusters are eligible.
        #[builder(into, default)]
        pub multi_cluster_routing_cluster_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// If true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available
        /// in the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes
        /// consistency to improve availability.
        #[builder(into, default)]
        pub multi_cluster_routing_use_any: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Must be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row
        /// affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key
        /// will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves
        /// read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency
        /// is not guaranteed, as requests may still fail over between clusters in the event of errors or latency.
        #[builder(into, default)]
        pub row_affinity: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Use a single-cluster routing policy.
        /// Structure is documented below.
        #[builder(into, default)]
        pub single_cluster_routing: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::AppProfileSingleClusterRouting>,
        >,
        /// The standard options used for isolating this app profile's traffic from other use cases.
        /// Structure is documented below.
        #[builder(into, default)]
        pub standard_isolation: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::AppProfileStandardIsolation>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppProfileResult {
        /// The unique name of the app profile in the form `[_a-zA-Z0-9][-_.a-zA-Z0-9]*`.
        ///
        ///
        /// - - -
        pub app_profile_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies that this app profile is intended for read-only usage via the Data Boost feature.
        /// Structure is documented below.
        pub data_boost_isolation_read_only: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::AppProfileDataBoostIsolationReadOnly>,
        >,
        /// Long form description of the use case for this app profile.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If true, ignore safety checks when deleting/updating the app profile.
        pub ignore_warnings: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the instance to create the app profile within.
        pub instance: pulumi_gestalt_rust::Output<Option<String>>,
        /// The set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all
        /// clusters are eligible.
        pub multi_cluster_routing_cluster_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// If true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available
        /// in the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes
        /// consistency to improve availability.
        pub multi_cluster_routing_use_any: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The unique name of the requested app profile. Values are of the form `projects/<project>/instances/<instance>/appProfiles/<appProfileId>`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Must be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row
        /// affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key
        /// will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves
        /// read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency
        /// is not guaranteed, as requests may still fail over between clusters in the event of errors or latency.
        pub row_affinity: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Use a single-cluster routing policy.
        /// Structure is documented below.
        pub single_cluster_routing: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::AppProfileSingleClusterRouting>,
        >,
        /// The standard options used for isolating this app profile's traffic from other use cases.
        /// Structure is documented below.
        pub standard_isolation: pulumi_gestalt_rust::Output<
            super::super::types::bigquery::AppProfileStandardIsolation,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppProfileArgs,
    ) -> AppProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_profile_id_binding = args.app_profile_id.get_output(context);
        let data_boost_isolation_read_only_binding = args
            .data_boost_isolation_read_only
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let ignore_warnings_binding = args.ignore_warnings.get_output(context);
        let instance_binding = args.instance.get_output(context);
        let multi_cluster_routing_cluster_ids_binding = args
            .multi_cluster_routing_cluster_ids
            .get_output(context);
        let multi_cluster_routing_use_any_binding = args
            .multi_cluster_routing_use_any
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let row_affinity_binding = args.row_affinity.get_output(context);
        let single_cluster_routing_binding = args
            .single_cluster_routing
            .get_output(context);
        let standard_isolation_binding = args.standard_isolation.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigquery/appProfile:AppProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appProfileId".into(),
                    value: app_profile_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataBoostIsolationReadOnly".into(),
                    value: data_boost_isolation_read_only_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreWarnings".into(),
                    value: ignore_warnings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiClusterRoutingClusterIds".into(),
                    value: multi_cluster_routing_cluster_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiClusterRoutingUseAny".into(),
                    value: multi_cluster_routing_use_any_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rowAffinity".into(),
                    value: row_affinity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "singleClusterRouting".into(),
                    value: single_cluster_routing_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "standardIsolation".into(),
                    value: standard_isolation_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppProfileResult {
            app_profile_id: o.get_field("appProfileId"),
            data_boost_isolation_read_only: o.get_field("dataBoostIsolationReadOnly"),
            description: o.get_field("description"),
            ignore_warnings: o.get_field("ignoreWarnings"),
            instance: o.get_field("instance"),
            multi_cluster_routing_cluster_ids: o
                .get_field("multiClusterRoutingClusterIds"),
            multi_cluster_routing_use_any: o.get_field("multiClusterRoutingUseAny"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            row_affinity: o.get_field("rowAffinity"),
            single_cluster_routing: o.get_field("singleClusterRouting"),
            standard_isolation: o.get_field("standardIsolation"),
        }
    }
}
