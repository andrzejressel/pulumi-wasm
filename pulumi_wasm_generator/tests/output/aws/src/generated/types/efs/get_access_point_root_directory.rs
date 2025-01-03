#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAccessPointRootDirectory {
    /// Single element list containing information on the creation permissions of the directory
    #[builder(into)]
    #[serde(rename = "creationInfos")]
    pub r#creation_infos: Box<Vec<super::super::types::efs::GetAccessPointRootDirectoryCreationInfo>>,
    /// Path exposed as the root directory
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
