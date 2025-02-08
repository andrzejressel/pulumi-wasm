#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod list_configurations {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListConfigurationsArgs {
        /// Holds details about product hierarchy information and filterable property.
        #[builder(into)]
        pub configuration_filters: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::ConfigurationFilters>,
        >,
        /// Customer subscription properties. Clients can display available products to unregistered customers by explicitly passing subscription details
        #[builder(into, default)]
        pub customer_subscription_details: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::CustomerSubscriptionDetails>,
        >,
        /// $skipToken is supported on list of configurations, which provides the next page in the list of configurations.
        #[builder(into, default)]
        pub skip_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ListConfigurationsResult {
        /// Link for the next set of configurations.
        pub next_link: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of configurations.
        pub value: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ConfigurationResponse>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: ListConfigurationsArgs,
    ) -> ListConfigurationsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let configuration_filters_binding = args
            .configuration_filters
            .get_output(context)
            .get_inner();
        let customer_subscription_details_binding = args
            .customer_subscription_details
            .get_output(context)
            .get_inner();
        let skip_token_binding = args.skip_token.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "myedgeorder::listConfigurations".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationFilters".into(),
                    value: &configuration_filters_binding,
                },
                register_interface::ObjectField {
                    name: "customerSubscriptionDetails".into(),
                    value: &customer_subscription_details_binding,
                },
                register_interface::ObjectField {
                    name: "skipToken".into(),
                    value: &skip_token_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        ListConfigurationsResult {
            next_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nextLink"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
