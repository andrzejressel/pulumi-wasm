#[derive(serde::Serialize)]
pub struct DevicePostureRuleInput {
    /// The number of active threats from SentinelOne.
    #[serde(rename = "activeThreats")]
    pub r#active_threats: Box<Option<i32>>,
    /// The UUID of a Cloudflare managed certificate.
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Box<Option<String>>,
    /// Specific volume(s) to check for encryption.
    #[serde(rename = "checkDisks")]
    pub r#check_disks: Box<Option<Vec<String>>>,
    /// The common name for a certificate.
    #[serde(rename = "cn")]
    pub r#cn: Box<Option<String>>,
    /// The workspace one device compliance status. Available values: `compliant`, `noncompliant`.
    #[serde(rename = "complianceStatus")]
    pub r#compliance_status: Box<Option<String>>,
    /// The workspace one connection id.
    #[serde(rename = "connectionId")]
    pub r#connection_id: Box<Option<String>>,
    /// The count comparison operator for kolide. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[serde(rename = "countOperator")]
    pub r#count_operator: Box<Option<String>>,
    /// The domain that the client must join.
    #[serde(rename = "domain")]
    pub r#domain: Box<Option<String>>,
    /// The datetime a device last seen in RFC 3339 format from Tanium.
    #[serde(rename = "eidLastSeen")]
    pub r#eid_last_seen: Box<Option<String>>,
    /// True if the firewall must be enabled.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Checks if the file should exist.
    #[serde(rename = "exists")]
    pub r#exists: Box<Option<bool>>,
    /// The Teams List id. Required for `serial_number` and `unique_client_id` rule types.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// True if SentinelOne device is infected.
    #[serde(rename = "infected")]
    pub r#infected: Box<Option<bool>>,
    /// True if SentinelOne device is active.
    #[serde(rename = "isActive")]
    pub r#is_active: Box<Option<bool>>,
    /// The number of issues for kolide.
    #[serde(rename = "issueCount")]
    pub r#issue_count: Box<Option<String>>,
    /// The duration of time that the host was last seen from Crowdstrike. Must be in the format `1h` or `30m`. Valid units are `d`, `h` and `m`.
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    /// The network status from SentinelOne. Available values: `connected`, `disconnected`, `disconnecting`, `connecting`.
    #[serde(rename = "networkStatus")]
    pub r#network_status: Box<Option<String>>,
    /// The version comparison operator. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
    /// OS signal score from Crowdstrike. Value must be between 1 and 100.
    #[serde(rename = "os")]
    pub r#os: Box<Option<String>>,
    /// The operating system excluding version information.
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    /// The operating system version excluding OS name information or release name.
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    /// Overall ZTA score from Crowdstrike. Value must be between 1 and 100.
    #[serde(rename = "overall")]
    pub r#overall: Box<Option<String>>,
    /// The path to the file.
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// True if all drives must be encrypted.
    #[serde(rename = "requireAll")]
    pub r#require_all: Box<Option<bool>>,
    /// The risk level from Tanium. Available values: `low`, `medium`, `high`, `critical`.
    #[serde(rename = "riskLevel")]
    pub r#risk_level: Box<Option<String>>,
    /// Checks if the application should be running.
    #[serde(rename = "running")]
    pub r#running: Box<Option<bool>>,
    /// Sensor signal score from Crowdstrike. Value must be between 1 and 100.
    #[serde(rename = "sensorConfig")]
    pub r#sensor_config: Box<Option<String>>,
    /// The sha256 hash of the file.
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<Option<String>>,
    /// The host’s current online status from Crowdstrike. Available values: `online`, `offline`, `unknown`.
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// The thumbprint of the file certificate.
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<Option<String>>,
    /// The total score from Tanium.
    #[serde(rename = "totalScore")]
    pub r#total_score: Box<Option<i32>>,
    /// The operating system semantic version.
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
    /// The version comparison operator for crowdstrike. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[serde(rename = "versionOperator")]
    pub r#version_operator: Box<Option<String>>,
}
