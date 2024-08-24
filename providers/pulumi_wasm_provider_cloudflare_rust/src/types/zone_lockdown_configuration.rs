#[derive(serde::Serialize)]
pub struct ZoneLockdownConfiguration {
    /// The request property to target. Available values: `ip`, `ip_range`.
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// The value to target. Depends on target's type. IP addresses should just be standard IPv4/IPv6 notation i.e. `192.0.2.1` or `2001:db8::/32` and IP ranges in CIDR format i.e. `192.0.2.0/24`.
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
