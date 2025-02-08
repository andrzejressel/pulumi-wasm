#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EntityRecognizerInputDataConfigAugmentedManifest {
    /// Location of annotation files.
    #[builder(into, default)]
    #[serde(rename = "annotationDataS3Uri")]
    pub r#annotation_data_s_3_uri: Box<Option<String>>,
    /// The JSON attribute that contains the annotations for the training documents.
    #[builder(into)]
    #[serde(rename = "attributeNames")]
    pub r#attribute_names: Box<Vec<String>>,
    /// Type of augmented manifest.
    /// One of `PLAIN_TEXT_DOCUMENT` or `SEMI_STRUCTURED_DOCUMENT`.
    #[builder(into, default)]
    #[serde(rename = "documentType")]
    pub r#document_type: Box<Option<String>>,
    /// Location of augmented manifest file.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Box<String>,
    /// Location of source PDF files.
    #[builder(into, default)]
    #[serde(rename = "sourceDocumentsS3Uri")]
    pub r#source_documents_s_3_uri: Box<Option<String>>,
    /// Purpose of data in augmented manifest.
    /// One of `TRAIN` or `TEST`.
    #[builder(into, default)]
    #[serde(rename = "split")]
    pub r#split: Box<Option<String>>,
}
