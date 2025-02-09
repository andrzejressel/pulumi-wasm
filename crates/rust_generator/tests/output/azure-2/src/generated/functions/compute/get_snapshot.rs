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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSnapshotArgs,
    ) -> GetSnapshotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSnapshotResult {
            creation_option: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationOption"),
            ),
            disk_size_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskSizeGb"),
            ),
            encryption_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionSettings"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            os_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osType"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            source_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceResourceId"),
            ),
            source_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceUri"),
            ),
            storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            time_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeCreated"),
            ),
            trusted_launch_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trustedLaunchEnabled"),
            ),
        }
    }
}
