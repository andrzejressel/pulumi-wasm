#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBucketLifecycleRuleCondition {
    /// Minimum age of an object in days to satisfy this condition.
    #[builder(into)]
    #[serde(rename = "age")]
    pub r#age: Box<i32>,
    /// Creation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition.
    #[builder(into)]
    #[serde(rename = "createdBefore")]
    pub r#created_before: Box<String>,
    /// Creation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition.
    #[builder(into)]
    #[serde(rename = "customTimeBefore")]
    pub r#custom_time_before: Box<String>,
    /// Number of days elapsed since the user-specified timestamp set on an object.
    #[builder(into)]
    #[serde(rename = "daysSinceCustomTime")]
    pub r#days_since_custom_time: Box<i32>,
    /// Number of days elapsed since the noncurrent timestamp of an object. This
    /// condition is relevant only for versioned objects.
    #[builder(into)]
    #[serde(rename = "daysSinceNoncurrentTime")]
    pub r#days_since_noncurrent_time: Box<i32>,
    /// One or more matching name prefixes to satisfy this condition.
    #[builder(into)]
    #[serde(rename = "matchesPrefixes")]
    pub r#matches_prefixes: Box<Vec<String>>,
    /// Storage Class of objects to satisfy this condition. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE, STANDARD, DURABLE_REDUCED_AVAILABILITY.
    #[builder(into)]
    #[serde(rename = "matchesStorageClasses")]
    pub r#matches_storage_classes: Box<Vec<String>>,
    /// One or more matching name suffixes to satisfy this condition.
    #[builder(into)]
    #[serde(rename = "matchesSuffixes")]
    pub r#matches_suffixes: Box<Vec<String>>,
    /// Creation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition.
    #[builder(into)]
    #[serde(rename = "noncurrentTimeBefore")]
    pub r#noncurrent_time_before: Box<String>,
    /// Relevant only for versioned objects. The number of newer versions of an object to satisfy this condition.
    #[builder(into)]
    #[serde(rename = "numNewerVersions")]
    pub r#num_newer_versions: Box<i32>,
    /// While set true, age value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the age field. It can be used alone or together with age.
    #[builder(into)]
    #[serde(rename = "sendAgeIfZero")]
    pub r#send_age_if_zero: Box<bool>,
    /// While set true, days_since_custom_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_custom_time field. It can be used alone or together with days_since_custom_time.
    #[builder(into)]
    #[serde(rename = "sendDaysSinceCustomTimeIfZero")]
    pub r#send_days_since_custom_time_if_zero: Box<bool>,
    /// While set true, days_since_noncurrent_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_noncurrent_time field. It can be used alone or together with days_since_noncurrent_time.
    #[builder(into)]
    #[serde(rename = "sendDaysSinceNoncurrentTimeIfZero")]
    pub r#send_days_since_noncurrent_time_if_zero: Box<bool>,
    /// While set true, num_newer_versions value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the num_newer_versions field. It can be used alone or together with num_newer_versions.
    #[builder(into)]
    #[serde(rename = "sendNumNewerVersionsIfZero")]
    pub r#send_num_newer_versions_if_zero: Box<bool>,
    /// Match to live and/or archived objects. Unversioned buckets have only live objects. Supported values include: "LIVE", "ARCHIVED", "ANY".
    #[builder(into)]
    #[serde(rename = "withState")]
    pub r#with_state: Box<String>,
}
