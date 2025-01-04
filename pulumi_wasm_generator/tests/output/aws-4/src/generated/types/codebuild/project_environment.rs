#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectEnvironment {
    /// ARN of the S3 bucket, path prefix and object key that contains the PEM-encoded certificate.
    #[builder(into, default)]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<String>>,
    /// Information about the compute resources the build project will use. Valid values: `BUILD_GENERAL1_SMALL`, `BUILD_GENERAL1_MEDIUM`, `BUILD_GENERAL1_LARGE`, `BUILD_GENERAL1_2XLARGE`, `BUILD_LAMBDA_1GB`, `BUILD_LAMBDA_2GB`, `BUILD_LAMBDA_4GB`, `BUILD_LAMBDA_8GB`, `BUILD_LAMBDA_10GB`. `BUILD_GENERAL1_SMALL` is only valid if `type` is set to `LINUX_CONTAINER`. When `type` is set to `LINUX_GPU_CONTAINER`, `compute_type` must be `BUILD_GENERAL1_LARGE`. When `type` is set to `LINUX_LAMBDA_CONTAINER` or `ARM_LAMBDA_CONTAINER`, `compute_type` must be `BUILD_LAMBDA_XGB`.`
    #[builder(into)]
    #[serde(rename = "computeType")]
    pub r#compute_type: Box<String>,
    /// Configuration block. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<Vec<super::super::types::codebuild::ProjectEnvironmentEnvironmentVariable>>>,
    /// Configuration block. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "fleet")]
    pub r#fleet: Box<Option<super::super::types::codebuild::ProjectEnvironmentFleet>>,
    /// Docker image to use for this build project. Valid values include [Docker images provided by CodeBuild](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-available.html) (e.g `aws/codebuild/amazonlinux2-x86_64-standard:4.0`), [Docker Hub images](https://hub.docker.com/) (e.g., `pulumi/pulumi:latest`), and full Docker repository URIs such as those for ECR (e.g., `137112412989.dkr.ecr.us-west-2.amazonaws.com/amazonlinux:latest`).
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// Type of credentials AWS CodeBuild uses to pull images in your build. Valid values: `CODEBUILD`, `SERVICE_ROLE`. When you use a cross-account or private registry image, you must use SERVICE_ROLE credentials. When you use an AWS CodeBuild curated image, you must use CodeBuild credentials. Defaults to `CODEBUILD`.
    #[builder(into, default)]
    #[serde(rename = "imagePullCredentialsType")]
    pub r#image_pull_credentials_type: Box<Option<String>>,
    /// Whether to enable running the Docker daemon inside a Docker container. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "privilegedMode")]
    pub r#privileged_mode: Box<Option<bool>>,
    /// Configuration block. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "registryCredential")]
    pub r#registry_credential: Box<Option<super::super::types::codebuild::ProjectEnvironmentRegistryCredential>>,
    /// Type of build environment to use for related builds. Valid values: `LINUX_CONTAINER`, `LINUX_GPU_CONTAINER`, `WINDOWS_CONTAINER` (deprecated), `WINDOWS_SERVER_2019_CONTAINER`, `ARM_CONTAINER`, `LINUX_LAMBDA_CONTAINER`, `ARM_LAMBDA_CONTAINER`. For additional information, see the [CodeBuild User Guide](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
