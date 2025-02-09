/// Manages a Storage Object Replication.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dst = resource_group::create(
///         "dst",
///         ResourceGroupArgs::builder()
///             .location("East US")
///             .name("dstResourceGroupName")
///             .build_struct(),
///     );
///     let dstAccount = account::create(
///         "dstAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .blob_properties(
///                 AccountBlobProperties::builder()
///                     .changeFeedEnabled(true)
///                     .versioningEnabled(true)
///                     .build_struct(),
///             )
///             .location("${dst.location}")
///             .name("dststorageaccount")
///             .resource_group_name("${dst.name}")
///             .build_struct(),
///     );
///     let dstContainer = container::create(
///         "dstContainer",
///         ContainerArgs::builder()
///             .container_access_type("private")
///             .name("dststrcontainer")
///             .storage_account_name("${dstAccount.name}")
///             .build_struct(),
///     );
///     let example = object_replication::create(
///         "example",
///         ObjectReplicationArgs::builder()
///             .destination_storage_account_id("${dstAccount.id}")
///             .rules(
///                 vec![
///                     ObjectReplicationRule::builder()
///                     .destinationContainerName("${dstContainer.name}")
///                     .sourceContainerName("${srcContainer.name}").build_struct(),
///                 ],
///             )
///             .source_storage_account_id("${srcAccount.id}")
///             .build_struct(),
///     );
///     let src = resource_group::create(
///         "src",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("srcResourceGroupName")
///             .build_struct(),
///     );
///     let srcAccount = account::create(
///         "srcAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .blob_properties(
///                 AccountBlobProperties::builder()
///                     .changeFeedEnabled(true)
///                     .versioningEnabled(true)
///                     .build_struct(),
///             )
///             .location("${src.location}")
///             .name("srcstorageaccount")
///             .resource_group_name("${src.name}")
///             .build_struct(),
///     );
///     let srcContainer = container::create(
///         "srcContainer",
///         ContainerArgs::builder()
///             .container_access_type("private")
///             .name("srcstrcontainer")
///             .storage_account_name("${srcAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Object Replication Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/objectReplication:ObjectReplication example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Storage/storageAccounts/storageAccount1/objectReplicationPolicies/objectReplicationPolicy1;/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group2/providers/Microsoft.Storage/storageAccounts/storageAccount2/objectReplicationPolicies/objectReplicationPolicy2
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod object_replication {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectReplicationArgs {
        /// The ID of the destination storage account. Changing this forces a new Storage Object Replication to be created.
        #[builder(into)]
        pub destination_storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `rules` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::storage::ObjectReplicationRule>,
        >,
        /// The ID of the source storage account. Changing this forces a new Storage Object Replication to be created.
        #[builder(into)]
        pub source_storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ObjectReplicationResult {
        /// The ID of the Object Replication in the destination storage account.
        pub destination_object_replication_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the destination storage account. Changing this forces a new Storage Object Replication to be created.
        pub destination_storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `rules` blocks as defined below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::storage::ObjectReplicationRule>,
        >,
        /// The ID of the Object Replication in the source storage account.
        pub source_object_replication_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the source storage account. Changing this forces a new Storage Object Replication to be created.
        pub source_storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ObjectReplicationArgs,
    ) -> ObjectReplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_storage_account_id_binding = args
            .destination_storage_account_id
            .get_output(context);
        let rules_binding = args.rules.get_output(context);
        let source_storage_account_id_binding = args
            .source_storage_account_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/objectReplication:ObjectReplication".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationStorageAccountId".into(),
                    value: destination_storage_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceStorageAccountId".into(),
                    value: source_storage_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ObjectReplicationResult {
            destination_object_replication_id: o
                .get_field("destinationObjectReplicationId"),
            destination_storage_account_id: o.get_field("destinationStorageAccountId"),
            rules: o.get_field("rules"),
            source_object_replication_id: o.get_field("sourceObjectReplicationId"),
            source_storage_account_id: o.get_field("sourceStorageAccountId"),
        }
    }
}
