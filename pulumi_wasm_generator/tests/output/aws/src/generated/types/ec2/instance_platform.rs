#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum InstancePlatform {
    #[serde(rename = "Linux/UNIX")]
    LinuxUnix,
    #[serde(rename = "Red Hat Enterprise Linux")]
    RedHatEnterpriseLinux,
    #[serde(rename = "SUSE Linux")]
    SuseLinux,
    Windows,
    #[serde(rename = "Windows with SQL Server")]
    WindowsWithSqlServer,
    #[serde(rename = "Windows with SQL Server Enterprise")]
    WindowsWithSqlServerEnterprise,
    #[serde(rename = "Windows with SQL Server Standard")]
    WindowsWithSqlServerStandard,
    #[serde(rename = "Windows with SQL Server Web")]
    WindowsWithSqlServerWeb,
}
