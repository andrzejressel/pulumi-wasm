#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum EngineType {
    #[serde(rename = "aurora")]
    Aurora,
    #[serde(rename = "aurora-mysql")]
    AuroraMysql,
    #[serde(rename = "aurora-postgresql")]
    AuroraPostgresql,
}