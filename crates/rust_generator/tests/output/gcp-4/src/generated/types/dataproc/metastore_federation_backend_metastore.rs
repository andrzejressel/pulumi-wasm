#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetastoreFederationBackendMetastore {
    /// The type of the backend metastore.
    /// Possible values are: `METASTORE_TYPE_UNSPECIFIED`, `DATAPROC_METASTORE`, `BIGQUERY`.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "metastoreType")]
    pub r#metastore_type: Box<String>,
    /// The relative resource name of the metastore that is being federated. The formats of the relative resource names for the currently supported metastores are listed below: Dataplex: projects/{projectId}/locations/{location}/lakes/{lake_id} BigQuery: projects/{projectId} Dataproc Metastore: projects/{projectId}/locations/{location}/services/{serviceId}
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "rank")]
    pub r#rank: Box<String>,
}
