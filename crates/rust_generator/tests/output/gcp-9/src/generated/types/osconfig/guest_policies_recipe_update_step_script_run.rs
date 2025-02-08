#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GuestPoliciesRecipeUpdateStepScriptRun {
    /// Return codes that indicate that the software installed or updated successfully. Behaviour defaults to [0]
    #[builder(into, default)]
    #[serde(rename = "allowedExitCodes")]
    pub r#allowed_exit_codes: Box<Option<Vec<i32>>>,
    /// The script interpreter to use to run the script. If no interpreter is specified the script is executed directly,
    /// which likely only succeed for scripts with shebang lines.
    /// Possible values are: `SHELL`, `POWERSHELL`.
    #[builder(into, default)]
    #[serde(rename = "interpreter")]
    pub r#interpreter: Box<Option<String>>,
    /// The shell script to be executed.
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Box<String>,
}
