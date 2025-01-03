#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Runtime {
    #[serde(rename = "dotnet6")]
    Dotnet6,
    #[serde(rename = "dotnet8")]
    Dotnet8,
    #[serde(rename = "java11")]
    Java11,
    #[serde(rename = "java17")]
    Java17,
    #[serde(rename = "java21")]
    Java21,
    #[serde(rename = "java8.al2")]
    Java8AL2,
    #[serde(rename = "nodejs18.x")]
    NodeJS18dX,
    #[serde(rename = "nodejs20.x")]
    NodeJS20dX,
    #[serde(rename = "nodejs22.x")]
    NodeJS22dX,
    #[serde(rename = "provided.al2")]
    CustomAL2,
    #[serde(rename = "provided.al2023")]
    CustomAL2023,
    #[serde(rename = "python3.10")]
    Python3d10,
    #[serde(rename = "python3.11")]
    Python3d11,
    #[serde(rename = "python3.12")]
    Python3d12,
    #[serde(rename = "python3.13")]
    Python3d13,
    #[serde(rename = "python3.9")]
    Python3d9,
    #[serde(rename = "ruby3.2")]
    Ruby3d2,
    #[serde(rename = "ruby3.3")]
    Ruby3d3,
    #[serde(rename = "dotnet5.0")]
    Dotnet5d0,
    #[serde(rename = "dotnet7")]
    Dotnet7,
    #[serde(rename = "dotnetcore2.1")]
    DotnetCore2d1,
    #[serde(rename = "dotnetcore3.1")]
    DotnetCore3d1,
    #[serde(rename = "go1.x")]
    Go1dx,
    #[serde(rename = "java8")]
    Java8,
    #[serde(rename = "nodejs10.x")]
    NodeJS10dX,
    #[serde(rename = "nodejs12.x")]
    NodeJS12dX,
    #[serde(rename = "nodejs14.x")]
    NodeJS14dX,
    #[serde(rename = "nodejs16.x")]
    NodeJS16dX,
    #[serde(rename = "provided")]
    Custom,
    #[serde(rename = "python2.7")]
    Python2d7,
    #[serde(rename = "python3.6")]
    Python3d6,
    #[serde(rename = "python3.7")]
    Python3d7,
    #[serde(rename = "python3.8")]
    Python3d8,
    #[serde(rename = "ruby2.5")]
    Ruby2d5,
    #[serde(rename = "ruby2.7")]
    Ruby2d7,
}
