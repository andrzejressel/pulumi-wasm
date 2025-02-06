#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNetworkGatewayVpnClientConfigurationIpsecPolicy {
    /// The DH Group, used in IKE Phase 1. Possible values are `DHGroup1`, `DHGroup2`, `DHGroup14`, `DHGroup24`, `DHGroup2048`, `ECP256`, `ECP384` and `None`.
    #[builder(into)]
    #[serde(rename = "dhGroup")]
    pub r#dh_group: Box<String>,
    /// The IKE encryption algorithm, used for IKE Phase 2. Possible values are `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128` and `GCMAES256`.
    #[builder(into)]
    #[serde(rename = "ikeEncryption")]
    pub r#ike_encryption: Box<String>,
    /// The IKE encryption integrity algorithm, used for IKE Phase 2. Possible values are `GCMAES128`, `GCMAES256`, `MD5`, `SHA1`, `SHA256` and `SHA384`.
    #[builder(into)]
    #[serde(rename = "ikeIntegrity")]
    pub r#ike_integrity: Box<String>,
    /// The IPSec encryption algorithm, used for IKE phase 1. Possible values are `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128`, `GCMAES192`, `GCMAES256` and `None`.
    #[builder(into)]
    #[serde(rename = "ipsecEncryption")]
    pub r#ipsec_encryption: Box<String>,
    /// The IPSec integrity algorithm, used for IKE phase 1. Possible values are `GCMAES128`, `GCMAES192`, `GCMAES256`, `MD5`, `SHA1` and `SHA256`.
    #[builder(into)]
    #[serde(rename = "ipsecIntegrity")]
    pub r#ipsec_integrity: Box<String>,
    /// The Pfs Group, used in IKE Phase 2. Possible values are `ECP256`, `ECP384`, `PFS1`, `PFS2`, `PFS14`, `PFS24`, `PFS2048`, `PFSMM` and `None`.
    #[builder(into)]
    #[serde(rename = "pfsGroup")]
    pub r#pfs_group: Box<String>,
    /// The IPSec Security Association payload size in KB for a Site-to-Site VPN tunnel. Possible values are between `1024` and `2147483647`.
    #[builder(into)]
    #[serde(rename = "saDataSizeInKilobytes")]
    pub r#sa_data_size_in_kilobytes: Box<i32>,
    /// The IPSec Security Association lifetime in seconds for a Site-to-Site VPN tunnel. Possible values are between `300` and `172799`.
    #[builder(into)]
    #[serde(rename = "saLifetimeInSeconds")]
    pub r#sa_lifetime_in_seconds: Box<i32>,
}
