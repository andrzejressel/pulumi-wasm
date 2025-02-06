#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EntityRecognizerInputDataConfig {
    /// Specifies location of the document annotation data.
    /// See the `annotations` Configuration Block section below.
    /// One of `annotations` or `entity_list` is required.
    #[builder(into, default)]
    #[serde(rename = "annotations")]
    pub r#annotations: Box<Option<super::super::types::comprehend::EntityRecognizerInputDataConfigAnnotations>>,
    /// List of training datasets produced by Amazon SageMaker Ground Truth.
    /// Used if `data_format` is `AUGMENTED_MANIFEST`.
    /// See the `augmented_manifests` Configuration Block section below.
    #[builder(into, default)]
    #[serde(rename = "augmentedManifests")]
    pub r#augmented_manifests: Box<Option<Vec<super::super::types::comprehend::EntityRecognizerInputDataConfigAugmentedManifest>>>,
    /// The format for the training data.
    /// One of `COMPREHEND_CSV` or `AUGMENTED_MANIFEST`.
    #[builder(into, default)]
    #[serde(rename = "dataFormat")]
    pub r#data_format: Box<Option<String>>,
    /// Specifies a collection of training documents.
    /// Used if `data_format` is `COMPREHEND_CSV`.
    /// See the `documents` Configuration Block section below.
    #[builder(into, default)]
    #[serde(rename = "documents")]
    pub r#documents: Box<Option<super::super::types::comprehend::EntityRecognizerInputDataConfigDocuments>>,
    /// Specifies location of the entity list data.
    /// See the `entity_list` Configuration Block section below.
    /// One of `entity_list` or `annotations` is required.
    #[builder(into, default)]
    #[serde(rename = "entityList")]
    pub r#entity_list: Box<Option<super::super::types::comprehend::EntityRecognizerInputDataConfigEntityList>>,
    /// Set of entity types to be recognized.
    /// Has a maximum of 25 items.
    /// See the `entity_types` Configuration Block section below.
    #[builder(into)]
    #[serde(rename = "entityTypes")]
    pub r#entity_types: Box<Vec<super::super::types::comprehend::EntityRecognizerInputDataConfigEntityType>>,
}
