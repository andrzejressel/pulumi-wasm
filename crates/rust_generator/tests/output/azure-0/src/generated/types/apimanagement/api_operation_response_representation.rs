#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiOperationResponseRepresentation {
    /// The Content Type of this representation, such as `application/json`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
    /// One or more `example` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "examples")]
    pub r#examples: Box<Option<Vec<super::super::types::apimanagement::ApiOperationResponseRepresentationExample>>>,
    /// One or more `form_parameter` block as defined above.
    /// 
    /// > **NOTE:** This is Required when `content_type` is set to `application/x-www-form-urlencoded` or `multipart/form-data`.
    #[builder(into, default)]
    #[serde(rename = "formParameters")]
    pub r#form_parameters: Box<Option<Vec<super::super::types::apimanagement::ApiOperationResponseRepresentationFormParameter>>>,
    /// The ID of an API Management Schema which represents this Response.
    /// 
    /// > **NOTE:** This can only be specified when `content_type` is not set to `application/x-www-form-urlencoded` or `multipart/form-data`.
    #[builder(into, default)]
    #[serde(rename = "schemaId")]
    pub r#schema_id: Box<Option<String>>,
    /// The Type Name defined by the Schema.
    /// 
    /// > **NOTE:** This can only be specified when `content_type` is not set to `application/x-www-form-urlencoded` or `multipart/form-data`.
    #[builder(into, default)]
    #[serde(rename = "typeName")]
    pub r#type_name: Box<Option<String>>,
}
