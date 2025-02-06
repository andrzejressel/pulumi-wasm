#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateTemplatePassthroughExtensionsAdditionalExtension {
    /// Required. The parts of an OID path. The most significant parts of the path come first.
    #[builder(into)]
    #[serde(rename = "objectIdPaths")]
    pub r#object_id_paths: Box<Vec<i32>>,
}
