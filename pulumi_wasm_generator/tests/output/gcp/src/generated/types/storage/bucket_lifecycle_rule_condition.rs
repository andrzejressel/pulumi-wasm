#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLifecycleRuleCondition {
    /// Minimum age of an object in days to satisfy this condition. **Note** To set `0` value of `age`, `send_age_if_zero` should be set `true` otherwise `0` value of `age` field will be ignored.
    #[builder(into, default)]
    #[serde(rename = "age")]
    pub r#age: Box<Option<i32>>,
    /// A date in the RFC 3339 format YYYY-MM-DD. This condition is satisfied when an object is created before midnight of the specified date in UTC.
    #[builder(into, default)]
    #[serde(rename = "createdBefore")]
    pub r#created_before: Box<Option<String>>,
    /// A date in the RFC 3339 format YYYY-MM-DD. This condition is satisfied when the customTime metadata for the object is set to an earlier date than the date used in this lifecycle condition.
    #[builder(into, default)]
    #[serde(rename = "customTimeBefore")]
    pub r#custom_time_before: Box<Option<String>>,
    /// Number of days elapsed since the user-specified timestamp set on an object.
    #[builder(into, default)]
    #[serde(rename = "daysSinceCustomTime")]
    pub r#days_since_custom_time: Box<Option<i32>>,
    /// Number of days elapsed since the noncurrent timestamp of an object. This
    /// 										condition is relevant only for versioned objects.
    #[builder(into, default)]
    #[serde(rename = "daysSinceNoncurrentTime")]
    pub r#days_since_noncurrent_time: Box<Option<i32>>,
    /// One or more matching name prefixes to satisfy this condition.
    #[builder(into, default)]
    #[serde(rename = "matchesPrefixes")]
    pub r#matches_prefixes: Box<Option<Vec<String>>>,
    /// [Storage Class](https://cloud.google.com/storage/docs/storage-classes) of objects to satisfy this condition. Supported values include: `STANDARD`, `MULTI_REGIONAL`, `REGIONAL`, `NEARLINE`, `COLDLINE`, `ARCHIVE`, `DURABLE_REDUCED_AVAILABILITY`.
    #[builder(into, default)]
    #[serde(rename = "matchesStorageClasses")]
    pub r#matches_storage_classes: Box<Option<Vec<String>>>,
    /// One or more matching name suffixes to satisfy this condition.
    #[builder(into, default)]
    #[serde(rename = "matchesSuffixes")]
    pub r#matches_suffixes: Box<Option<Vec<String>>>,
    /// Creation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition.
    #[builder(into, default)]
    #[serde(rename = "noncurrentTimeBefore")]
    pub r#noncurrent_time_before: Box<Option<String>>,
    /// Relevant only for versioned objects. The number of newer versions of an object to satisfy this condition.
    #[builder(into, default)]
    #[serde(rename = "numNewerVersions")]
    pub r#num_newer_versions: Box<Option<i32>>,
    /// While set true, `age` value will be sent in the request even for zero value of the field. This field is only useful and required for setting 0 value to the `age` field. It can be used alone or together with `age` attribute. **NOTE** `age` attibute with `0` value will be ommitted from the API request if `send_age_if_zero` field is having `false` value.
    #[builder(into, default)]
    #[serde(rename = "sendAgeIfZero")]
    pub r#send_age_if_zero: Box<Option<bool>>,
    /// While set true, `days_since_custom_time` value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the `days_since_custom_time` field. It can be used alone or together with `days_since_custom_time`.
    #[builder(into, default)]
    #[serde(rename = "sendDaysSinceCustomTimeIfZero")]
    pub r#send_days_since_custom_time_if_zero: Box<Option<bool>>,
    /// While set true, `days_since_noncurrent_time` value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the `days_since_noncurrent_time` field. It can be used alone or together with `days_since_noncurrent_time`.
    #[builder(into, default)]
    #[serde(rename = "sendDaysSinceNoncurrentTimeIfZero")]
    pub r#send_days_since_noncurrent_time_if_zero: Box<Option<bool>>,
    /// While set true, `num_newer_versions` value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the `num_newer_versions` field. It can be used alone or together with `num_newer_versions`.
    #[builder(into, default)]
    #[serde(rename = "sendNumNewerVersionsIfZero")]
    pub r#send_num_newer_versions_if_zero: Box<Option<bool>>,
    /// Match to live and/or archived objects. Unversioned buckets have only live objects. Supported values include: `"LIVE"`, `"ARCHIVED"`, `"ANY"`.
    #[builder(into, default)]
    #[serde(rename = "withState")]
    pub r#with_state: Box<Option<String>>,
}
