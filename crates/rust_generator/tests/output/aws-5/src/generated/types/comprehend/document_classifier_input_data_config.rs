#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DocumentClassifierInputDataConfig {
    /// List of training datasets produced by Amazon SageMaker Ground Truth.
    /// Used if `data_format` is `AUGMENTED_MANIFEST`.
    /// See the `augmented_manifests` Configuration Block section below.
    #[builder(into, default)]
    #[serde(rename = "augmentedManifests")]
    pub r#augmented_manifests: Box<Option<Vec<super::super::types::comprehend::DocumentClassifierInputDataConfigAugmentedManifest>>>,
    /// The format for the training data.
    /// One of `COMPREHEND_CSV` or `AUGMENTED_MANIFEST`.
    #[builder(into, default)]
    #[serde(rename = "dataFormat")]
    pub r#data_format: Box<Option<String>>,
    /// Delimiter between labels when training a multi-label classifier.
    /// Valid values are `|`, `~`, `!`, `@`, `#`, `$`, `%`, `^`, `*`, `-`, `_`, `+`, `=`, `\`, `:`, `;`, `>`, `?`, `/`, `<space>`, and `<tab>`.
    /// Default is `|`.
    #[builder(into, default)]
    #[serde(rename = "labelDelimiter")]
    pub r#label_delimiter: Box<Option<String>>,
    /// Location of training documents.
    /// Used if `data_format` is `COMPREHEND_CSV`.
    #[builder(into, default)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "testS3Uri")]
    pub r#test_s_3_uri: Box<Option<String>>,
}
