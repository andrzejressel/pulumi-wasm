#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct CertificatePackValidationRecord {
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "cnameName")]
    pub r#cname_name: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "cnameTarget")]
    pub r#cname_target: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "httpBody")]
    pub r#http_body: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "httpUrl")]
    pub r#http_url: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "txtName")]
    pub r#txt_name: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "txtValue")]
    pub r#txt_value: Box<Option<String>>,
}
