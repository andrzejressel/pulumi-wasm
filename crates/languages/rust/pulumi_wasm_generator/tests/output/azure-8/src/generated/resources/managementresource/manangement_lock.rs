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
/// $ pulumi import azure:managementresource/manangementLock:ManangementLock lock1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Authorization/locks/lock1
/// ```
///
pub mod manangement_lock {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManangementLockArgs {
        /// Specifies the Level to be used for this Lock. Possible values are `CanNotDelete` and `ReadOnly`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `CanNotDelete` means authorized users are able to read and modify the resources, but not delete. `ReadOnly` means authorized users can only read from a resource, but they can't modify or delete it.
        #[builder(into)]
        pub lock_level: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Management Lock. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies some notes about the lock. Maximum of 512 characters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub notes: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the scope at which the Management Lock should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManangementLockResult {
        /// Specifies the Level to be used for this Lock. Possible values are `CanNotDelete` and `ReadOnly`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `CanNotDelete` means authorized users are able to read and modify the resources, but not delete. `ReadOnly` means authorized users can only read from a resource, but they can't modify or delete it.
        pub lock_level: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Management Lock. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies some notes about the lock. Maximum of 512 characters. Changing this forces a new resource to be created.
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the scope at which the Management Lock should be created. Changing this forces a new resource to be created.
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ManangementLockArgs,
    ) -> ManangementLockResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let lock_level_binding = args.lock_level.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let notes_binding = args.notes.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:managementresource/manangementLock:ManangementLock".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "lockLevel".into(),
                    value: &lock_level_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notes".into(),
                    value: &notes_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManangementLockResult {
            lock_level: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lockLevel"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            notes: pulumi_wasm_rust::__private::into_domain(o.extract_field("notes")),
            scope: pulumi_wasm_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
