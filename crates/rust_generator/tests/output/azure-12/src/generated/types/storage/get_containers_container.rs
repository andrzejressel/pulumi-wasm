#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetContainersContainer {
    /// The data plane ID of the Storage Container.
    #[builder(into)]
    #[serde(rename = "dataPlaneId")]
    pub r#data_plane_id: Box<String>,
    /// The name of this Storage Container.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The resource manager ID of the Storage Container.
    #[builder(into)]
    #[serde(rename = "resourceManagerId")]
    pub r#resource_manager_id: Box<String>,
}
