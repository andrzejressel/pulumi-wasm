#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataCollectionRuleDataSourcesSyslog {
    /// Specifies a list of facility names. Use a wildcard `*` to collect logs for all facility names. Possible values are `alert`, `*`, `audit`, `auth`, `authpriv`, `clock`, `cron`, `daemon`, `ftp`, `kern`, `local5`, `local4`, `local1`, `local7`, `local6`, `local3`, `local2`, `local0`, `lpr`, `mail`, `mark`, `news`, `nopri`, `ntp`, `syslog`, `user` and `uucp`.
    #[builder(into)]
    #[serde(rename = "facilityNames")]
    pub r#facility_names: Box<Vec<String>>,
    /// Specifies a list of log levels. Use a wildcard `*` to collect logs for all log levels. Possible values are `Debug`, `Info`, `Notice`, `Warning`, `Error`, `Critical`, `Alert`, `Emergency`,and `*`.
    #[builder(into)]
    #[serde(rename = "logLevels")]
    pub r#log_levels: Box<Vec<String>>,
    /// The name which should be used for this data source. This name should be unique across all data sources regardless of type within the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to. Possible values include but not limited to `Microsoft-Syslog`,and `Microsoft-CiscoAsa`, and `Microsoft-CommonSecurityLog`.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Box<Vec<String>>,
}
