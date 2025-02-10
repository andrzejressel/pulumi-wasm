/// Manages a Management Lock which is scoped to a Subscription, Resource Group or Resource.
///
/// ## Example Usage
///
/// ### Subscription Level Lock)
///
/// ```yaml
/// resources:
///   subscription-level:
///     type: azure:management:Lock
///     properties:
///       name: subscription-level
///       scope: ${current.id}
///       lockLevel: CanNotDelete
///       notes: Items can't be deleted in this subscription!
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
///
/// ### Resource Group Level Lock)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: locked-resource-group
///       location: West Europe
///   resource-group-level:
///     type: azure:management:Lock
///     properties:
///       name: resource-group-level
///       scope: ${example.id}
///       lockLevel: ReadOnly
///       notes: This Resource Group is Read-Only
/// ```
///
///
/// ### Resource Level Lock)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: locked-resource-group
///       location: West Europe
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: locked-publicip
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///       idleTimeoutInMinutes: 30
///   public-ip:
///     type: azure:management:Lock
///     properties:
///       name: resource-ip
///       scope: ${examplePublicIp.id}
///       lockLevel: CanNotDelete
///       notes: Locked because it's needed by a third-party
/// ```
///
/// ## Import
///
/// Management Locks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:management/lock:Lock lock1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Authorization/locks/lock1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lock {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LockArgs {
        /// Specifies the Level to be used for this Lock. Possible values are `CanNotDelete` and `ReadOnly`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `CanNotDelete` means authorized users are able to read and modify the resources, but not delete. `ReadOnly` means authorized users can only read from a resource, but they can't modify or delete it.
        #[builder(into)]
        pub lock_level: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Management Lock. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies some notes about the lock. Maximum of 512 characters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub notes: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the scope at which the Management Lock should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LockResult {
        /// Specifies the Level to be used for this Lock. Possible values are `CanNotDelete` and `ReadOnly`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `CanNotDelete` means authorized users are able to read and modify the resources, but not delete. `ReadOnly` means authorized users can only read from a resource, but they can't modify or delete it.
        pub lock_level: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Management Lock. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies some notes about the lock. Maximum of 512 characters. Changing this forces a new resource to be created.
        pub notes: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the scope at which the Management Lock should be created. Changing this forces a new resource to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LockArgs,
    ) -> LockResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let lock_level_binding = args.lock_level.get_output(context);
        let name_binding = args.name.get_output(context);
        let notes_binding = args.notes.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:management/lock:Lock".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lockLevel".into(),
                    value: lock_level_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notes".into(),
                    value: notes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LockResult {
            lock_level: o.get_field("lockLevel"),
            name: o.get_field("name"),
            notes: o.get_field("notes"),
            scope: o.get_field("scope"),
        }
    }
}
