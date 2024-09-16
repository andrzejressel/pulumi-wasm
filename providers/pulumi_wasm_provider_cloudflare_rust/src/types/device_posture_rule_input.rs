#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct DevicePostureRuleInput {
    /// The number of active threats from SentinelOne.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "activeThreats")]
    pub r#active_threats: Box<Option<i32>>,
    /// The UUID of a Cloudflare managed certificate.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Box<Option<String>>,
    /// Specific volume(s) to check for encryption.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "checkDisks")]
    pub r#check_disks: Box<Option<Vec<String>>>,
    /// The common name for a certificate.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "cn")]
    pub r#cn: Box<Option<String>>,
    /// The workspace one device compliance status. Available values: `compliant`, `noncompliant`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "complianceStatus")]
    pub r#compliance_status: Box<Option<String>>,
    /// The workspace one connection id.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "connectionId")]
    pub r#connection_id: Box<Option<String>>,
    /// The count comparison operator for kolide. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "countOperator")]
    pub r#count_operator: Box<Option<String>>,
    /// The domain that the client must join.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "domain")]
    pub r#domain: Box<Option<String>>,
    /// The datetime a device last seen in RFC 3339 format from Tanium.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "eidLastSeen")]
    pub r#eid_last_seen: Box<Option<String>>,
    /// True if the firewall must be enabled.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Checks if the file should exist.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "exists")]
    pub r#exists: Box<Option<bool>>,
    /// The Teams List id. Required for `serial_number` and `unique_client_id` rule types.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// True if SentinelOne device is infected.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "infected")]
    pub r#infected: Box<Option<bool>>,
    /// True if SentinelOne device is active.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "isActive")]
    pub r#is_active: Box<Option<bool>>,
    /// The number of issues for kolide.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "issueCount")]
    pub r#issue_count: Box<Option<String>>,
    /// The duration of time that the host was last seen from Crowdstrike. Must be in the format `1h` or `30m`. Valid units are `d`, `h` and `m`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    /// The network status from SentinelOne. Available values: `connected`, `disconnected`, `disconnecting`, `connecting`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "networkStatus")]
    pub r#network_status: Box<Option<String>>,
    /// The version comparison operator. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
    /// OS signal score from Crowdstrike. Value must be between 1 and 100.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "os")]
    pub r#os: Box<Option<String>>,
    /// The operating system excluding version information.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    /// The operating system version excluding OS name information or release name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    /// Overall ZTA score from Crowdstrike. Value must be between 1 and 100.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "overall")]
    pub r#overall: Box<Option<String>>,
    /// The path to the file.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// True if all drives must be encrypted.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "requireAll")]
    pub r#require_all: Box<Option<bool>>,
    /// The risk level from Tanium. Available values: `low`, `medium`, `high`, `critical`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "riskLevel")]
    pub r#risk_level: Box<Option<String>>,
    /// Checks if the application should be running.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "running")]
    pub r#running: Box<Option<bool>>,
    /// Sensor signal score from Crowdstrike. Value must be between 1 and 100.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "sensorConfig")]
    pub r#sensor_config: Box<Option<String>>,
    /// The sha256 hash of the file.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<Option<String>>,
    /// The host’s current online status from Crowdstrike. Available values: `online`, `offline`, `unknown`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// The thumbprint of the file certificate.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<Option<String>>,
    /// The total score from Tanium.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "totalScore")]
    pub r#total_score: Box<Option<i32>>,
    /// The operating system semantic version.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
    /// The version comparison operator for crowdstrike. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "versionOperator")]
    pub r#version_operator: Box<Option<String>>,
}
