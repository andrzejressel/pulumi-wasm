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
pub mod lock {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LockArgs {
        /// Specifies the Level to be used for this Lock. Possible values are `CanNotDelete` and `ReadOnly`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `CanNotDelete` means authorized users are able to read and modify the resources, but not delete. `ReadOnly` means authorized users can only read from a resource, but they can't modify or delete it.
        #[builder(into)]
        pub lock_level: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Management Lock. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies some notes about the lock. Maximum of 512 characters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the scope at which the Management Lock should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LockResult {
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
    pub fn create(name: &str, args: LockArgs) -> LockResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let lock_level_binding = args.lock_level.get_inner();
        let name_binding = args.name.get_inner();
        let notes_binding = args.notes.get_inner();
        let scope_binding = args.scope.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:management/lock:Lock".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "lockLevel".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notes".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LockResult {
            lock_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lockLevel").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notes").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}
