#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_scheduled_query_rules_log {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetScheduledQueryRulesLogArgs {
        /// Specifies the name of the scheduled query rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group where the scheduled query rule is located.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetScheduledQueryRulesLogResult {
        /// A list of IDs of Resources referred into query.
        pub authorized_resource_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `criteria` block as defined below.
        pub criterias: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetScheduledQueryRulesLogCriteria,
            >,
        >,
        /// The resource URI over which log search query is to be run.
        pub data_source_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the scheduled query rule.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether this scheduled query rule is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region where the resource should exist.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the dimension.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetScheduledQueryRulesLogArgs,
    ) -> GetScheduledQueryRulesLogResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:monitoring/getScheduledQueryRulesLog:getScheduledQueryRulesLog"
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
        GetScheduledQueryRulesLogResult {
            authorized_resource_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizedResourceIds"),
            ),
            criterias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("criterias"),
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
