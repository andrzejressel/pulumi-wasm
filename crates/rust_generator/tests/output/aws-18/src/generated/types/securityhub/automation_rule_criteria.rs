#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AutomationRuleCriteria {
    /// The AWS account ID in which a finding was generated. Documented below.
    #[builder(into, default)]
    #[serde(rename = "awsAccountIds")]
    pub r#aws_account_ids: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaAwsAccountId>>>,
    /// The name of the AWS account in which a finding was generated. Documented below.
    #[builder(into, default)]
    #[serde(rename = "awsAccountNames")]
    pub r#aws_account_names: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaAwsAccountName>>>,
    /// The name of the company for the product that generated the finding. For control-based findings, the company is AWS. Documented below.
    #[builder(into, default)]
    #[serde(rename = "companyNames")]
    pub r#company_names: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaCompanyName>>>,
    /// The unique identifier of a standard in which a control is enabled. Documented below.
    #[builder(into, default)]
    #[serde(rename = "complianceAssociatedStandardsIds")]
    pub r#compliance_associated_standards_ids: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaComplianceAssociatedStandardsId>>>,
    /// The security control ID for which a finding was generated. Security control IDs are the same across standards. Documented below.
    #[builder(into, default)]
    #[serde(rename = "complianceSecurityControlIds")]
    pub r#compliance_security_control_ids: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaComplianceSecurityControlId>>>,
    /// The result of a security check. This field is only used for findings generated from controls. Documented below.
    #[builder(into, default)]
    #[serde(rename = "complianceStatuses")]
    pub r#compliance_statuses: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaComplianceStatus>>>,
    /// The likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. `Confidence` is scored on a 0–100 basis using a ratio scale. A value of `0` means 0 percent confidence, and a value of `100` means 100 percent confidence. Documented below.
    #[builder(into, default)]
    #[serde(rename = "confidences")]
    pub r#confidences: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaConfidence>>>,
    /// A timestamp that indicates when this finding record was created. Documented below.
    #[builder(into, default)]
    #[serde(rename = "createdAts")]
    pub r#created_ats: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaCreatedAt>>>,
    /// The level of importance that is assigned to the resources that are associated with a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "criticalities")]
    pub r#criticalities: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaCriticality>>>,
    /// A finding's description. Documented below.
    #[builder(into, default)]
    #[serde(rename = "descriptions")]
    pub r#descriptions: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaDescription>>>,
    /// A timestamp that indicates when the potential security issue captured by a finding was first observed by the security findings product. Documented below.
    #[builder(into, default)]
    #[serde(rename = "firstObservedAts")]
    pub r#first_observed_ats: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaFirstObservedAt>>>,
    /// The identifier for the solution-specific component that generated a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "generatorIds")]
    pub r#generator_ids: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaGeneratorId>>>,
    /// The product-specific identifier for a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaId>>>,
    /// A timestamp that indicates when the potential security issue captured by a finding was most recently observed by the security findings product. Documented below.
    #[builder(into, default)]
    #[serde(rename = "lastObservedAts")]
    pub r#last_observed_ats: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaLastObservedAt>>>,
    /// The text of a user-defined note that's added to a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "noteTexts")]
    pub r#note_texts: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaNoteText>>>,
    /// The timestamp of when the note was updated. Documented below.
    #[builder(into, default)]
    #[serde(rename = "noteUpdatedAts")]
    pub r#note_updated_ats: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaNoteUpdatedAt>>>,
    /// The principal that created a note. Documented below.
    #[builder(into, default)]
    #[serde(rename = "noteUpdatedBies")]
    pub r#note_updated_bies: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaNoteUpdatedBy>>>,
    /// The Amazon Resource Name (ARN) for a third-party product that generated a finding in Security Hub. Documented below.
    #[builder(into, default)]
    #[serde(rename = "productArns")]
    pub r#product_arns: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaProductArn>>>,
    /// Provides the name of the product that generated the finding. For control-based findings, the product name is Security Hub. Documented below.
    #[builder(into, default)]
    #[serde(rename = "productNames")]
    pub r#product_names: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaProductName>>>,
    /// Provides the current state of a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "recordStates")]
    pub r#record_states: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaRecordState>>>,
    /// The product-generated identifier for a related finding.  Documented below.
    #[builder(into, default)]
    #[serde(rename = "relatedFindingsIds")]
    pub r#related_findings_ids: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaRelatedFindingsId>>>,
    /// The ARN for the product that generated a related finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "relatedFindingsProductArns")]
    pub r#related_findings_product_arns: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaRelatedFindingsProductArn>>>,
    /// The Amazon Resource Name (ARN) of the application that is related to a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceApplicationArns")]
    pub r#resource_application_arns: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceApplicationArn>>>,
    /// The name of the application that is related to a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceApplicationNames")]
    pub r#resource_application_names: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceApplicationName>>>,
    /// Custom fields and values about the resource that a finding pertains to. Documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceDetailsOthers")]
    pub r#resource_details_others: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceDetailsOther>>>,
    /// The identifier for the given resource type. For AWS resources that are identified by Amazon Resource Names (ARNs), this is the ARN. For AWS resources that lack ARNs, this is the identifier as defined by the AWS service that created the resource. For non-AWS resources, this is a unique identifier that is associated with the resource. Documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceIds")]
    pub r#resource_ids: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceId>>>,
    /// The partition in which the resource that the finding pertains to is located. A partition is a group of AWS Regions. Each AWS account is scoped to one partition. Documented below.
    #[builder(into, default)]
    #[serde(rename = "resourcePartitions")]
    pub r#resource_partitions: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourcePartition>>>,
    /// The AWS Region where the resource that a finding pertains to is located. Documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceRegions")]
    pub r#resource_regions: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceRegion>>>,
    /// A list of AWS tags associated with a resource at the time the finding was processed. Documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceTags")]
    pub r#resource_tags: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceTag>>>,
    /// The type of resource that the finding pertains to. Documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceType>>>,
    /// The severity value of the finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "severityLabels")]
    pub r#severity_labels: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaSeverityLabel>>>,
    /// Provides a URL that links to a page about the current finding in the finding product. Documented below.
    #[builder(into, default)]
    #[serde(rename = "sourceUrls")]
    pub r#source_urls: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaSourceUrl>>>,
    /// A finding's title. Documented below.
    #[builder(into, default)]
    #[serde(rename = "titles")]
    pub r#titles: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaTitle>>>,
    /// One or more finding types in the format of namespace/category/classifier that classify a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "types")]
    pub r#types: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaType>>>,
    /// A timestamp that indicates when the finding record was most recently updated. Documented below.
    #[builder(into, default)]
    #[serde(rename = "updatedAts")]
    pub r#updated_ats: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaUpdatedAt>>>,
    /// A list of user-defined name and value string pairs added to a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "userDefinedFields")]
    pub r#user_defined_fields: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaUserDefinedField>>>,
    /// Provides the veracity of a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "verificationStates")]
    pub r#verification_states: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaVerificationState>>>,
    /// Provides information about the status of the investigation into a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "workflowStatuses")]
    pub r#workflow_statuses: Box<Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaWorkflowStatus>>>,
}
