/// Manages an Immutability Policy for a Container within an Azure Storage Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestoraccount
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       tags:
///         environment: staging
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: example
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleContainerImmutabilityPolicy:
///     type: azure:storage:ContainerImmutabilityPolicy
///     name: example
///     properties:
///       storageContainerResourceManagerId: ${exampleContainer.resourceManagerId}
///       immutabilityPeriodInDays: 14
///       protectedAppendWritesAllEnabled: false
///       protectedAppendWritesEnabled: true
/// ```
///
/// ## Import
///
/// Storage Container Immutability Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/containerImmutabilityPolicy:ContainerImmutabilityPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Storage/storageAccounts/myaccount/blobServices/default/containers/mycontainer/immutabilityPolicies/default
/// ```
///
pub mod container_immutability_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerImmutabilityPolicyArgs {
        /// The time interval in days that the data needs to be kept in a non-erasable and non-modifiable state.
        #[builder(into)]
        pub immutability_period_in_days: pulumi_wasm_rust::Output<i32>,
        /// Whether to lock this immutability policy. Cannot be set to `false` once the policy has been locked.
        ///
        /// !> **Locking an Immutability Policy** Once an Immutability Policy has been locked, it cannot be unlocked. After locking, it will only be possible to increase the value for `retention_period_in_days` up to 5 times for the lifetime of the policy. No other properties will be updateable. Furthermore, the Storage Container and the Storage Account in which it resides will become protected by the policy. It will no longer be possible to delete the Storage Container or the Storage Account. Please refer to [official documentation](https://learn.microsoft.com/en-us/azure/storage/blobs/immutable-policy-configure-container-scope?tabs=azure-portal#lock-a-time-based-retention-policy) for more information.
        #[builder(into, default)]
        pub locked: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to allow protected append writes to block and append blobs to the container. Defaults to `false`. Cannot be set with `protected_append_writes_enabled`.
        #[builder(into, default)]
        pub protected_append_writes_all_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to allow protected append writes to append blobs to the container. Defaults to `false`. Cannot be set with `protected_append_writes_all_enabled`.
        #[builder(into, default)]
        pub protected_append_writes_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Resource Manager ID of the Storage Container where this Immutability Policy should be applied. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_container_resource_manager_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ContainerImmutabilityPolicyResult {
        /// The time interval in days that the data needs to be kept in a non-erasable and non-modifiable state.
        pub immutability_period_in_days: pulumi_wasm_rust::Output<i32>,
        /// Whether to lock this immutability policy. Cannot be set to `false` once the policy has been locked.
        ///
        /// !> **Locking an Immutability Policy** Once an Immutability Policy has been locked, it cannot be unlocked. After locking, it will only be possible to increase the value for `retention_period_in_days` up to 5 times for the lifetime of the policy. No other properties will be updateable. Furthermore, the Storage Container and the Storage Account in which it resides will become protected by the policy. It will no longer be possible to delete the Storage Container or the Storage Account. Please refer to [official documentation](https://learn.microsoft.com/en-us/azure/storage/blobs/immutable-policy-configure-container-scope?tabs=azure-portal#lock-a-time-based-retention-policy) for more information.
        pub locked: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to allow protected append writes to block and append blobs to the container. Defaults to `false`. Cannot be set with `protected_append_writes_enabled`.
        pub protected_append_writes_all_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to allow protected append writes to append blobs to the container. Defaults to `false`. Cannot be set with `protected_append_writes_all_enabled`.
        pub protected_append_writes_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Resource Manager ID of the Storage Container where this Immutability Policy should be applied. Changing this forces a new resource to be created.
        pub storage_container_resource_manager_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ContainerImmutabilityPolicyArgs,
    ) -> ContainerImmutabilityPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let immutability_period_in_days_binding = args
            .immutability_period_in_days
            .get_inner();
        let locked_binding = args.locked.get_inner();
        let protected_append_writes_all_enabled_binding = args
            .protected_append_writes_all_enabled
            .get_inner();
        let protected_append_writes_enabled_binding = args
            .protected_append_writes_enabled
            .get_inner();
        let storage_container_resource_manager_id_binding = args
            .storage_container_resource_manager_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/containerImmutabilityPolicy:ContainerImmutabilityPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "immutabilityPeriodInDays".into(),
                    value: &immutability_period_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "locked".into(),
                    value: &locked_binding,
                },
                register_interface::ObjectField {
                    name: "protectedAppendWritesAllEnabled".into(),
                    value: &protected_append_writes_all_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "protectedAppendWritesEnabled".into(),
                    value: &protected_append_writes_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "storageContainerResourceManagerId".into(),
                    value: &storage_container_resource_manager_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "immutabilityPeriodInDays".into(),
                },
                register_interface::ResultField {
                    name: "locked".into(),
                },
                register_interface::ResultField {
                    name: "protectedAppendWritesAllEnabled".into(),
                },
                register_interface::ResultField {
                    name: "protectedAppendWritesEnabled".into(),
                },
                register_interface::ResultField {
                    name: "storageContainerResourceManagerId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContainerImmutabilityPolicyResult {
            immutability_period_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("immutabilityPeriodInDays").unwrap(),
            ),
            locked: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locked").unwrap(),
            ),
            protected_append_writes_all_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectedAppendWritesAllEnabled").unwrap(),
            ),
            protected_append_writes_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectedAppendWritesEnabled").unwrap(),
            ),
            storage_container_resource_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageContainerResourceManagerId").unwrap(),
            ),
        }
    }
}