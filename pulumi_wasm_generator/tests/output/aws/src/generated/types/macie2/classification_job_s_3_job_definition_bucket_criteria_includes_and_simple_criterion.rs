#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndSimpleCriterion {
    /// The operator to use in a condition. Valid combination of values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-jobcomparator)
    #[builder(into, default)]
    #[serde(rename = "comparator")]
    pub r#comparator: Box<Option<String>>,
    /// The object property to use in the condition. Valid combination of values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-simplecriterionkeyforjob)
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// An array that lists the values to use in the condition. Valid combination of values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-simplecriterionforjob)
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}