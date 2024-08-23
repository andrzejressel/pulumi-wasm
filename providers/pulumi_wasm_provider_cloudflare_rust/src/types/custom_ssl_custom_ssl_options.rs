#[derive(serde::Serialize)]
pub struct CustomSslCustomSslOptions {
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Box<Option<String>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<String>>,
    #[serde(rename = "geoRestrictions")]
    pub r#geo_restrictions: Box<Option<String>>,
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
