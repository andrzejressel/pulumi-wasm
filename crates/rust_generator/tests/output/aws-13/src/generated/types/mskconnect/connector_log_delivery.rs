#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectorLogDelivery {
    /// The workers can send worker logs to different destination types. This configuration specifies the details of these destinations. See `worker_log_delivery` Block for details.
    #[builder(into)]
    #[serde(rename = "workerLogDelivery")]
    pub r#worker_log_delivery: Box<super::super::types::mskconnect::ConnectorLogDeliveryWorkerLogDelivery>,
}
