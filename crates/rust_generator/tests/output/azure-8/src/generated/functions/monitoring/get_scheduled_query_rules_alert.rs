#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_scheduled_query_rules_alert {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetScheduledQueryRulesAlertArgs {
        /// Specifies the name of the scheduled query rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group where the scheduled query rule is located.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetScheduledQueryRulesAlertResult {
        /// supports the following:
        pub actions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetScheduledQueryRulesAlertAction,
            >,
        >,
        /// The list of Resource IDs referred into query.
        pub authorized_resource_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The resource URI over which log search query is to be run.
        pub data_source_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the scheduled query rule.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether this scheduled query rule is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Frequency at which rule condition should be evaluated.
        pub frequency: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region where the resource should exist.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Log search query.
        pub query: pulumi_gestalt_rust::Output<String>,
        /// The type of query results.
        pub query_type: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Severity of the alert.
        pub severity: pulumi_gestalt_rust::Output<i32>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Time for which alerts should be throttled or suppressed.
        pub throttling: pulumi_gestalt_rust::Output<i32>,
        /// Time window for which data needs to be fetched for query.
        pub time_window: pulumi_gestalt_rust::Output<i32>,
        /// A `trigger` block as defined below.
        pub triggers: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetScheduledQueryRulesAlertTrigger,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetScheduledQueryRulesAlertArgs,
    ) -> GetScheduledQueryRulesAlertResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:monitoring/getScheduledQueryRulesAlert:getScheduledQueryRulesAlert"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
        GetScheduledQueryRulesAlertResult {
            actions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("actions"),
            ),
            authorized_resource_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizedResourceIds"),
            ),
            data_source_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataSourceId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            frequency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frequency"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            query: pulumi_gestalt_rust::__private::into_domain(o.extract_field("query")),
            query_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryType"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            severity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("severity"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            throttling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("throttling"),
            ),
            time_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeWindow"),
            ),
            triggers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggers"),
            ),
        }
    }
}
