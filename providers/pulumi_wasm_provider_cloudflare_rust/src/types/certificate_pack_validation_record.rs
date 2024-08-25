#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CertificatePackValidationRecord {
    #[serde(rename = "cnameName")]
    pub r#cname_name: Box<Option<String>>,
    #[serde(rename = "cnameTarget")]
    pub r#cname_target: Box<Option<String>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "httpBody")]
    pub r#http_body: Box<Option<String>>,
    #[serde(rename = "httpUrl")]
    pub r#http_url: Box<Option<String>>,
    #[serde(rename = "txtName")]
    pub r#txt_name: Box<Option<String>>,
    #[serde(rename = "txtValue")]
    pub r#txt_value: Box<Option<String>>,
}
