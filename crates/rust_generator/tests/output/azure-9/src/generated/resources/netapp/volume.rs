/// Manages a NetApp Volume.
///
/// !>**IMPORTANT:** This resource uses a feature to prevent deletion called `prevent_volume_destruction`, defaulting to `true`. It is intentionally set to `true` to prevent the possibility of accidental data loss. The example in this page shows all possible protection options you can apply, it is using same values as the defaults.
///
/// ## Import
///
/// NetApp Volumes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/volume:Volume example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.NetApp/netAppAccounts/account1/capacityPools/pool1/volumes/volume1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod volume {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeArgs {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is the NetApp Volume enabled for Azure VMware Solution (AVS) datastore purpose. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub azure_vmware_data_store_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Creates volume from snapshot. Following properties must be the same as the original volume where the snapshot was taken from: `protocols`, `subnet_id`, `location`, `service_level`, `resource_group_name`, `account_name` and `pool_name`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub create_from_snapshot_resource_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `data_protection_backup_policy` block as defined below.
        #[builder(into, default)]
        pub data_protection_backup_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::netapp::VolumeDataProtectionBackupPolicy>,
        >,
        /// A `data_protection_replication` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub data_protection_replication: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::netapp::VolumeDataProtectionReplication>,
        >,
        /// A `data_protection_snapshot_policy` block as defined below.
        #[builder(into, default)]
        pub data_protection_snapshot_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::netapp::VolumeDataProtectionSnapshotPolicy>,
        >,
        /// The encryption key source, it can be `Microsoft.NetApp` for platform managed keys or `Microsoft.KeyVault` for customer-managed keys. This is required with `key_vault_private_endpoint_id`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_key_source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `export_policy_rule` block defined below.
        #[builder(into, default)]
        pub export_policy_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::netapp::VolumeExportPolicyRule>>,
        >,
        /// Enable to allow Kerberos secured volumes. Requires appropriate export rules as well as the parent `azure.netapp.Account`
        /// having a defined AD connection.
        #[builder(into, default)]
        pub kerberos_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Private Endpoint ID for Key Vault, which is required when using customer-managed keys. This is required with `encryption_key_source`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub key_vault_private_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the NetApp Volume. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates which network feature to use, accepted values are `Basic` or `Standard`, it defaults to `Basic` if not defined. This is a feature in public preview and for more information about it and how to register, please refer to [Configure network features for an Azure NetApp Files volume](https://docs.microsoft.com/en-us/azure/azure-netapp-files/configure-network-features).
        #[builder(into, default)]
        pub network_features: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the NetApp pool in which the NetApp Volume should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The target volume protocol expressed as a list. Supported single value include `CIFS`, `NFSv3`, or `NFSv4.1`. If argument is not defined it will default to `NFSv3`. Changing this forces a new resource to be created and data will be lost. Dual protocol scenario is supported for CIFS and NFSv3, for more information, please refer to [Create a dual-protocol volume for Azure NetApp Files](https://docs.microsoft.com/azure/azure-netapp-files/create-volumes-dual-protocol) document.
        #[builder(into, default)]
        pub protocols: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the resource group where the NetApp Volume should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Volume security style, accepted values are `unix` or `ntfs`. If not provided, single-protocol volume is created defaulting to `unix` if it is `NFSv3` or `NFSv4.1` volume, if `CIFS`, it will default to `ntfs`. In a dual-protocol volume, if not provided, its value will be `ntfs`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub security_style: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The target performance of the file system. Valid values include `Premium`, `Standard`, or `Ultra`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub service_level: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enable SMB encryption.
        #[builder(into, default)]
        pub smb3_protocol_encryption_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Limits enumeration of files and folders (that is, listing the contents) in SMB only to users with allowed access on the share. For instance, if a user doesn't have access to read a file or folder in a share with access-based enumeration enabled, then the file or folder doesn't show up in directory listings. Defaults to `false`. For more information, please refer to [Understand NAS share permissions in Azure NetApp Files](https://learn.microsoft.com/en-us/azure/azure-netapp-files/network-attached-storage-permissions#:~:text=security%20for%20administrators.-,Access%2Dbased%20enumeration,in%20an%20Azure%20NetApp%20Files%20SMB%20volume.%20Only%20contosoadmin%20has%20access.,-In%20the%20below)
        #[builder(into, default)]
        pub smb_access_based_enumeration_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Enable SMB Continuous Availability.
        #[builder(into, default)]
        pub smb_continuous_availability_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Limits clients from browsing for an SMB share by hiding the share from view in Windows Explorer or when listing shares in "net view." Only end users that know the absolute paths to the share are able to find the share. Defaults to `false`. For more information, please refer to [Understand NAS share permissions in Azure NetApp Files](https://learn.microsoft.com/en-us/azure/azure-netapp-files/network-attached-storage-permissions#:~:text=Non%2Dbrowsable%20shares,find%20the%20share.)
        #[builder(into, default)]
        pub smb_non_browsable_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether the .snapshot (NFS clients) or ~snapshot (SMB clients) path of a volume is visible, default value is true.
        #[builder(into, default)]
        pub snapshot_directory_visible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The maximum Storage Quota allowed for a file system in Gigabytes.
        #[builder(into)]
        pub storage_quota_in_gb: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of the Subnet the NetApp Volume resides in, which must have the `Microsoft.NetApp/volumes` delegation. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **Note:** It is highly recommended to use the **lifecycle** property as noted in the example since it will prevent an accidental deletion of the volume if the `protocols` argument changes to a different protocol type.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Throughput of this volume in Mibps.
        #[builder(into, default)]
        pub throughput_in_mibps: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// A unique file path for the volume. Used when creating mount targets. Changing this forces a new resource to be created.
        #[builder(into)]
        pub volume_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Availability Zone in which the Volume should be located. Possible values are `1`, `2` and `3`. Changing this forces a new resource to be created. This feature is currently in preview, for more information on how to enable it, please refer to [Manage availability zone volume placement for Azure NetApp Files](https://learn.microsoft.com/en-us/azure/azure-netapp-files/manage-availability-zone-volume-placement#register-the-feature).
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VolumeResult {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// Is the NetApp Volume enabled for Azure VMware Solution (AVS) datastore purpose. Defaults to `false`. Changing this forces a new resource to be created.
        pub azure_vmware_data_store_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Creates volume from snapshot. Following properties must be the same as the original volume where the snapshot was taken from: `protocols`, `subnet_id`, `location`, `service_level`, `resource_group_name`, `account_name` and `pool_name`. Changing this forces a new resource to be created.
        pub create_from_snapshot_resource_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// A `data_protection_backup_policy` block as defined below.
        pub data_protection_backup_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::netapp::VolumeDataProtectionBackupPolicy>,
        >,
        /// A `data_protection_replication` block as defined below. Changing this forces a new resource to be created.
        pub data_protection_replication: pulumi_gestalt_rust::Output<
            Option<super::super::types::netapp::VolumeDataProtectionReplication>,
        >,
        /// A `data_protection_snapshot_policy` block as defined below.
        pub data_protection_snapshot_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::netapp::VolumeDataProtectionSnapshotPolicy>,
        >,
        /// The encryption key source, it can be `Microsoft.NetApp` for platform managed keys or `Microsoft.KeyVault` for customer-managed keys. This is required with `key_vault_private_endpoint_id`. Changing this forces a new resource to be created.
        pub encryption_key_source: pulumi_gestalt_rust::Output<String>,
        /// One or more `export_policy_rule` block defined below.
        pub export_policy_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::netapp::VolumeExportPolicyRule>>,
        >,
        /// Enable to allow Kerberos secured volumes. Requires appropriate export rules as well as the parent `azure.netapp.Account`
        /// having a defined AD connection.
        pub kerberos_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Private Endpoint ID for Key Vault, which is required when using customer-managed keys. This is required with `encryption_key_source`. Changing this forces a new resource to be created.
        pub key_vault_private_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A list of IPv4 Addresses which should be used to mount the volume.
        pub mount_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the NetApp Volume. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Indicates which network feature to use, accepted values are `Basic` or `Standard`, it defaults to `Basic` if not defined. This is a feature in public preview and for more information about it and how to register, please refer to [Configure network features for an Azure NetApp Files volume](https://docs.microsoft.com/en-us/azure/azure-netapp-files/configure-network-features).
        pub network_features: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the NetApp pool in which the NetApp Volume should be created. Changing this forces a new resource to be created.
        pub pool_name: pulumi_gestalt_rust::Output<String>,
        /// The target volume protocol expressed as a list. Supported single value include `CIFS`, `NFSv3`, or `NFSv4.1`. If argument is not defined it will default to `NFSv3`. Changing this forces a new resource to be created and data will be lost. Dual protocol scenario is supported for CIFS and NFSv3, for more information, please refer to [Create a dual-protocol volume for Azure NetApp Files](https://docs.microsoft.com/azure/azure-netapp-files/create-volumes-dual-protocol) document.
        pub protocols: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the resource group where the NetApp Volume should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Volume security style, accepted values are `unix` or `ntfs`. If not provided, single-protocol volume is created defaulting to `unix` if it is `NFSv3` or `NFSv4.1` volume, if `CIFS`, it will default to `ntfs`. In a dual-protocol volume, if not provided, its value will be `ntfs`. Changing this forces a new resource to be created.
        pub security_style: pulumi_gestalt_rust::Output<String>,
        /// The target performance of the file system. Valid values include `Premium`, `Standard`, or `Ultra`. Changing this forces a new resource to be created.
        pub service_level: pulumi_gestalt_rust::Output<String>,
        /// Enable SMB encryption.
        pub smb3_protocol_encryption_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Limits enumeration of files and folders (that is, listing the contents) in SMB only to users with allowed access on the share. For instance, if a user doesn't have access to read a file or folder in a share with access-based enumeration enabled, then the file or folder doesn't show up in directory listings. Defaults to `false`. For more information, please refer to [Understand NAS share permissions in Azure NetApp Files](https://learn.microsoft.com/en-us/azure/azure-netapp-files/network-attached-storage-permissions#:~:text=security%20for%20administrators.-,Access%2Dbased%20enumeration,in%20an%20Azure%20NetApp%20Files%20SMB%20volume.%20Only%20contosoadmin%20has%20access.,-In%20the%20below)
        pub smb_access_based_enumeration_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Enable SMB Continuous Availability.
        pub smb_continuous_availability_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Limits clients from browsing for an SMB share by hiding the share from view in Windows Explorer or when listing shares in "net view." Only end users that know the absolute paths to the share are able to find the share. Defaults to `false`. For more information, please refer to [Understand NAS share permissions in Azure NetApp Files](https://learn.microsoft.com/en-us/azure/azure-netapp-files/network-attached-storage-permissions#:~:text=Non%2Dbrowsable%20shares,find%20the%20share.)
        pub smb_non_browsable_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether the .snapshot (NFS clients) or ~snapshot (SMB clients) path of a volume is visible, default value is true.
        pub snapshot_directory_visible: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The maximum Storage Quota allowed for a file system in Gigabytes.
        pub storage_quota_in_gb: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the Subnet the NetApp Volume resides in, which must have the `Microsoft.NetApp/volumes` delegation. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **Note:** It is highly recommended to use the **lifecycle** property as noted in the example since it will prevent an accidental deletion of the volume if the `protocols` argument changes to a different protocol type.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Throughput of this volume in Mibps.
        pub throughput_in_mibps: pulumi_gestalt_rust::Output<f64>,
        /// A unique file path for the volume. Used when creating mount targets. Changing this forces a new resource to be created.
        pub volume_path: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Availability Zone in which the Volume should be located. Possible values are `1`, `2` and `3`. Changing this forces a new resource to be created. This feature is currently in preview, for more information on how to enable it, please refer to [Manage availability zone volume placement for Azure NetApp Files](https://learn.microsoft.com/en-us/azure/azure-netapp-files/manage-availability-zone-volume-placement#register-the-feature).
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VolumeArgs,
    ) -> VolumeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let azure_vmware_data_store_enabled_binding = args
            .azure_vmware_data_store_enabled
            .get_output(context);
        let create_from_snapshot_resource_id_binding = args
            .create_from_snapshot_resource_id
            .get_output(context);
        let data_protection_backup_policy_binding = args
            .data_protection_backup_policy
            .get_output(context);
        let data_protection_replication_binding = args
            .data_protection_replication
            .get_output(context);
        let data_protection_snapshot_policy_binding = args
            .data_protection_snapshot_policy
            .get_output(context);
        let encryption_key_source_binding = args
            .encryption_key_source
            .get_output(context);
        let export_policy_rules_binding = args.export_policy_rules.get_output(context);
        let kerberos_enabled_binding = args.kerberos_enabled.get_output(context);
        let key_vault_private_endpoint_id_binding = args
            .key_vault_private_endpoint_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_features_binding = args.network_features.get_output(context);
        let pool_name_binding = args.pool_name.get_output(context);
        let protocols_binding = args.protocols.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let security_style_binding = args.security_style.get_output(context);
        let service_level_binding = args.service_level.get_output(context);
        let smb3_protocol_encryption_enabled_binding = args
            .smb3_protocol_encryption_enabled
            .get_output(context);
        let smb_access_based_enumeration_enabled_binding = args
            .smb_access_based_enumeration_enabled
            .get_output(context);
        let smb_continuous_availability_enabled_binding = args
            .smb_continuous_availability_enabled
            .get_output(context);
        let smb_non_browsable_enabled_binding = args
            .smb_non_browsable_enabled
            .get_output(context);
        let snapshot_directory_visible_binding = args
            .snapshot_directory_visible
            .get_output(context);
        let storage_quota_in_gb_binding = args.storage_quota_in_gb.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let throughput_in_mibps_binding = args.throughput_in_mibps.get_output(context);
        let volume_path_binding = args.volume_path.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:netapp/volume:Volume".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureVmwareDataStoreEnabled".into(),
                    value: azure_vmware_data_store_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createFromSnapshotResourceId".into(),
                    value: create_from_snapshot_resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataProtectionBackupPolicy".into(),
                    value: data_protection_backup_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataProtectionReplication".into(),
                    value: data_protection_replication_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataProtectionSnapshotPolicy".into(),
                    value: data_protection_snapshot_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionKeySource".into(),
                    value: encryption_key_source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportPolicyRules".into(),
                    value: export_policy_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kerberosEnabled".into(),
                    value: kerberos_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultPrivateEndpointId".into(),
                    value: key_vault_private_endpoint_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkFeatures".into(),
                    value: network_features_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "poolName".into(),
                    value: pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocols".into(),
                    value: protocols_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityStyle".into(),
                    value: security_style_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceLevel".into(),
                    value: service_level_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smb3ProtocolEncryptionEnabled".into(),
                    value: smb3_protocol_encryption_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smbAccessBasedEnumerationEnabled".into(),
                    value: smb_access_based_enumeration_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smbContinuousAvailabilityEnabled".into(),
                    value: smb_continuous_availability_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smbNonBrowsableEnabled".into(),
                    value: smb_non_browsable_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotDirectoryVisible".into(),
                    value: snapshot_directory_visible_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageQuotaInGb".into(),
                    value: storage_quota_in_gb_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throughputInMibps".into(),
                    value: throughput_in_mibps_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumePath".into(),
                    value: volume_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VolumeResult {
            account_name: o.get_field("accountName"),
            azure_vmware_data_store_enabled: o.get_field("azureVmwareDataStoreEnabled"),
            create_from_snapshot_resource_id: o
                .get_field("createFromSnapshotResourceId"),
            data_protection_backup_policy: o.get_field("dataProtectionBackupPolicy"),
            data_protection_replication: o.get_field("dataProtectionReplication"),
            data_protection_snapshot_policy: o.get_field("dataProtectionSnapshotPolicy"),
            encryption_key_source: o.get_field("encryptionKeySource"),
            export_policy_rules: o.get_field("exportPolicyRules"),
            kerberos_enabled: o.get_field("kerberosEnabled"),
            key_vault_private_endpoint_id: o.get_field("keyVaultPrivateEndpointId"),
            location: o.get_field("location"),
            mount_ip_addresses: o.get_field("mountIpAddresses"),
            name: o.get_field("name"),
            network_features: o.get_field("networkFeatures"),
            pool_name: o.get_field("poolName"),
            protocols: o.get_field("protocols"),
            resource_group_name: o.get_field("resourceGroupName"),
            security_style: o.get_field("securityStyle"),
            service_level: o.get_field("serviceLevel"),
            smb3_protocol_encryption_enabled: o
                .get_field("smb3ProtocolEncryptionEnabled"),
            smb_access_based_enumeration_enabled: o
                .get_field("smbAccessBasedEnumerationEnabled"),
            smb_continuous_availability_enabled: o
                .get_field("smbContinuousAvailabilityEnabled"),
            smb_non_browsable_enabled: o.get_field("smbNonBrowsableEnabled"),
            snapshot_directory_visible: o.get_field("snapshotDirectoryVisible"),
            storage_quota_in_gb: o.get_field("storageQuotaInGb"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            throughput_in_mibps: o.get_field("throughputInMibps"),
            volume_path: o.get_field("volumePath"),
            zone: o.get_field("zone"),
        }
    }
}
