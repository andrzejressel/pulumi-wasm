#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAppDapr {
    /// The Dapr Application Identifier.
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: Box<String>,
    /// The port which the application is listening on. This is the same as the `ingress` port.
    #[builder(into)]
    #[serde(rename = "appPort")]
    pub r#app_port: Box<i32>,
    /// The protocol for the app. Possible values include `http` and `grpc`. Defaults to `http`.
    #[builder(into)]
    #[serde(rename = "appProtocol")]
    pub r#app_protocol: Box<String>,
}
