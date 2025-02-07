#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciDeploymentSettingScaleUnit {
    /// Specify the full name of the Active Directory Organizational Unit container object prepared for the deployment, including the domain components. For example:`OU=HCI01,DC=contoso,DC=com`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "activeDirectoryOrganizationalUnitPath")]
    pub r#active_directory_organizational_unit_path: Box<String>,
    /// Whether to enable BitLocker for boot volume. Possible values are `true` and `false`. When set to `true`, BitLocker XTS_AES 256-bit encryption is enabled for all data-at-rest on the OS volume of your Azure Stack HCI cluster. This setting is TPM-hardware dependent. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "bitlockerBootVolumeEnabled")]
    pub r#bitlocker_boot_volume_enabled: Box<Option<bool>>,
    /// Whether to enable BitLocker for data volume. Possible values are `true` and `false`. When set to `true`, BitLocker XTS-AES 256-bit encryption is enabled for all data-at-rest on your Azure Stack HCI cluster shared volumes. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "bitlockerDataVolumeEnabled")]
    pub r#bitlocker_data_volume_enabled: Box<Option<bool>>,
    /// A `cluster` block as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "cluster")]
    pub r#cluster: Box<super::super::types::stack::HciDeploymentSettingScaleUnitCluster>,
    /// Whether to enable credential guard. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "credentialGuardEnabled")]
    pub r#credential_guard_enabled: Box<Option<bool>>,
    /// Specifies the FQDN for deploying cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "domainFqdn")]
    pub r#domain_fqdn: Box<String>,
    /// Whether to enable drift control. Possible values are `true` and `false`. When set to `true`, the security baseline is re-applied regularly. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "driftControlEnabled")]
    pub r#drift_control_enabled: Box<Option<bool>>,
    /// Whether to enable DRTM protection. Possible values are `true` and `false`. When set to `true`, Secure Boot is enabled on your Azure HCI cluster. This setting is hardware dependent. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "drtmProtectionEnabled")]
    pub r#drtm_protection_enabled: Box<Option<bool>>,
    /// Whether to collect log data to facilitate quicker issue resolution. Possible values are `true` and `false`. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "episodicDataUploadEnabled")]
    pub r#episodic_data_upload_enabled: Box<Option<bool>>,
    /// Whether to store data sent to Microsoft in EU. The log and diagnostic data is sent to the appropriate diagnostics servers depending upon where your cluster resides. Setting this to `false` results in all data sent to Microsoft to be stored outside of the EU. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "euLocationEnabled")]
    pub r#eu_location_enabled: Box<Option<bool>>,
    /// A `host_network` block as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "hostNetwork")]
    pub r#host_network: Box<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetwork>,
    /// Whether to enable HVCI protection. Possible values are `true` and `false`. When set to `true`, Hypervisor-protected Code Integrity is enabled on your Azure HCI cluster. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "hvciProtectionEnabled")]
    pub r#hvci_protection_enabled: Box<Option<bool>>,
    /// One or more `infrastructure_network` blocks as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "infrastructureNetworks")]
    pub r#infrastructure_networks: Box<Vec<super::super::types::stack::HciDeploymentSettingScaleUnitInfrastructureNetwork>>,
    /// Specifies the name prefix to deploy cluster. It must be 1-8 characters long and contain only letters, numbers and hyphens Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: Box<String>,
    /// A `optional_service` block as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "optionalService")]
    pub r#optional_service: Box<super::super::types::stack::HciDeploymentSettingScaleUnitOptionalService>,
    /// One or more `physical_node` blocks as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "physicalNodes")]
    pub r#physical_nodes: Box<Vec<super::super::types::stack::HciDeploymentSettingScaleUnitPhysicalNode>>,
    /// The URI to the Key Vault or secret store. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "secretsLocation")]
    pub r#secrets_location: Box<String>,
    /// Whether to enable side channel mitigation. Possible values are `true` and `false`. When set to `true`, all side channel mitigations are enabled on your Azure HCI cluster. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "sideChannelMitigationEnabled")]
    pub r#side_channel_mitigation_enabled: Box<Option<bool>>,
    /// Whether to enable SMB cluster encryption. Possible values are `true` and `false`. When set to `true`, cluster east-west traffic is encrypted. Defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "smbClusterEncryptionEnabled")]
    pub r#smb_cluster_encryption_enabled: Box<Option<bool>>,
    /// Whether to enable SMB signing. Possible values are `true` and `false`. When set to `true`, the SMB default instance requires sign in for the client and server services. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "smbSigningEnabled")]
    pub r#smb_signing_enabled: Box<Option<bool>>,
    /// A `storage` block as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "storage")]
    pub r#storage: Box<super::super::types::stack::HciDeploymentSettingScaleUnitStorage>,
    /// Whether the telemetry data will be sent to Microsoft. Possible values are `true` and `false`. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "streamingDataClientEnabled")]
    pub r#streaming_data_client_enabled: Box<Option<bool>>,
    /// Whether to enable WDAC. Possible values are `true` and `false`. When set to `true`, applications and the code that you can run on your Azure Stack HCI cluster are limited. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "wdacEnabled")]
    pub r#wdac_enabled: Box<Option<bool>>,
}
