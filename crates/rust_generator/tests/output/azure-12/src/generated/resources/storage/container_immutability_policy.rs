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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerImmutabilityPolicyArgs {
        /// The time interval in days that the data needs to be kept in a non-erasable and non-modifiable state.
        #[builder(into)]
        pub immutability_period_in_days: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Whether to lock this immutability policy. Cannot be set to `false` once the policy has been locked.
        ///
        /// !> **Locking an Immutability Policy** Once an Immutability Policy has been locked, it cannot be unlocked. After locking, it will only be possible to increase the value for `retention_period_in_days` up to 5 times for the lifetime of the policy. No other properties will be updateable. Furthermore, the Storage Container and the Storage Account in which it resides will become protected by the policy. It will no longer be possible to delete the Storage Container or the Storage Account. Please refer to [official documentation](https://learn.microsoft.com/en-us/azure/storage/blobs/immutable-policy-configure-container-scope?tabs=azure-portal#lock-a-time-based-retention-policy) for more information.
        #[builder(into, default)]
        pub locked: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to allow protected append writes to block and append blobs to the container. Defaults to `false`. Cannot be set with `protected_append_writes_enabled`.
        #[builder(into, default)]
        pub protected_append_writes_all_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to allow protected append writes to append blobs to the container. Defaults to `false`. Cannot be set with `protected_append_writes_all_enabled`.
        #[builder(into, default)]
        pub protected_append_writes_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Resource Manager ID of the Storage Container where this Immutability Policy should be applied. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_container_resource_manager_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct ContainerImmutabilityPolicyResult {
        /// The time interval in days that the data needs to be kept in a non-erasable and non-modifiable state.
        pub immutability_period_in_days: pulumi_gestalt_rust::Output<i32>,
        /// Whether to lock this immutability policy. Cannot be set to `false` once the policy has been locked.
        ///
        /// !> **Locking an Immutability Policy** Once an Immutability Policy has been locked, it cannot be unlocked. After locking, it will only be possible to increase the value for `retention_period_in_days` up to 5 times for the lifetime of the policy. No other properties will be updateable. Furthermore, the Storage Container and the Storage Account in which it resides will become protected by the policy. It will no longer be possible to delete the Storage Container or the Storage Account. Please refer to [official documentation](https://learn.microsoft.com/en-us/azure/storage/blobs/immutable-policy-configure-container-scope?tabs=azure-portal#lock-a-time-based-retention-policy) for more information.
        pub locked: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to allow protected append writes to block and append blobs to the container. Defaults to `false`. Cannot be set with `protected_append_writes_enabled`.
        pub protected_append_writes_all_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Whether to allow protected append writes to append blobs to the container. Defaults to `false`. Cannot be set with `protected_append_writes_all_enabled`.
        pub protected_append_writes_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Resource Manager ID of the Storage Container where this Immutability Policy should be applied. Changing this forces a new resource to be created.
        pub storage_container_resource_manager_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ContainerImmutabilityPolicyArgs,
    ) -> ContainerImmutabilityPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let immutability_period_in_days_binding = args
            .immutability_period_in_days
            .get_output(context)
            .get_inner();
        let locked_binding = args.locked.get_output(context).get_inner();
        let protected_append_writes_all_enabled_binding = args
            .protected_append_writes_all_enabled
            .get_output(context)
            .get_inner();
        let protected_append_writes_enabled_binding = args
            .protected_append_writes_enabled
            .get_output(context)
            .get_inner();
        let storage_container_resource_manager_id_binding = args
            .storage_container_resource_manager_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/containerImmutabilityPolicy:ContainerImmutabilityPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ContainerImmutabilityPolicyResult {
            immutability_period_in_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("immutabilityPeriodInDays"),
            ),
            locked: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locked"),
            ),
            protected_append_writes_all_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protectedAppendWritesAllEnabled"),
            ),
            protected_append_writes_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protectedAppendWritesEnabled"),
            ),
            storage_container_resource_manager_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageContainerResourceManagerId"),
            ),
        }
    }
}
