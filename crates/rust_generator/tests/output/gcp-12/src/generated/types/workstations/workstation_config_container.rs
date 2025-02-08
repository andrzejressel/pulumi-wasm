#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkstationConfigContainer {
    /// Arguments passed to the entrypoint.
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// If set, overrides the default ENTRYPOINT specified by the image.
    #[builder(into, default)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    /// Environment variables passed to the container.
    /// The elements are of the form "KEY=VALUE" for the environment variable "KEY" being given the value "VALUE".
    #[builder(into, default)]
    #[serde(rename = "env")]
    pub r#env: Box<Option<std::collections::HashMap<String, String>>>,
    /// Docker image defining the container. This image must be accessible by the config's service account.
    #[builder(into, default)]
    #[serde(rename = "image")]
    pub r#image: Box<Option<String>>,
    /// If set, overrides the USER specified in the image with the given uid.
    #[builder(into, default)]
    #[serde(rename = "runAsUser")]
    pub r#run_as_user: Box<Option<i32>>,
    /// If set, overrides the default DIR specified by the image.
    #[builder(into, default)]
    #[serde(rename = "workingDir")]
    pub r#working_dir: Box<Option<String>>,
}
