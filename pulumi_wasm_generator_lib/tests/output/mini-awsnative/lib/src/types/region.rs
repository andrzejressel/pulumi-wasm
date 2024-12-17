//! A Region represents any valid Amazon region that may be targeted with deployments.

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum Region {
    /// Africa (Cape Town)
    #[serde(rename = "af-south-1")]
    AfSouth1,
    /// Asia Pacific (Hong Kong)
    #[serde(rename = "ap-east-1")]
    ApEast1,
    /// Asia Pacific (Tokyo)
    #[serde(rename = "ap-northeast-1")]
    ApNortheast1,
    /// Asia Pacific (Seoul)
    #[serde(rename = "ap-northeast-2")]
    ApNortheast2,
    /// Asia Pacific (Osaka)
    #[serde(rename = "ap-northeast-3")]
    ApNortheast3,
    /// Asia Pacific (Mumbai)
    #[serde(rename = "ap-south-1")]
    ApSouth1,
    /// Asia Pacific (Singapore)
    #[serde(rename = "ap-southeast-1")]
    ApSoutheast1,
    /// Asia Pacific (Sydney)
    #[serde(rename = "ap-southeast-2")]
    ApSoutheast2,
    /// Canada (Central)
    #[serde(rename = "ca-central-1")]
    CaCentral,
    /// China (Beijing)
    #[serde(rename = "cn-north-1")]
    CnNorth1,
    /// China (Ningxia)
    #[serde(rename = "cn-northwest-1")]
    CnNorthwest1,
    /// Europe (Frankfurt)
    #[serde(rename = "eu-central-1")]
    EuCentral1,
    /// Europe (Stockholm)
    #[serde(rename = "eu-north-1")]
    EuNorth1,
    /// Europe (Ireland)
    #[serde(rename = "eu-west-1")]
    EuWest1,
    /// Europe (London)
    #[serde(rename = "eu-west-2")]
    EuWest2,
    /// Europe (Paris)
    #[serde(rename = "eu-west-3")]
    EuWest3,
    /// Europe (Milan)
    #[serde(rename = "eu-south-1")]
    EuSouth1,
    /// Middle East (Bahrain)
    #[serde(rename = "me-south-1")]
    MeSouth1,
    /// South America (São Paulo)
    #[serde(rename = "sa-east-1")]
    SaEast1,
    /// AWS GovCloud (US-East)
    #[serde(rename = "us-gov-east-1")]
    UsGovEast1,
    /// AWS GovCloud (US-West)
    #[serde(rename = "us-gov-west-1")]
    UsGovWest1,
    /// US East (N. Virginia)
    #[serde(rename = "us-east-1")]
    UsEast1,
    /// US East (Ohio)
    #[serde(rename = "us-east-2")]
    UsEast2,
    /// US West (N. California)
    #[serde(rename = "us-west-1")]
    UsWest1,
    /// US West (Oregon)
    #[serde(rename = "us-west-2")]
    UsWest2,
}
