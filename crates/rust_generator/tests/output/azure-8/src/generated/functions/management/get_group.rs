#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupArgs {
        /// Specifies the display name of this Management Group.
        ///
        /// > **NOTE** Whilst multiple management groups may share the same display name, when filtering, the provider expects a single management group to be found with this name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name or UUID of this Management Group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetGroupResult {
        /// A list of Management Group IDs which directly or indirectly belong to this Management Group.
        pub all_management_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of Subscription IDs which are assigned to this Management Group or its children Management Groups.
        pub all_subscription_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of Management Group IDs which directly belong to this Management Group.
        pub management_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of any Parent Management Group.
        pub parent_management_group_id: pulumi_gestalt_rust::Output<String>,
        /// A list of Subscription IDs which are directly assigned to this Management Group.
        pub subscription_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Management Group ID with the Tenant ID prefix.
        pub tenant_scoped_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGroupArgs,
    ) -> GetGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:management/getGroup:getGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGroupResult {
            all_management_group_ids: o.get_field("allManagementGroupIds"),
            all_subscription_ids: o.get_field("allSubscriptionIds"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            management_group_ids: o.get_field("managementGroupIds"),
            name: o.get_field("name"),
            parent_management_group_id: o.get_field("parentManagementGroupId"),
            subscription_ids: o.get_field("subscriptionIds"),
            tenant_scoped_id: o.get_field("tenantScopedId"),
        }
    }
}
