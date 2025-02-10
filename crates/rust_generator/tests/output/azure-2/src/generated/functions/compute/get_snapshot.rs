#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// Specifies the name of the Snapshot.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Snapshot is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        pub creation_option: pulumi_gestalt_rust::Output<String>,
        /// The size of the Snapshotted Disk in GB.
        pub disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        pub encryption_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetSnapshotEncryptionSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub os_type: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The reference to an existing snapshot.
        pub source_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The URI to a Managed or Unmanaged Disk.
        pub source_uri: pulumi_gestalt_rust::Output<String>,
        /// The ID of an storage account.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
        pub time_created: pulumi_gestalt_rust::Output<String>,
        /// Whether Trusted Launch is enabled for the Snapshot.
        pub trusted_launch_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSnapshotArgs,
    ) -> GetSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSnapshotResult {
            creation_option: o.get_field("creationOption"),
            disk_size_gb: o.get_field("diskSizeGb"),
            encryption_settings: o.get_field("encryptionSettings"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            os_type: o.get_field("osType"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_resource_id: o.get_field("sourceResourceId"),
            source_uri: o.get_field("sourceUri"),
            storage_account_id: o.get_field("storageAccountId"),
            time_created: o.get_field("timeCreated"),
            trusted_launch_enabled: o.get_field("trustedLaunchEnabled"),
        }
    }
}
