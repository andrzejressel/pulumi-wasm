/// Manages a Management Group Subscription Association.
///
/// !> **Note:** When using this resource, configuring `subscription_ids` on the `azure.management.Group` resource is not supported.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleGroupSubscriptionAssociation:
///     type: azure:management:GroupSubscriptionAssociation
///     name: example
///     properties:
///       managementGroupId: ${example.id}
///       subscriptionId: ${exampleGetSubscription.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:management:getGroup
///       arguments:
///         name: exampleManagementGroup
///   exampleGetSubscription:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments:
///         subscriptionId: 12345678-1234-1234-1234-123456789012
/// ```
///
/// ## Import
///
/// Managements can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:management/groupSubscriptionAssociation:GroupSubscriptionAssociation example /providers/Microsoft.Management/managementGroups/MyManagementGroup/subscriptions/12345678-1234-1234-1234-123456789012
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_subscription_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupSubscriptionAssociationArgs {
        /// The ID of the Management Group to associate the Subscription with. Changing this forces a new Management to be created.
        #[builder(into)]
        pub management_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subscription to be associated with the Management Group. Changing this forces a new Management to be created.
        #[builder(into)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupSubscriptionAssociationResult {
        /// The ID of the Management Group to associate the Subscription with. Changing this forces a new Management to be created.
        pub management_group_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subscription to be associated with the Management Group. Changing this forces a new Management to be created.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GroupSubscriptionAssociationArgs,
    ) -> GroupSubscriptionAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let management_group_id_binding = args
            .management_group_id
            .get_output(context)
            .get_inner();
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:management/groupSubscriptionAssociation:GroupSubscriptionAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managementGroupId".into(),
                    value: &management_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupSubscriptionAssociationResult {
            management_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementGroupId"),
            ),
            subscription_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
        }
    }
}
