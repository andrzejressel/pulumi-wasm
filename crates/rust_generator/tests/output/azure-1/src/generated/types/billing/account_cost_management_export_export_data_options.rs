#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountCostManagementExportExportDataOptions {
    /// The time frame for pulling data for the query. If custom, then a specific time period must be provided. Possible values include: `WeekToDate`, `MonthToDate`, `BillingMonthToDate`, `TheLast7Days`, `TheLastMonth`, `TheLastBillingMonth`, `Custom`.
    #[builder(into)]
    #[serde(rename = "timeFrame")]
    pub r#time_frame: Box<String>,
    /// The type of the query. Possible values are `ActualCost`, `AmortizedCost` and `Usage`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
