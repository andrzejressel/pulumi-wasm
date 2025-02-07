#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiImportWsdlSelector {
    /// The name of endpoint (port) to import from WSDL.
    #[builder(into)]
    #[serde(rename = "endpointName")]
    pub r#endpoint_name: Box<String>,
    /// The name of service to import from WSDL.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<String>,
}
