#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum InstanceType {
    #[serde(rename = "db.t4g.micro")]
    T4G_Micro,
    #[serde(rename = "db.t4g.small")]
    T4G_Small,
    #[serde(rename = "db.t4g.medium")]
    T4G_Medium,
    #[serde(rename = "db.t4g.large")]
    T4G_Large,
    #[serde(rename = "db.t4g.xlarge")]
    T4G_XLarge,
    #[serde(rename = "db.t4g.2xlarge")]
    T4G_2XLarge,
    #[serde(rename = "db.t3.micro")]
    T3_Micro,
    #[serde(rename = "db.t3.small")]
    T3_Small,
    #[serde(rename = "db.t3.medium")]
    T3_Medium,
    #[serde(rename = "db.t3.large")]
    T3_Large,
    #[serde(rename = "db.t3.xlarge")]
    T3_XLarge,
    #[serde(rename = "db.t3.2xlarge")]
    T3_2XLarge,
    #[serde(rename = "db.t2.micro")]
    T2_Micro,
    #[serde(rename = "db.t2.small")]
    T2_Small,
    #[serde(rename = "db.t2.medium")]
    T2_Medium,
    #[serde(rename = "db.t2.large")]
    T2_Large,
    #[serde(rename = "db.t2.xlarge")]
    T2_XLarge,
    #[serde(rename = "db.t2.2xlarge")]
    T2_2XLarge,
    #[serde(rename = "db.m1.small")]
    M1_Small,
    #[serde(rename = "db.m1.medium")]
    M1_Medium,
    #[serde(rename = "db.m1.large")]
    M1_Large,
    #[serde(rename = "db.m1.xlarge")]
    M1_XLarge,
    #[serde(rename = "db.m2.xlarge")]
    M2_XLarge,
    #[serde(rename = "db.m2.2xlarge")]
    M2_2XLarge,
    #[serde(rename = "db.m2.4xlarge")]
    M2_4XLarge,
    #[serde(rename = "db.m3.medium")]
    M3_Medium,
    #[serde(rename = "db.m3.large")]
    M3_Large,
    #[serde(rename = "db.m3.xlarge")]
    M3_XLarge,
    #[serde(rename = "db.m3.2xlarge")]
    M3_2XLarge,
    #[serde(rename = "db.m4.large")]
    M4_Large,
    #[serde(rename = "db.m4.xlarge")]
    M4_XLarge,
    #[serde(rename = "db.m4.2xlarge")]
    M4_2XLarge,
    #[serde(rename = "db.m4.4xlarge")]
    M4_4XLarge,
    #[serde(rename = "db.m4.10xlarge")]
    M4_10XLarge,
    #[serde(rename = "db.m4.10xlarge")]
    M4_16XLarge,
    #[serde(rename = "db.m5.large")]
    M5_Large,
    #[serde(rename = "db.m5.xlarge")]
    M5_XLarge,
    #[serde(rename = "db.m5.2xlarge")]
    M5_2XLarge,
    #[serde(rename = "db.m5.4xlarge")]
    M5_4XLarge,
    #[serde(rename = "db.m5.12xlarge")]
    M5_12XLarge,
    #[serde(rename = "db.m5.24xlarge")]
    M5_24XLarge,
    #[serde(rename = "db.m6g.large")]
    M6G_Large,
    #[serde(rename = "db.m6g.xlarge")]
    M6G_XLarge,
    #[serde(rename = "db.m6g.2xlarge")]
    M6G_2XLarge,
    #[serde(rename = "db.m6g.4xlarge")]
    M6G_4XLarge,
    #[serde(rename = "db.m6g.8xlarge")]
    M6G_8XLarge,
    #[serde(rename = "db.m6g.12xlarge")]
    M6G_12XLarge,
    #[serde(rename = "db.m6g.16xlarge")]
    M6G_16XLarge,
    #[serde(rename = "db.r3.large")]
    R3_Large,
    #[serde(rename = "db.r3.xlarge")]
    R3_XLarge,
    #[serde(rename = "db.r3.2xlarge")]
    R3_2XLarge,
    #[serde(rename = "db.r3.4xlarge")]
    R3_4XLarge,
    #[serde(rename = "db.r3.8xlarge")]
    R3_8XLarge,
    #[serde(rename = "db.r4.large")]
    R4_Large,
    #[serde(rename = "db.r4.xlarge")]
    R4_XLarge,
    #[serde(rename = "db.r4.2xlarge")]
    R4_2XLarge,
    #[serde(rename = "db.r4.4xlarge")]
    R4_4XLarge,
    #[serde(rename = "db.r4.8xlarge")]
    R4_8XLarge,
    #[serde(rename = "db.r4.16xlarge")]
    R4_16XLarge,
    #[serde(rename = "db.r5.large")]
    R5_Large,
    #[serde(rename = "db.r5.xlarge")]
    R5_XLarge,
    #[serde(rename = "db.r5.2xlarge")]
    R5_2XLarge,
    #[serde(rename = "db.r5.4xlarge")]
    R5_4XLarge,
    #[serde(rename = "db.r5.12xlarge")]
    R5_12XLarge,
    #[serde(rename = "db.r5.24xlarge")]
    R5_24XLarge,
    #[serde(rename = "db.r6g.large")]
    R6G_Large,
    #[serde(rename = "db.r6g.xlarge")]
    R6G_XLarge,
    #[serde(rename = "db.r6g.2xlarge")]
    R6G_2XLarge,
    #[serde(rename = "db.r6g.4xlarge")]
    R6G_4XLarge,
    #[serde(rename = "db.r6g.8xlarge")]
    R6G_8XLarge,
    #[serde(rename = "db.r6g.12xlarge")]
    R6G_12XLarge,
    #[serde(rename = "db.r6g.16xlarge")]
    R6G_16XLarge,
    #[serde(rename = "db.x1.16xlarge")]
    X1_16XLarge,
    #[serde(rename = "db.x1.32xlarge")]
    X1_32XLarge,
    #[serde(rename = "db.x1e.xlarge")]
    X1E_XLarge,
    #[serde(rename = "db.x1e.2xlarge")]
    X1E_2XLarge,
    #[serde(rename = "db.x1e.4xlarge")]
    X1E_4XLarge,
    #[serde(rename = "db.x1e.8xlarge")]
    X1E_8XLarge,
    #[serde(rename = "db.x1e.32xlarge")]
    X1E_32XLarge,
}
