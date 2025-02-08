#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatascanDataQualitySpecRule {
    /// The unnested column which this rule is evaluated against.
    #[builder(into, default)]
    #[serde(rename = "column")]
    pub r#column: Box<Option<String>>,
    /// Description of the rule.
    /// The maximum length is 1,024 characters.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The dimension a rule belongs to. Results are also aggregated at the dimension level. Supported dimensions are ["COMPLETENESS", "ACCURACY", "CONSISTENCY", "VALIDITY", "UNIQUENESS", "INTEGRITY"]
    #[builder(into)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<String>,
    /// Rows with null values will automatically fail a rule, unless ignoreNull is true. In that case, such null rows are trivially considered passing. Only applicable to ColumnMap rules.
    #[builder(into, default)]
    #[serde(rename = "ignoreNull")]
    pub r#ignore_null: Box<Option<bool>>,
    /// A mutable name for the rule.
    /// The name must contain only letters (a-z, A-Z), numbers (0-9), or hyphens (-).
    /// The maximum length is 63 characters.
    /// Must start with a letter.
    /// Must end with a number or a letter.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// ColumnMap rule which evaluates whether each column value is null.
    #[builder(into, default)]
    #[serde(rename = "nonNullExpectation")]
    pub r#non_null_expectation: Box<Option<super::super::types::dataplex::DatascanDataQualitySpecRuleNonNullExpectation>>,
    /// ColumnMap rule which evaluates whether each column value lies between a specified range.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "rangeExpectation")]
    pub r#range_expectation: Box<Option<super::super::types::dataplex::DatascanDataQualitySpecRuleRangeExpectation>>,
    /// ColumnMap rule which evaluates whether each column value matches a specified regex.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "regexExpectation")]
    pub r#regex_expectation: Box<Option<super::super::types::dataplex::DatascanDataQualitySpecRuleRegexExpectation>>,
    /// Table rule which evaluates whether each row passes the specified condition.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "rowConditionExpectation")]
    pub r#row_condition_expectation: Box<Option<super::super::types::dataplex::DatascanDataQualitySpecRuleRowConditionExpectation>>,
    /// ColumnMap rule which evaluates whether each column value is contained by a specified set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "setExpectation")]
    pub r#set_expectation: Box<Option<super::super::types::dataplex::DatascanDataQualitySpecRuleSetExpectation>>,
    /// Table rule which evaluates whether any row matches invalid state.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sqlAssertion")]
    pub r#sql_assertion: Box<Option<super::super::types::dataplex::DatascanDataQualitySpecRuleSqlAssertion>>,
    /// ColumnAggregate rule which evaluates whether the column aggregate statistic lies between a specified range.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "statisticRangeExpectation")]
    pub r#statistic_range_expectation: Box<Option<super::super::types::dataplex::DatascanDataQualitySpecRuleStatisticRangeExpectation>>,
    /// Table rule which evaluates whether the provided expression is true.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tableConditionExpectation")]
    pub r#table_condition_expectation: Box<Option<super::super::types::dataplex::DatascanDataQualitySpecRuleTableConditionExpectation>>,
    /// The minimum ratio of passing_rows / total_rows required to pass this rule, with a range of [0.0, 1.0]. 0 indicates default value (i.e. 1.0).
    #[builder(into, default)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<Option<f64>>,
    /// Row-level rule which evaluates whether each column value is unique.
    #[builder(into, default)]
    #[serde(rename = "uniquenessExpectation")]
    pub r#uniqueness_expectation: Box<Option<super::super::types::dataplex::DatascanDataQualitySpecRuleUniquenessExpectation>>,
}
