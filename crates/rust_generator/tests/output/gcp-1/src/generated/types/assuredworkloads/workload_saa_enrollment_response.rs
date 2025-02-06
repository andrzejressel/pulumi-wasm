#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkloadSaaEnrollmentResponse {
    /// Indicates SAA enrollment setup error if any.
    #[builder(into, default)]
    #[serde(rename = "setupErrors")]
    pub r#setup_errors: Box<Option<Vec<String>>>,
    /// Indicates SAA enrollment status of a given workload. Possible values: SETUP_STATE_UNSPECIFIED, STATUS_PENDING, STATUS_COMPLETE
    #[builder(into, default)]
    #[serde(rename = "setupStatus")]
    pub r#setup_status: Box<Option<String>>,
}
