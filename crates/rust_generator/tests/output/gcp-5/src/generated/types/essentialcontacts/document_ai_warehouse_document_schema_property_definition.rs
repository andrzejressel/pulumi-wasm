#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinition {
    /// Date time property. Not supported by CMEK compliant deployment.
    #[builder(into, default)]
    #[serde(rename = "dateTimeTypeOptions")]
    pub r#date_time_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionDateTimeTypeOptions>>,
    /// The display-name for the property, used for front-end.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// Enum/categorical property.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "enumTypeOptions")]
    pub r#enum_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionEnumTypeOptions>>,
    /// Float property.
    #[builder(into, default)]
    #[serde(rename = "floatTypeOptions")]
    pub r#float_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionFloatTypeOptions>>,
    /// Integer property.
    #[builder(into, default)]
    #[serde(rename = "integerTypeOptions")]
    pub r#integer_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionIntegerTypeOptions>>,
    /// Whether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable.
    #[builder(into, default)]
    #[serde(rename = "isFilterable")]
    pub r#is_filterable: Box<Option<bool>>,
    /// Whether the property is user supplied metadata.
    #[builder(into, default)]
    #[serde(rename = "isMetadata")]
    pub r#is_metadata: Box<Option<bool>>,
    /// Whether the property can have multiple values.
    #[builder(into, default)]
    #[serde(rename = "isRepeatable")]
    pub r#is_repeatable: Box<Option<bool>>,
    /// Whether the property is mandatory.
    #[builder(into, default)]
    #[serde(rename = "isRequired")]
    pub r#is_required: Box<Option<bool>>,
    /// Indicates that the property should be included in a global search.
    #[builder(into, default)]
    #[serde(rename = "isSearchable")]
    pub r#is_searchable: Box<Option<bool>>,
    /// Map property.
    #[builder(into, default)]
    #[serde(rename = "mapTypeOptions")]
    pub r#map_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionMapTypeOptions>>,
    /// The name of the metadata property.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Nested structured data property.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "propertyTypeOptions")]
    pub r#property_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptions>>,
    /// Stores the retrieval importance.
    /// Possible values are: `HIGHEST`, `HIGHER`, `HIGH`, `MEDIUM`, `LOW`, `LOWEST`.
    #[builder(into, default)]
    #[serde(rename = "retrievalImportance")]
    pub r#retrieval_importance: Box<Option<String>>,
    /// The schema source information.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "schemaSources")]
    pub r#schema_sources: Box<Option<Vec<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionSchemaSource>>>,
    /// Text property.
    #[builder(into, default)]
    #[serde(rename = "textTypeOptions")]
    pub r#text_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionTextTypeOptions>>,
    /// Timestamp property. Not supported by CMEK compliant deployment.
    #[builder(into, default)]
    #[serde(rename = "timestampTypeOptions")]
    pub r#timestamp_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionTimestampTypeOptions>>,
}
