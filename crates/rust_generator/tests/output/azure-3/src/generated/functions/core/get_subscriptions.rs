#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_subscriptions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubscriptionsArgs {
        /// A case-insensitive value which must be contained within the `display_name` field, used to filter the results
        #[builder(into, default)]
        pub display_name_contains: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A case-insensitive prefix which can be used to filter on the `display_name` field
        #[builder(into, default)]
        pub display_name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubscriptionsResult {
        pub display_name_contains: pulumi_gestalt_rust::Output<Option<String>>,
        pub display_name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// One or more `subscription` blocks as defined below.
        pub subscriptions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::core::GetSubscriptionsSubscription>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSubscriptionsArgs,
    ) -> GetSubscriptionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_contains_binding = args
            .display_name_contains
            .get_output(context);
        let display_name_prefix_binding = args.display_name_prefix.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:core/getSubscriptions:getSubscriptions".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayNameContains".into(),
                    value: display_name_contains_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayNamePrefix".into(),
                    value: display_name_prefix_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSubscriptionsResult {
            display_name_contains: o.get_field("displayNameContains"),
            display_name_prefix: o.get_field("displayNamePrefix"),
            id: o.get_field("id"),
            subscriptions: o.get_field("subscriptions"),
        }
    }
}
