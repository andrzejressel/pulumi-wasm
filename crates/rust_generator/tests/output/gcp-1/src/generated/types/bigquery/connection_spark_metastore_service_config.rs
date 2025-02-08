#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionSparkMetastoreServiceConfig {
    /// Resource name of an existing Dataproc Metastore service in the form of projects/[projectId]/locations/[region]/services/[serviceId].
    #[builder(into, default)]
    #[serde(rename = "metastoreService")]
    pub r#metastore_service: Box<Option<String>>,
}
