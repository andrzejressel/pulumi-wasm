#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ReservationShareSettingsProjectMap {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The project id/number, should be same as the key of this project config in the project map.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<String>>,
}
