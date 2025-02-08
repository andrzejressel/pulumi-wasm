#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceConfigurationWebCrawlerConfigurationAuthenticationConfigurationBasicAuthentication {
    /// Your secret ARN, which you can create in AWS Secrets Manager. You use a secret if basic authentication credentials are required to connect to a website. The secret stores your credentials of user name and password.
    #[builder(into)]
    #[serde(rename = "credentials")]
    pub r#credentials: Box<String>,
    /// The name of the website host you want to connect to using authentication credentials. For example, the host name of `https://a.example.com/page1.html` is `"a.example.com"`.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The port number of the website host you want to connect to using authentication credentials. For example, the port for `https://a.example.com/page1.html` is `443`, the standard port for HTTPS.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
