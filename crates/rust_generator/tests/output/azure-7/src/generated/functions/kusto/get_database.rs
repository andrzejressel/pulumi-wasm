#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseArgs {
        /// The name of the Kusto Cluster this database is added to.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Kusto Database.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Resource Group where the Kusto Database exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseResult {
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// The time the data that should be kept in cache for fast queries as ISO 8601 timespan.
        pub hot_cache_period: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region in which the managed Kusto Database exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The size of the database in bytes.
        pub size: pulumi_gestalt_rust::Output<f64>,
        /// The time the data should be kept before it stops being accessible to queries as ISO 8601 timespan.
        pub soft_delete_period: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDatabaseArgs,
    ) -> GetDatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:kusto/getDatabase:getDatabase".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDatabaseResult {
            cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            hot_cache_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hotCachePeriod"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            size: pulumi_gestalt_rust::__private::into_domain(o.extract_field("size")),
            soft_delete_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("softDeletePeriod"),
            ),
        }
    }
}
