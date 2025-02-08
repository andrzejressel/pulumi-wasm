#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VpnGatewayConnectionVpnLinkIpsecPolicy {
    /// The DH Group used in IKE Phase 1 for initial SA. Possible values are `None`, `DHGroup1`, `DHGroup2`, `DHGroup14`, `DHGroup24`, `DHGroup2048`, `ECP256`, `ECP384`.
    #[builder(into)]
    #[serde(rename = "dhGroup")]
    pub r#dh_group: Box<String>,
    /// The IPSec encryption algorithm (IKE phase 1). Possible values are `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128`, `GCMAES192`, `GCMAES256`, `None`.
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Box<String>,
    /// The IKE encryption algorithm (IKE phase 2). Possible values are `DES`, `DES3`, `AES128`, `AES192`, `AES256`, `GCMAES128`, `GCMAES256`.
    #[builder(into)]
    #[serde(rename = "ikeEncryptionAlgorithm")]
    pub r#ike_encryption_algorithm: Box<String>,
    /// The IKE integrity algorithm (IKE phase 2). Possible values are `MD5`, `SHA1`, `SHA256`, `SHA384`, `GCMAES128`, `GCMAES256`.
    #[builder(into)]
    #[serde(rename = "ikeIntegrityAlgorithm")]
    pub r#ike_integrity_algorithm: Box<String>,
    /// The IPSec integrity algorithm (IKE phase 1). Possible values are `MD5`, `SHA1`, `SHA256`, `GCMAES128`, `GCMAES192`, `GCMAES256`.
    #[builder(into)]
    #[serde(rename = "integrityAlgorithm")]
    pub r#integrity_algorithm: Box<String>,
    /// The Pfs Group used in IKE Phase 2 for the new child SA. Possible values are `None`, `PFS1`, `PFS2`, `PFS14`, `PFS24`, `PFS2048`, `PFSMM`, `ECP256`, `ECP384`.
    #[builder(into)]
    #[serde(rename = "pfsGroup")]
    pub r#pfs_group: Box<String>,
    /// The IPSec Security Association (also called Quick Mode or Phase 2 SA) payload size in KB for the site to site VPN tunnel.
    #[builder(into)]
    #[serde(rename = "saDataSizeKb")]
    pub r#sa_data_size_kb: Box<i32>,
    /// The IPSec Security Association (also called Quick Mode or Phase 2 SA) lifetime in seconds for the site to site VPN tunnel.
    #[builder(into)]
    #[serde(rename = "saLifetimeSec")]
    pub r#sa_lifetime_sec: Box<i32>,
}
