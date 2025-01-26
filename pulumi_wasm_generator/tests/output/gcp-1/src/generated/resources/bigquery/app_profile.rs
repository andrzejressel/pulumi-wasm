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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod app_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppProfileArgs {
        /// The unique name of the app profile in the form `[_a-zA-Z0-9][-_.a-zA-Z0-9]*`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_profile_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies that this app profile is intended for read-only usage via the Data Boost feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub data_boost_isolation_read_only: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bigquery::AppProfileDataBoostIsolationReadOnly>,
        >,
        /// Long form description of the use case for this app profile.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If true, ignore safety checks when deleting/updating the app profile.
        #[builder(into, default)]
        pub ignore_warnings: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the instance to create the app profile within.
        #[builder(into, default)]
        pub instance: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all
        /// clusters are eligible.
        #[builder(into, default)]
        pub multi_cluster_routing_cluster_ids: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// If true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available
        /// in the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes
        /// consistency to improve availability.
        #[builder(into, default)]
        pub multi_cluster_routing_use_any: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Must be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row
        /// affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key
        /// will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves
        /// read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency
        /// is not guaranteed, as requests may still fail over between clusters in the event of errors or latency.
        #[builder(into, default)]
        pub row_affinity: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Use a single-cluster routing policy.
        /// Structure is documented below.
        #[builder(into, default)]
        pub single_cluster_routing: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bigquery::AppProfileSingleClusterRouting>,
        >,
        /// The standard options used for isolating this app profile's traffic from other use cases.
        /// Structure is documented below.
        #[builder(into, default)]
        pub standard_isolation: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bigquery::AppProfileStandardIsolation>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppProfileResult {
        /// The unique name of the app profile in the form `[_a-zA-Z0-9][-_.a-zA-Z0-9]*`.
        ///
        ///
        /// - - -
        pub app_profile_id: pulumi_wasm_rust::Output<String>,
        /// Specifies that this app profile is intended for read-only usage via the Data Boost feature.
        /// Structure is documented below.
        pub data_boost_isolation_read_only: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::AppProfileDataBoostIsolationReadOnly>,
        >,
        /// Long form description of the use case for this app profile.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// If true, ignore safety checks when deleting/updating the app profile.
        pub ignore_warnings: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the instance to create the app profile within.
        pub instance: pulumi_wasm_rust::Output<Option<String>>,
        /// The set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all
        /// clusters are eligible.
        pub multi_cluster_routing_cluster_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// If true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available
        /// in the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes
        /// consistency to improve availability.
        pub multi_cluster_routing_use_any: pulumi_wasm_rust::Output<Option<bool>>,
        /// The unique name of the requested app profile. Values are of the form `projects/<project>/instances/<instance>/appProfiles/<appProfileId>`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Must be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row
        /// affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key
        /// will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves
        /// read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency
        /// is not guaranteed, as requests may still fail over between clusters in the event of errors or latency.
        pub row_affinity: pulumi_wasm_rust::Output<Option<bool>>,
        /// Use a single-cluster routing policy.
        /// Structure is documented below.
        pub single_cluster_routing: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::AppProfileSingleClusterRouting>,
        >,
        /// The standard options used for isolating this app profile's traffic from other use cases.
        /// Structure is documented below.
        pub standard_isolation: pulumi_wasm_rust::Output<
            super::super::types::bigquery::AppProfileStandardIsolation,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AppProfileArgs,
    ) -> AppProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_profile_id_binding = args.app_profile_id.get_output(context).get_inner();
        let data_boost_isolation_read_only_binding = args
            .data_boost_isolation_read_only
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let ignore_warnings_binding = args
            .ignore_warnings
            .get_output(context)
            .get_inner();
        let instance_binding = args.instance.get_output(context).get_inner();
        let multi_cluster_routing_cluster_ids_binding = args
            .multi_cluster_routing_cluster_ids
            .get_output(context)
            .get_inner();
        let multi_cluster_routing_use_any_binding = args
            .multi_cluster_routing_use_any
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let row_affinity_binding = args.row_affinity.get_output(context).get_inner();
        let single_cluster_routing_binding = args
            .single_cluster_routing
            .get_output(context)
            .get_inner();
        let standard_isolation_binding = args
            .standard_isolation
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/appProfile:AppProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appProfileId".into(),
                    value: &app_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataBoostIsolationReadOnly".into(),
                    value: &data_boost_isolation_read_only_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreWarnings".into(),
                    value: &ignore_warnings_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "multiClusterRoutingClusterIds".into(),
                    value: &multi_cluster_routing_cluster_ids_binding,
                },
                register_interface::ObjectField {
                    name: "multiClusterRoutingUseAny".into(),
                    value: &multi_cluster_routing_use_any_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "rowAffinity".into(),
                    value: &row_affinity_binding,
                },
                register_interface::ObjectField {
                    name: "singleClusterRouting".into(),
                    value: &single_cluster_routing_binding,
                },
                register_interface::ObjectField {
                    name: "standardIsolation".into(),
                    value: &standard_isolation_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AppProfileResult {
            app_profile_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appProfileId"),
            ),
            data_boost_isolation_read_only: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataBoostIsolationReadOnly"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            ignore_warnings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ignoreWarnings"),
            ),
            instance: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            multi_cluster_routing_cluster_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multiClusterRoutingClusterIds"),
            ),
            multi_cluster_routing_use_any: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multiClusterRoutingUseAny"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            row_affinity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rowAffinity"),
            ),
            single_cluster_routing: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("singleClusterRouting"),
            ),
            standard_isolation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("standardIsolation"),
            ),
        }
    }
}
