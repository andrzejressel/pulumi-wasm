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
        context: &pulumi_gestalt_rust::Context,
        args: ListConfigurationsArgs,
    ) -> ListConfigurationsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_filters_binding = args
            .configuration_filters
            .get_output(context);
        let customer_subscription_details_binding = args
            .customer_subscription_details
            .get_output(context);
        let skip_token_binding = args.skip_token.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "myedgeorder::listConfigurations".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationFilters".into(),
                    value: configuration_filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerSubscriptionDetails".into(),
                    value: customer_subscription_details_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipToken".into(),
                    value: skip_token_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        ListConfigurationsResult {
            next_link: o.get_field("nextLink"),
            value: o.get_field("value"),
        }
    }
}
