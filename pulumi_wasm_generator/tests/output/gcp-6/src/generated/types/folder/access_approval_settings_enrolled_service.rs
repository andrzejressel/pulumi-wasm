#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessApprovalSettingsEnrolledService {
    /// The product for which Access Approval will be enrolled. Allowed values are listed (case-sensitive):
    /// * all
    /// * App Engine
    /// * BigQuery
    /// * Cloud Bigtable
    /// * Cloud Key Management Service
    /// * Compute Engine
    /// * Cloud Dataflow
    /// * Cloud Identity and Access Management
    /// * Cloud Pub/Sub
    /// * Cloud Storage
    /// * Persistent Disk
    /// Note: These values are supported as input, but considered a legacy format:
    /// * all
    /// * appengine.googleapis.com
    /// * bigquery.googleapis.com
    /// * bigtable.googleapis.com
    /// * cloudkms.googleapis.com
    /// * compute.googleapis.com
    /// * dataflow.googleapis.com
    /// * iam.googleapis.com
    /// * pubsub.googleapis.com
    /// * storage.googleapis.com
    #[builder(into)]
    #[serde(rename = "cloudProduct")]
    pub r#cloud_product: Box<String>,
    /// The enrollment level of the service.
    /// Default value is `BLOCK_ALL`.
    /// Possible values are: `BLOCK_ALL`.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "enrollmentLevel")]
    pub r#enrollment_level: Box<Option<String>>,
}
