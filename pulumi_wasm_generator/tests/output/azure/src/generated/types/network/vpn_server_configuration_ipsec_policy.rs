#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnServerConfigurationIpsecPolicy {
    /// The DH Group, used in IKE Phase 1. Possible values include `DHGroup1`, `DHGroup2`, `DHGroup14`, `DHGroup24`, `DHGroup2048`, `ECP256`, `ECP384` and `None`.
    #[builder(into)]
    #[serde(rename = "dhGroup")]
    pub r#dh_group: Box<String>,
    /// The IKE encryption algorithm, used for IKE Phase 2. Possible values include `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128` and `GCMAES256`.
    #[builder(into)]
    #[serde(rename = "ikeEncryption")]
    pub r#ike_encryption: Box<String>,
    /// The IKE encryption integrity algorithm, used for IKE Phase 2. Possible values include `GCMAES128`, `GCMAES256`, `MD5`, `SHA1`, `SHA256` and `SHA384`.
    #[builder(into)]
    #[serde(rename = "ikeIntegrity")]
    pub r#ike_integrity: Box<String>,
    /// The IPSec encryption algorithm, used for IKE phase 1. Possible values include `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128`, `GCMAES192`, `GCMAES256` and `None`.
    #[builder(into)]
    #[serde(rename = "ipsecEncryption")]
    pub r#ipsec_encryption: Box<String>,
    /// The IPSec integrity algorithm, used for IKE phase 1. Possible values include `GCMAES128`, `GCMAES192`, `GCMAES256`, `MD5`, `SHA1` and `SHA256`.
    #[builder(into)]
    #[serde(rename = "ipsecIntegrity")]
    pub r#ipsec_integrity: Box<String>,
    /// The Pfs Group, used in IKE Phase 2. Possible values include `ECP256`, `ECP384`, `PFS1`, `PFS2`, `PFS14`, `PFS24`, `PFS2048`, `PFSMM` and `None`.
    #[builder(into)]
    #[serde(rename = "pfsGroup")]
    pub r#pfs_group: Box<String>,
    /// The IPSec Security Association payload size in KB for a Site-to-Site VPN tunnel.
    #[builder(into)]
    #[serde(rename = "saDataSizeKilobytes")]
    pub r#sa_data_size_kilobytes: Box<i32>,
    /// The IPSec Security Association lifetime in seconds for a Site-to-Site VPN tunnel.
    #[builder(into)]
    #[serde(rename = "saLifetimeSeconds")]
    pub r#sa_lifetime_seconds: Box<i32>,
}
