pub mod get_volume {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeArgs {
        /// The name of the NetApp account where the NetApp pool exists.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp Volume.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp pool where the NetApp volume exists.
        #[builder(into)]
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the NetApp Volume exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Volume security style
        #[builder(into, default)]
        pub security_style: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetVolumeResult {
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// A data protecion backup policy block
        pub data_protection_backup_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::netapp::GetVolumeDataProtectionBackupPolicy>,
        >,
        /// Volume data protection replication block
        pub data_protection_replications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::netapp::GetVolumeDataProtectionReplication>,
        >,
        pub encryption_key_source: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key_vault_private_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the NetApp Volume exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A list of IPv4 Addresses which should be used to mount the volume.
        pub mount_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Network features in use `Basic` or `Standard`.
        pub network_features: pulumi_wasm_rust::Output<String>,
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// A list of protocol types enabled on volume.
        pub protocols: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Volume security style
        pub security_style: pulumi_wasm_rust::Output<Option<String>>,
        /// The service level of the file system.
        pub service_level: pulumi_wasm_rust::Output<String>,
        /// Limits enumeration of files and folders (that is, listing the contents) in SMB only to users with allowed access on the share.
        pub smb_access_based_enumeration_enabled: pulumi_wasm_rust::Output<bool>,
        /// Limits clients from browsing for an SMB share.
        pub smb_non_browsable_enabled: pulumi_wasm_rust::Output<bool>,
        /// The maximum Storage Quota in Gigabytes allowed for a file system.
        pub storage_quota_in_gb: pulumi_wasm_rust::Output<i32>,
        /// The ID of a Subnet in which the NetApp Volume resides.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// The unique file path of the volume.
        pub volume_path: pulumi_wasm_rust::Output<String>,
        /// The Availability Zone in which the Volume is located.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVolumeArgs) -> GetVolumeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let name_binding = args.name.get_inner();
        let pool_name_binding = args.pool_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let security_style_binding = args.security_style.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:netapp/getVolume:getVolume".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "poolName".into(),
                    value: &pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "securityStyle".into(),
                    value: &security_style_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "dataProtectionBackupPolicies".into(),
                },
                register_interface::ResultField {
                    name: "dataProtectionReplications".into(),
                },
                register_interface::ResultField {
                    name: "encryptionKeySource".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultPrivateEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mountIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkFeatures".into(),
                },
                register_interface::ResultField {
                    name: "poolName".into(),
                },
                register_interface::ResultField {
                    name: "protocols".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "securityStyle".into(),
                },
                register_interface::ResultField {
                    name: "serviceLevel".into(),
                },
                register_interface::ResultField {
                    name: "smbAccessBasedEnumerationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "smbNonBrowsableEnabled".into(),
                },
                register_interface::ResultField {
                    name: "storageQuotaInGb".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "volumePath".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVolumeResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            data_protection_backup_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataProtectionBackupPolicies").unwrap(),
            ),
            data_protection_replications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataProtectionReplications").unwrap(),
            ),
            encryption_key_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionKeySource").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_vault_private_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultPrivateEndpointId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mount_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mountIpAddresses").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkFeatures").unwrap(),
            ),
            pool_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolName").unwrap(),
            ),
            protocols: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocols").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            security_style: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityStyle").unwrap(),
            ),
            service_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceLevel").unwrap(),
            ),
            smb_access_based_enumeration_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("smbAccessBasedEnumerationEnabled").unwrap(),
            ),
            smb_non_browsable_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("smbNonBrowsableEnabled").unwrap(),
            ),
            storage_quota_in_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageQuotaInGb").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            volume_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumePath").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}