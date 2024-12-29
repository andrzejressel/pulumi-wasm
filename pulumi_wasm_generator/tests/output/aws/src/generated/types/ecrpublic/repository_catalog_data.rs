#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryCatalogData {
    /// A detailed description of the contents of the repository. It is publicly visible in the Amazon ECR Public Gallery. The text must be in markdown format.
    #[builder(into, default)]
    #[serde(rename = "aboutText")]
    pub r#about_text: Box<Option<String>>,
    /// The system architecture that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported architectures will appear as badges on the repository and are used as search filters: `ARM`, `ARM 64`, `x86`, `x86-64`
    #[builder(into, default)]
    #[serde(rename = "architectures")]
    pub r#architectures: Box<Option<Vec<String>>>,
    /// A short description of the contents of the repository. This text appears in both the image details and also when searching for repositories on the Amazon ECR Public Gallery.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The base64-encoded repository logo payload. (Only visible for verified accounts) Note that drift detection is disabled for this attribute.
    #[builder(into, default)]
    #[serde(rename = "logoImageBlob")]
    pub r#logo_image_blob: Box<Option<String>>,
    /// The operating systems that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported operating systems will appear as badges on the repository and are used as search filters: `Linux`, `Windows`
    #[builder(into, default)]
    #[serde(rename = "operatingSystems")]
    pub r#operating_systems: Box<Option<Vec<String>>>,
    /// Detailed information on how to use the contents of the repository. It is publicly visible in the Amazon ECR Public Gallery. The usage text provides context, support information, and additional usage details for users of the repository. The text must be in markdown format.
    #[builder(into, default)]
    #[serde(rename = "usageText")]
    pub r#usage_text: Box<Option<String>>,
}
