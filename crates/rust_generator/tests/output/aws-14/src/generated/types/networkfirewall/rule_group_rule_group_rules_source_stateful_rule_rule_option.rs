#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RuleGroupRuleGroupRulesSourceStatefulRuleRuleOption {
    /// Keyword defined by open source detection systems like Snort or Suricata for stateful rule inspection.
    /// See [Snort General Rule Options](http://manual-snort-org.s3-website-us-east-1.amazonaws.com/node31.html) or [Suricata Rule Options](https://suricata.readthedocs.io/en/suricata-5.0.1/rules/intro.html#rule-options) for more details.
    #[builder(into)]
    #[serde(rename = "keyword")]
    pub r#keyword: Box<String>,
    /// Set of strings for additional settings to use in stateful rule inspection.
    #[builder(into, default)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<Vec<String>>>,
}
