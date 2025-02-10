#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_volume {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeArgs {
        /// The name of the NetApp account where the NetApp pool exists.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the NetApp Volume.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the NetApp pool where the NetApp volume exists.
        #[builder(into)]
        pub pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the NetApp Volume exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Volume security style
        #[builder(into, default)]
        pub security_style: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetVolumeResult {
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// A data protecion backup policy block
        pub data_protection_backup_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::netapp::GetVolumeDataProtectionBackupPolicy>,
        >,
        /// Volume data protection replication block
        pub data_protection_replications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::netapp::GetVolumeDataProtectionReplication>,
        >,
        pub encryption_key_source: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_vault_private_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the NetApp Volume exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A list of IPv4 Addresses which should be used to mount the volume.
        pub mount_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Network features in use `Basic` or `Standard`.
        pub network_features: pulumi_gestalt_rust::Output<String>,
        pub pool_name: pulumi_gestalt_rust::Output<String>,
        /// A list of protocol types enabled on volume.
        pub protocols: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Volume security style
        pub security_style: pulumi_gestalt_rust::Output<Option<String>>,
        /// The service level of the file system.
        pub service_level: pulumi_gestalt_rust::Output<String>,
        /// Limits enumeration of files and folders (that is, listing the contents) in SMB only to users with allowed access on the share.
        pub smb_access_based_enumeration_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Limits clients from browsing for an SMB share.
        pub smb_non_browsable_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The maximum Storage Quota in Gigabytes allowed for a file system.
        pub storage_quota_in_gb: pulumi_gestalt_rust::Output<i32>,
        /// The ID of a Subnet in which the NetApp Volume resides.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The unique file path of the volume.
        pub volume_path: pulumi_gestalt_rust::Output<String>,
        /// The Availability Zone in which the Volume is located.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVolumeArgs,
    ) -> GetVolumeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let pool_name_binding = args.pool_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let security_style_binding = args.security_style.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:netapp/getVolume:getVolume".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "poolName".into(),
                    value: pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityStyle".into(),
                    value: security_style_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVolumeResult {
            account_name: o.get_field("accountName"),
            data_protection_backup_policies: o.get_field("dataProtectionBackupPolicies"),
            data_protection_replications: o.get_field("dataProtectionReplications"),
            encryption_key_source: o.get_field("encryptionKeySource"),
            id: o.get_field("id"),
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
            smb_access_based_enumeration_enabled: o
                .get_field("smbAccessBasedEnumerationEnabled"),
            smb_non_browsable_enabled: o.get_field("smbNonBrowsableEnabled"),
            storage_quota_in_gb: o.get_field("storageQuotaInGb"),
            subnet_id: o.get_field("subnetId"),
            volume_path: o.get_field("volumePath"),
            zone: o.get_field("zone"),
        }
    }
}
