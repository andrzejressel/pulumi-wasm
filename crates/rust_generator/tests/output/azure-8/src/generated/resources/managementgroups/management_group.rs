/// Manages a Management Group.
///
/// !> **Note:** Configuring `subscription_ids` is not supported when using the `azure.management.GroupSubscriptionAssociation` resource, results will be unpredictable.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleParent:
///     type: azure:management:Group
///     name: example_parent
///     properties:
///       displayName: ParentGroup
///       subscriptionIds:
///         - ${current.subscriptionId}
///   exampleChild:
///     type: azure:management:Group
///     name: example_child
///     properties:
///       displayName: ChildGroup
///       parentManagementGroupId: ${exampleParent.id}
///       subscriptionIds: # other subscription IDs can go here
///         - ${current.subscriptionId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Management Groups can be imported using the `management group resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:managementgroups/managementGroup:ManagementGroup example /providers/Microsoft.Management/managementGroups/group1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod management_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagementGroupArgs {
        /// A friendly name for this Management Group. If not specified, this will be the same as the `name`.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name or UUID for this Management Group, which needs to be unique across your tenant. A new UUID will be generated if not provided. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Parent Management Group.
        #[builder(into, default)]
        pub parent_management_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of Subscription GUIDs which should be assigned to the Management Group.
        ///
        /// > **Note:** To clear all Subscriptions from the Management Group set `subscription_ids` to an empty list
        #[builder(into, default)]
        pub subscription_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ManagementGroupResult {
        /// A friendly name for this Management Group. If not specified, this will be the same as the `name`.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The name or UUID for this Management Group, which needs to be unique across your tenant. A new UUID will be generated if not provided. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Parent Management Group.
        pub parent_management_group_id: pulumi_gestalt_rust::Output<String>,
        /// A list of Subscription GUIDs which should be assigned to the Management Group.
        ///
        /// > **Note:** To clear all Subscriptions from the Management Group set `subscription_ids` to an empty list
        pub subscription_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Management Group ID with the Tenant ID prefix.
        pub tenant_scoped_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ManagementGroupArgs,
    ) -> ManagementGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_management_group_id_binding = args
            .parent_management_group_id
            .get_output(context)
            .get_inner();
        let subscription_ids_binding = args
            .subscription_ids
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:managementgroups/managementGroup:ManagementGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parentManagementGroupId".into(),
                    value: &parent_management_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionIds".into(),
                    value: &subscription_ids_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagementGroupResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent_management_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentManagementGroupId"),
            ),
            subscription_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionIds"),
            ),
            tenant_scoped_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantScopedId"),
            ),
        }
    }
}
