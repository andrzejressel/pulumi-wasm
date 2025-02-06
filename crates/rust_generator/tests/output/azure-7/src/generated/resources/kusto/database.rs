/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cluster = cluster::create(
///         "cluster",
///         ClusterArgs::builder()
///             .location("${example.location}")
///             .name("kustocluster")
///             .resource_group_name("${example.name}")
///             .sku(
///                 ClusterSku::builder().capacity(2).name("Standard_D13_v2").build_struct(),
///             )
///             .build_struct(),
///     );
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .cluster_name("${cluster.name}")
///             .hot_cache_period("P7D")
///             .location("${example.location}")
///             .name("my-kusto-database")
///             .resource_group_name("${example.name}")
///             .soft_delete_period("P31D")
///             .build_struct(),
///     );
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("my-kusto-rg")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Kusto Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/database:Database example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/databases/database1
/// ```
///
pub mod database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// Specifies the name of the Kusto Cluster this database will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The time the data that should be kept in cache for fast queries as ISO 8601 timespan. Default is unlimited. For more information see: [ISO 8601 Timespan](https://en.wikipedia.org/wiki/ISO_8601#Durations)
        #[builder(into, default)]
        pub hot_cache_period: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Kusto Database to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The time the data should be kept before it stops being accessible to queries as ISO 8601 timespan. Default is unlimited. For more information see: [ISO 8601 Timespan](https://en.wikipedia.org/wiki/ISO_8601#Durations)
        #[builder(into, default)]
        pub soft_delete_period: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// Specifies the name of the Kusto Cluster this database will be added to. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The time the data that should be kept in cache for fast queries as ISO 8601 timespan. Default is unlimited. For more information see: [ISO 8601 Timespan](https://en.wikipedia.org/wiki/ISO_8601#Durations)
        pub hot_cache_period: pulumi_wasm_rust::Output<Option<String>>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Kusto Database to create. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The size of the database in bytes.
        pub size: pulumi_wasm_rust::Output<f64>,
        /// The time the data should be kept before it stops being accessible to queries as ISO 8601 timespan. Default is unlimited. For more information see: [ISO 8601 Timespan](https://en.wikipedia.org/wiki/ISO_8601#Durations)
        pub soft_delete_period: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DatabaseArgs,
    ) -> DatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let hot_cache_period_binding = args
            .hot_cache_period
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let soft_delete_period_binding = args
            .soft_delete_period
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "hotCachePeriod".into(),
                    value: &hot_cache_period_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "softDeletePeriod".into(),
                    value: &soft_delete_period_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatabaseResult {
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            hot_cache_period: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hotCachePeriod"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            size: pulumi_wasm_rust::__private::into_domain(o.extract_field("size")),
            soft_delete_period: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("softDeletePeriod"),
            ),
        }
    }
}
