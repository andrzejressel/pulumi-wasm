#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InsightFilters {
    /// AWS account ID that a finding is generated in. See String_Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "awsAccountIds")]
    pub r#aws_account_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersAwsAccountId>>>,
    /// The name of the findings provider (company) that owns the solution (product) that generates findings. See String_Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "companyNames")]
    pub r#company_names: Box<Option<Vec<super::super::types::securityhub::InsightFiltersCompanyName>>>,
    /// Exclusive to findings that are generated as the result of a check run against a specific rule in a supported standard, such as CIS AWS Foundations. Contains security standard-related finding details. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "complianceStatuses")]
    pub r#compliance_statuses: Box<Option<Vec<super::super::types::securityhub::InsightFiltersComplianceStatus>>>,
    /// A finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence. See Number Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "confidences")]
    pub r#confidences: Box<Option<Vec<super::super::types::securityhub::InsightFiltersConfidence>>>,
    /// An ISO8601-formatted timestamp that indicates when the security-findings provider captured the potential security issue that a finding captured. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "createdAts")]
    pub r#created_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersCreatedAt>>>,
    /// The level of importance assigned to the resources associated with the finding. A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources. See Number Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "criticalities")]
    pub r#criticalities: Box<Option<Vec<super::super::types::securityhub::InsightFiltersCriticality>>>,
    /// A finding's description. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "descriptions")]
    pub r#descriptions: Box<Option<Vec<super::super::types::securityhub::InsightFiltersDescription>>>,
    /// The finding provider value for the finding confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence. See Number Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "findingProviderFieldsConfidences")]
    pub r#finding_provider_fields_confidences: Box<Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsConfidence>>>,
    /// The finding provider value for the level of importance assigned to the resources associated with the findings. A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources. See Number Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "findingProviderFieldsCriticalities")]
    pub r#finding_provider_fields_criticalities: Box<Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsCriticality>>>,
    /// The finding identifier of a related finding that is identified by the finding provider. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "findingProviderFieldsRelatedFindingsIds")]
    pub r#finding_provider_fields_related_findings_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsRelatedFindingsId>>>,
    /// The ARN of the solution that generated a related finding that is identified by the finding provider. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "findingProviderFieldsRelatedFindingsProductArns")]
    pub r#finding_provider_fields_related_findings_product_arns: Box<Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsRelatedFindingsProductArn>>>,
    /// The finding provider value for the severity label. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "findingProviderFieldsSeverityLabels")]
    pub r#finding_provider_fields_severity_labels: Box<Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsSeverityLabel>>>,
    /// The finding provider's original value for the severity. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "findingProviderFieldsSeverityOriginals")]
    pub r#finding_provider_fields_severity_originals: Box<Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsSeverityOriginal>>>,
    /// One or more finding types that the finding provider assigned to the finding. Uses the format of `namespace/category/classifier` that classify a finding. Valid namespace values include: `Software and Configuration Checks`, `TTPs`, `Effects`, `Unusual Behaviors`, and `Sensitive Data Identifications`. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "findingProviderFieldsTypes")]
    pub r#finding_provider_fields_types: Box<Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsType>>>,
    /// An ISO8601-formatted timestamp that indicates when the security-findings provider first observed the potential security issue that a finding captured. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "firstObservedAts")]
    pub r#first_observed_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersFirstObservedAt>>>,
    /// The identifier for the solution-specific component (a discrete unit of logic) that generated a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "generatorIds")]
    pub r#generator_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersGeneratorId>>>,
    /// The security findings provider-specific identifier for a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersId>>>,
    /// A keyword for a finding. See Keyword Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "keywords")]
    pub r#keywords: Box<Option<Vec<super::super::types::securityhub::InsightFiltersKeyword>>>,
    /// An ISO8601-formatted timestamp that indicates when the security-findings provider most recently observed the potential security issue that a finding captured. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "lastObservedAts")]
    pub r#last_observed_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersLastObservedAt>>>,
    /// The name of the malware that was observed. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "malwareNames")]
    pub r#malware_names: Box<Option<Vec<super::super::types::securityhub::InsightFiltersMalwareName>>>,
    /// The filesystem path of the malware that was observed. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "malwarePaths")]
    pub r#malware_paths: Box<Option<Vec<super::super::types::securityhub::InsightFiltersMalwarePath>>>,
    /// The state of the malware that was observed. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "malwareStates")]
    pub r#malware_states: Box<Option<Vec<super::super::types::securityhub::InsightFiltersMalwareState>>>,
    /// The type of the malware that was observed. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "malwareTypes")]
    pub r#malware_types: Box<Option<Vec<super::super::types::securityhub::InsightFiltersMalwareType>>>,
    /// The destination domain of network-related information about a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkDestinationDomains")]
    pub r#network_destination_domains: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDestinationDomain>>>,
    /// The destination IPv4 address of network-related information about a finding. See Ip Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkDestinationIpv4s")]
    pub r#network_destination_ipv_4_s: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDestinationIpv4>>>,
    /// The destination IPv6 address of network-related information about a finding. See Ip Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkDestinationIpv6s")]
    pub r#network_destination_ipv_6_s: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDestinationIpv6>>>,
    /// The destination port of network-related information about a finding. See Number Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkDestinationPorts")]
    pub r#network_destination_ports: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDestinationPort>>>,
    /// Indicates the direction of network traffic associated with a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkDirections")]
    pub r#network_directions: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDirection>>>,
    /// The protocol of network-related information about a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkProtocols")]
    pub r#network_protocols: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkProtocol>>>,
    /// The source domain of network-related information about a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkSourceDomains")]
    pub r#network_source_domains: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourceDomain>>>,
    /// The source IPv4 address of network-related information about a finding. See Ip Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkSourceIpv4s")]
    pub r#network_source_ipv_4_s: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourceIpv4>>>,
    /// The source IPv6 address of network-related information about a finding. See Ip Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkSourceIpv6s")]
    pub r#network_source_ipv_6_s: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourceIpv6>>>,
    /// The source media access control (MAC) address of network-related information about a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkSourceMacs")]
    pub r#network_source_macs: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourceMac>>>,
    /// The source port of network-related information about a finding. See Number Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "networkSourcePorts")]
    pub r#network_source_ports: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourcePort>>>,
    /// The text of a note. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "noteTexts")]
    pub r#note_texts: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNoteText>>>,
    /// The timestamp of when the note was updated. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "noteUpdatedAts")]
    pub r#note_updated_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNoteUpdatedAt>>>,
    /// The principal that created a note. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "noteUpdatedBies")]
    pub r#note_updated_bies: Box<Option<Vec<super::super::types::securityhub::InsightFiltersNoteUpdatedBy>>>,
    /// The date/time that the process was launched. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "processLaunchedAts")]
    pub r#process_launched_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersProcessLaunchedAt>>>,
    /// The name of the process. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "processNames")]
    pub r#process_names: Box<Option<Vec<super::super::types::securityhub::InsightFiltersProcessName>>>,
    /// The parent process ID. See Number Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "processParentPids")]
    pub r#process_parent_pids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersProcessParentPid>>>,
    /// The path to the process executable. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "processPaths")]
    pub r#process_paths: Box<Option<Vec<super::super::types::securityhub::InsightFiltersProcessPath>>>,
    /// The process ID. See Number Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "processPids")]
    pub r#process_pids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersProcessPid>>>,
    /// The date/time that the process was terminated. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "processTerminatedAts")]
    pub r#process_terminated_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersProcessTerminatedAt>>>,
    /// The ARN generated by Security Hub that uniquely identifies a third-party company (security findings provider) after this provider's product (solution that generates findings) is registered with Security Hub. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "productArns")]
    pub r#product_arns: Box<Option<Vec<super::super::types::securityhub::InsightFiltersProductArn>>>,
    /// A data type where security-findings providers can include additional solution-specific details that aren't part of the defined `AwsSecurityFinding` format. See Map Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "productFields")]
    pub r#product_fields: Box<Option<Vec<super::super::types::securityhub::InsightFiltersProductField>>>,
    /// The name of the solution (product) that generates findings. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "productNames")]
    pub r#product_names: Box<Option<Vec<super::super::types::securityhub::InsightFiltersProductName>>>,
    /// The recommendation of what to do about the issue described in a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "recommendationTexts")]
    pub r#recommendation_texts: Box<Option<Vec<super::super::types::securityhub::InsightFiltersRecommendationText>>>,
    /// The updated record state for the finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "recordStates")]
    pub r#record_states: Box<Option<Vec<super::super::types::securityhub::InsightFiltersRecordState>>>,
    /// The solution-generated identifier for a related finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "relatedFindingsIds")]
    pub r#related_findings_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersRelatedFindingsId>>>,
    /// The ARN of the solution that generated a related finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "relatedFindingsProductArns")]
    pub r#related_findings_product_arns: Box<Option<Vec<super::super::types::securityhub::InsightFiltersRelatedFindingsProductArn>>>,
    /// The IAM profile ARN of the instance. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsEc2InstanceIamInstanceProfileArns")]
    pub r#resource_aws_ec_2_instance_iam_instance_profile_arns: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceIamInstanceProfileArn>>>,
    /// The Amazon Machine Image (AMI) ID of the instance. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsEc2InstanceImageIds")]
    pub r#resource_aws_ec_2_instance_image_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceImageId>>>,
    /// The IPv4 addresses associated with the instance. See Ip Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsEc2InstanceIpv4Addresses")]
    pub r#resource_aws_ec_2_instance_ipv_4_addresses: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceIpv4Address>>>,
    /// The IPv6 addresses associated with the instance. See Ip Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsEc2InstanceIpv6Addresses")]
    pub r#resource_aws_ec_2_instance_ipv_6_addresses: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceIpv6Address>>>,
    /// The key name associated with the instance. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsEc2InstanceKeyNames")]
    pub r#resource_aws_ec_2_instance_key_names: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceKeyName>>>,
    /// The date and time the instance was launched. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsEc2InstanceLaunchedAts")]
    pub r#resource_aws_ec_2_instance_launched_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceLaunchedAt>>>,
    /// The identifier of the subnet that the instance was launched in. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsEc2InstanceSubnetIds")]
    pub r#resource_aws_ec_2_instance_subnet_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceSubnetId>>>,
    /// The instance type of the instance. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsEc2InstanceTypes")]
    pub r#resource_aws_ec_2_instance_types: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceType>>>,
    /// The identifier of the VPC that the instance was launched in. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsEc2InstanceVpcIds")]
    pub r#resource_aws_ec_2_instance_vpc_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceVpcId>>>,
    /// The creation date/time of the IAM access key related to a finding. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsIamAccessKeyCreatedAts")]
    pub r#resource_aws_iam_access_key_created_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsIamAccessKeyCreatedAt>>>,
    /// The status of the IAM access key related to a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsIamAccessKeyStatuses")]
    pub r#resource_aws_iam_access_key_statuses: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsIamAccessKeyStatus>>>,
    /// The user associated with the IAM access key related to a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsIamAccessKeyUserNames")]
    pub r#resource_aws_iam_access_key_user_names: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsIamAccessKeyUserName>>>,
    /// The canonical user ID of the owner of the S3 bucket. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsS3BucketOwnerIds")]
    pub r#resource_aws_s_3_bucket_owner_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsS3BucketOwnerId>>>,
    /// The display name of the owner of the S3 bucket. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceAwsS3BucketOwnerNames")]
    pub r#resource_aws_s_3_bucket_owner_names: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsS3BucketOwnerName>>>,
    /// The identifier of the image related to a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceContainerImageIds")]
    pub r#resource_container_image_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceContainerImageId>>>,
    /// The name of the image related to a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceContainerImageNames")]
    pub r#resource_container_image_names: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceContainerImageName>>>,
    /// The date/time that the container was started. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceContainerLaunchedAts")]
    pub r#resource_container_launched_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceContainerLaunchedAt>>>,
    /// The name of the container related to a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceContainerNames")]
    pub r#resource_container_names: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceContainerName>>>,
    /// The details of a resource that doesn't have a specific subfield for the resource type defined. See Map Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceDetailsOthers")]
    pub r#resource_details_others: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceDetailsOther>>>,
    /// The canonical identifier for the given resource type. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceIds")]
    pub r#resource_ids: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceId>>>,
    /// The canonical AWS partition name that the Region is assigned to. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourcePartitions")]
    pub r#resource_partitions: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourcePartition>>>,
    /// The canonical AWS external Region name where this resource is located. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceRegions")]
    pub r#resource_regions: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceRegion>>>,
    /// A list of AWS tags associated with a resource at the time the finding was processed. See Map Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceTags")]
    pub r#resource_tags: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceTag>>>,
    /// Specifies the type of the resource that details are provided for. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Option<Vec<super::super::types::securityhub::InsightFiltersResourceType>>>,
    /// The label of a finding's severity. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "severityLabels")]
    pub r#severity_labels: Box<Option<Vec<super::super::types::securityhub::InsightFiltersSeverityLabel>>>,
    /// A URL that links to a page about the current finding in the security-findings provider's solution. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "sourceUrls")]
    pub r#source_urls: Box<Option<Vec<super::super::types::securityhub::InsightFiltersSourceUrl>>>,
    /// The category of a threat intelligence indicator. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "threatIntelIndicatorCategories")]
    pub r#threat_intel_indicator_categories: Box<Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorCategory>>>,
    /// The date/time of the last observation of a threat intelligence indicator. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "threatIntelIndicatorLastObservedAts")]
    pub r#threat_intel_indicator_last_observed_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorLastObservedAt>>>,
    /// The URL for more details from the source of the threat intelligence. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "threatIntelIndicatorSourceUrls")]
    pub r#threat_intel_indicator_source_urls: Box<Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorSourceUrl>>>,
    /// The source of the threat intelligence. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "threatIntelIndicatorSources")]
    pub r#threat_intel_indicator_sources: Box<Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorSource>>>,
    /// The type of a threat intelligence indicator. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "threatIntelIndicatorTypes")]
    pub r#threat_intel_indicator_types: Box<Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorType>>>,
    /// The value of a threat intelligence indicator. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "threatIntelIndicatorValues")]
    pub r#threat_intel_indicator_values: Box<Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorValue>>>,
    /// A finding's title. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "titles")]
    pub r#titles: Box<Option<Vec<super::super::types::securityhub::InsightFiltersTitle>>>,
    /// A finding type in the format of `namespace/category/classifier` that classifies a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "types")]
    pub r#types: Box<Option<Vec<super::super::types::securityhub::InsightFiltersType>>>,
    /// An ISO8601-formatted timestamp that indicates when the security-findings provider last updated the finding record. See Date Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "updatedAts")]
    pub r#updated_ats: Box<Option<Vec<super::super::types::securityhub::InsightFiltersUpdatedAt>>>,
    /// A list of name/value string pairs associated with the finding. These are custom, user-defined fields added to a finding. See Map Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "userDefinedValues")]
    pub r#user_defined_values: Box<Option<Vec<super::super::types::securityhub::InsightFiltersUserDefinedValue>>>,
    /// The veracity of a finding. See String Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "verificationStates")]
    pub r#verification_states: Box<Option<Vec<super::super::types::securityhub::InsightFiltersVerificationState>>>,
    /// The status of the investigation into a finding. See Workflow Status Filter below for more details.
    #[builder(into, default)]
    #[serde(rename = "workflowStatuses")]
    pub r#workflow_statuses: Box<Option<Vec<super::super::types::securityhub::InsightFiltersWorkflowStatus>>>,
}
