#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZeroTrustDevicePostureRuleInput {
    /// The number of active threats from SentinelOne.
    #[builder(into, default)]
    #[serde(rename = "activeThreats")]
    pub r#active_threats: Box<Option<i32>>,
    /// The UUID of a Cloudflare managed certificate.
    #[builder(into, default)]
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Box<Option<String>>,
    /// Specific volume(s) to check for encryption.
    #[builder(into, default)]
    #[serde(rename = "checkDisks")]
    pub r#check_disks: Box<Option<Vec<String>>>,
    /// Confirm the certificate was not imported from another device.
    #[builder(into, default)]
    #[serde(rename = "checkPrivateKey")]
    pub r#check_private_key: Box<Option<bool>>,
    /// The common name for a certificate.
    #[builder(into, default)]
    #[serde(rename = "cn")]
    pub r#cn: Box<Option<String>>,
    /// The workspace one or intune device compliance status. `compliant` and `noncompliant` are values supported by both providers. `unknown`, `conflict`, `error`, `ingraceperiod` values are only supported by intune. Available values: `compliant`, `noncompliant`, `unknown`, `conflict`, `error`, `ingraceperiod`.
    #[builder(into, default)]
    #[serde(rename = "complianceStatus")]
    pub r#compliance_status: Box<Option<String>>,
    /// The workspace one or intune connection id.
    #[builder(into, default)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: Box<Option<String>>,
    /// The count comparison operator for kolide. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[builder(into, default)]
    #[serde(rename = "countOperator")]
    pub r#count_operator: Box<Option<String>>,
    /// The domain that the client must join.
    #[builder(into, default)]
    #[serde(rename = "domain")]
    pub r#domain: Box<Option<String>>,
    /// The time a device last seen in Tanium. Must be in the format `1h` or `30m`. Valid units are `d`, `h` and `m`.
    #[builder(into, default)]
    #[serde(rename = "eidLastSeen")]
    pub r#eid_last_seen: Box<Option<String>>,
    /// True if the firewall must be enabled.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Checks if the file should exist.
    #[builder(into, default)]
    #[serde(rename = "exists")]
    pub r#exists: Box<Option<bool>>,
    /// List of values indicating purposes for which the certificate public key can be used. Available values: `clientAuth`, `emailProtection`.
    #[builder(into, default)]
    #[serde(rename = "extendedKeyUsages")]
    pub r#extended_key_usages: Box<Option<Vec<String>>>,
    /// The Teams List id. Required for `serial_number` and `unique_client_id` rule types.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// True if SentinelOne device is infected.
    #[builder(into, default)]
    #[serde(rename = "infected")]
    pub r#infected: Box<Option<bool>>,
    /// True if SentinelOne device is active.
    #[builder(into, default)]
    #[serde(rename = "isActive")]
    pub r#is_active: Box<Option<bool>>,
    /// The number of issues for kolide.
    #[builder(into, default)]
    #[serde(rename = "issueCount")]
    pub r#issue_count: Box<Option<String>>,
    /// The duration of time that the host was last seen from Crowdstrike. Must be in the format `1h` or `30m`. Valid units are `d`, `h` and `m`.
    #[builder(into, default)]
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    /// List of operating system locations to check for a client certificate..
    #[builder(into, default)]
    #[serde(rename = "locations")]
    pub r#locations: Box<Option<Vec<super::types::ZeroTrustDevicePostureRuleInputLocation>>>,
    /// The network status from SentinelOne. Available values: `connected`, `disconnected`, `disconnecting`, `connecting`.
    #[builder(into, default)]
    #[serde(rename = "networkStatus")]
    pub r#network_status: Box<Option<String>>,
    /// The current operational state of a SentinelOne Agent. Available values: `na`, `partially_disabled`, `auto_fully_disabled`, `fully_disabled`, `auto_partially_disabled`, `disabled_error`, `db_corruption`.
    #[builder(into, default)]
    #[serde(rename = "operationalState")]
    pub r#operational_state: Box<Option<String>>,
    /// The version comparison operator. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[builder(into, default)]
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
    /// OS signal score from Crowdstrike. Value must be between 1 and 100.
    #[builder(into, default)]
    #[serde(rename = "os")]
    pub r#os: Box<Option<String>>,
    /// The operating system excluding version information.
    #[builder(into, default)]
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    /// The operating system version excluding OS name information or release name.
    #[builder(into, default)]
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    /// Extra version value following the operating system semantic version.
    #[builder(into, default)]
    #[serde(rename = "osVersionExtra")]
    pub r#os_version_extra: Box<Option<String>>,
    /// Overall ZTA score from Crowdstrike. Value must be between 1 and 100.
    #[builder(into, default)]
    #[serde(rename = "overall")]
    pub r#overall: Box<Option<String>>,
    /// The path to the file.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// True if all drives must be encrypted.
    #[builder(into, default)]
    #[serde(rename = "requireAll")]
    pub r#require_all: Box<Option<bool>>,
    /// The risk level from Tanium. Available values: `low`, `medium`, `high`, `critical`.
    #[builder(into, default)]
    #[serde(rename = "riskLevel")]
    pub r#risk_level: Box<Option<String>>,
    /// Checks if the application should be running.
    #[builder(into, default)]
    #[serde(rename = "running")]
    pub r#running: Box<Option<bool>>,
    /// A value between 0-100 assigned to devices set by the 3rd party posture provider for custom device posture integrations.
    #[builder(into, default)]
    #[serde(rename = "score")]
    pub r#score: Box<Option<i32>>,
    /// Sensor signal score from Crowdstrike. Value must be between 1 and 100.
    #[builder(into, default)]
    #[serde(rename = "sensorConfig")]
    pub r#sensor_config: Box<Option<String>>,
    /// The sha256 hash of the file.
    #[builder(into, default)]
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<Option<String>>,
    /// The host’s current online status from Crowdstrike. Available values: `online`, `offline`, `unknown`.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// The thumbprint of the file certificate.
    #[builder(into, default)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<Option<String>>,
    /// The total score from Tanium.
    #[builder(into, default)]
    #[serde(rename = "totalScore")]
    pub r#total_score: Box<Option<i32>>,
    /// The operating system semantic version.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
    /// The version comparison operator for Crowdstrike. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[builder(into, default)]
    #[serde(rename = "versionOperator")]
    pub r#version_operator: Box<Option<String>>,
}
