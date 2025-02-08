#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDataCollectionRuleDataSourceLogFileSettingText {
    /// The timestamp format of the text log files. Possible values are `ISO 8601`, `YYYY-MM-DD HH:MM:SS`, `M/D/YYYY HH:MM:SS AM/PM`, `Mon DD, YYYY HH:MM:SS`, `yyMMdd HH:mm:ss`, `ddMMyy HH:mm:ss`, `MMM d hh:mm:ss`, `dd/MMM/yyyy:HH:mm:ss zzz`,and `yyyy-MM-ddTHH:mm:ssK`.
    #[builder(into)]
    #[serde(rename = "recordStartTimestampFormat")]
    pub r#record_start_timestamp_format: Box<String>,
}
