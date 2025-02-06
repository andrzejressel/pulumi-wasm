#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationRuntimeSsisExpressCustomSetup {
    /// One or more `command_key` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "commandKeys")]
    pub r#command_keys: Box<Option<Vec<super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetupCommandKey>>>,
    /// One or more `component` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "components")]
    pub r#components: Box<Option<Vec<super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetupComponent>>>,
    /// The Environment Variables for the Azure-SSIS Integration Runtime.
    #[builder(into, default)]
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<std::collections::HashMap<String, String>>>,
    /// The version of Azure Powershell installed for the Azure-SSIS Integration Runtime.
    /// 
    /// > **NOTE** At least one of `env`, `powershell_version`, `component` and `command_key` should be specified.
    #[builder(into, default)]
    #[serde(rename = "powershellVersion")]
    pub r#powershell_version: Box<Option<String>>,
}
