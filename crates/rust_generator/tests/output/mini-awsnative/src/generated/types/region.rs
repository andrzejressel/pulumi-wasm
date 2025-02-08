#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub enum Region {
    /// Africa (Cape Town)
    #[serde(rename = "af-south-1")]
    AFSouth1,
    /// Asia Pacific (Hong Kong)
    #[serde(rename = "ap-east-1")]
    APEast1,
    /// Asia Pacific (Tokyo)
    #[serde(rename = "ap-northeast-1")]
    APNortheast1,
    /// Asia Pacific (Seoul)
    #[serde(rename = "ap-northeast-2")]
    APNortheast2,
    /// Asia Pacific (Osaka)
    #[serde(rename = "ap-northeast-3")]
    APNortheast3,
    /// Asia Pacific (Mumbai)
    #[serde(rename = "ap-south-1")]
    APSouth1,
    /// Asia Pacific (Singapore)
    #[serde(rename = "ap-southeast-1")]
    APSoutheast1,
    /// Asia Pacific (Sydney)
    #[serde(rename = "ap-southeast-2")]
    APSoutheast2,
    /// Canada (Central)
    #[serde(rename = "ca-central-1")]
    CACentral,
    /// China (Beijing)
    #[serde(rename = "cn-north-1")]
    CNNorth1,
    /// China (Ningxia)
    #[serde(rename = "cn-northwest-1")]
    CNNorthwest1,
    /// Europe (Frankfurt)
    #[serde(rename = "eu-central-1")]
    EUCentral1,
    /// Europe (Stockholm)
    #[serde(rename = "eu-north-1")]
    EUNorth1,
    /// Europe (Ireland)
    #[serde(rename = "eu-west-1")]
    EUWest1,
    /// Europe (London)
    #[serde(rename = "eu-west-2")]
    EUWest2,
    /// Europe (Paris)
    #[serde(rename = "eu-west-3")]
    EUWest3,
    /// Europe (Milan)
    #[serde(rename = "eu-south-1")]
    EUSouth1,
    /// Middle East (Bahrain)
    #[serde(rename = "me-south-1")]
    MESouth1,
    /// South America (São Paulo)
    #[serde(rename = "sa-east-1")]
    SAEast1,
    /// AWS GovCloud (US-East)
    #[serde(rename = "us-gov-east-1")]
    USGovEast1,
    /// AWS GovCloud (US-West)
    #[serde(rename = "us-gov-west-1")]
    USGovWest1,
    /// US East (N. Virginia)
    #[serde(rename = "us-east-1")]
    USEast1,
    /// US East (Ohio)
    #[serde(rename = "us-east-2")]
    USEast2,
    /// US West (N. California)
    #[serde(rename = "us-west-1")]
    USWest1,
    /// US West (Oregon)
    #[serde(rename = "us-west-2")]
    USWest2,
}
