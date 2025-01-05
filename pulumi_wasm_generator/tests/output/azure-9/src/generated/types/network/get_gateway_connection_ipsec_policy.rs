#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGatewayConnectionIpsecPolicy {
    /// The DH group used in IKE phase 1 for initial SA. Valid
    /// options are `DHGroup1`, `DHGroup14`, `DHGroup2`, `DHGroup2048`, `DHGroup24`,
    /// `ECP256`, `ECP384`, or `None`.
    #[builder(into)]
    #[serde(rename = "dhGroup")]
    pub r#dh_group: Box<String>,
    /// The IKE encryption algorithm. Valid
    /// options are `AES128`, `AES192`, `AES256`, `DES`, or `DES3`.
    #[builder(into)]
    #[serde(rename = "ikeEncryption")]
    pub r#ike_encryption: Box<String>,
    /// The IKE integrity algorithm. Valid
    /// options are `MD5`, `SHA1`, `SHA256`, or `SHA384`.
    #[builder(into)]
    #[serde(rename = "ikeIntegrity")]
    pub r#ike_integrity: Box<String>,
    /// The IPSec encryption algorithm. Valid
    /// options are `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128`, `GCMAES192`, `GCMAES256`, or `None`.
    #[builder(into)]
    #[serde(rename = "ipsecEncryption")]
    pub r#ipsec_encryption: Box<String>,
    /// The IPSec integrity algorithm. Valid
    /// options are `GCMAES128`, `GCMAES192`, `GCMAES256`, `MD5`, `SHA1`, or `SHA256`.
    #[builder(into)]
    #[serde(rename = "ipsecIntegrity")]
    pub r#ipsec_integrity: Box<String>,
    /// The DH group used in IKE phase 2 for new child SA.
    /// Valid options are `ECP256`, `ECP384`, `PFS1`, `PFS2`, `PFS2048`, `PFS24`,
    /// or `None`.
    #[builder(into)]
    #[serde(rename = "pfsGroup")]
    pub r#pfs_group: Box<String>,
    /// The IPSec SA payload size in KB. Must be at least
    /// `1024` KB.
    #[builder(into)]
    #[serde(rename = "saDatasize")]
    pub r#sa_datasize: Box<i32>,
    /// The IPSec SA lifetime in seconds. Must be at least
    /// `300` seconds.
    #[builder(into)]
    #[serde(rename = "saLifetime")]
    pub r#sa_lifetime: Box<i32>,
}
