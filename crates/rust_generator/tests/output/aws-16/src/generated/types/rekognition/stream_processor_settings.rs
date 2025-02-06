#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamProcessorSettings {
    /// Label detection settings to use on a streaming video. See `connected_home`.
    #[builder(into, default)]
    #[serde(rename = "connectedHome")]
    pub r#connected_home: Box<Option<super::super::types::rekognition::StreamProcessorSettingsConnectedHome>>,
    /// Input face recognition parameters for an Amazon Rekognition stream processor. See `face_search`.
    #[builder(into, default)]
    #[serde(rename = "faceSearch")]
    pub r#face_search: Box<Option<super::super::types::rekognition::StreamProcessorSettingsFaceSearch>>,
}
