#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceConfigurationWebCrawlerConfigurationProxyConfiguration {
    /// Your secret ARN, which you can create in AWS Secrets Manager. The credentials are optional. You use a secret if web proxy credentials are required to connect to a website host. Amazon Kendra currently support basic authentication to connect to a web proxy server. The secret stores your credentials.
    #[builder(into, default)]
    #[serde(rename = "credentials")]
    pub r#credentials: Box<Option<String>>,
    /// The name of the website host you want to connect to via a web proxy server. For example, the host name of `https://a.example.com/page1.html` is `"a.example.com"`.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The port number of the website host you want to connect to via a web proxy server. For example, the port for `https://a.example.com/page1.html` is `443`, the standard port for HTTPS.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
