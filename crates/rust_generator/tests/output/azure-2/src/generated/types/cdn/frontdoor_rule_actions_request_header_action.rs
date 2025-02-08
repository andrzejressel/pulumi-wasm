#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FrontdoorRuleActionsRequestHeaderAction {
    /// The action to be taken on the specified `header_name`. Possible values include `Append`, `Overwrite` or `Delete`.
    /// 
    /// > **NOTE:** `Append` causes the specified header to be added to the request with the specified value. If the header is already present, the value is appended to the existing header value using string concatenation. No delimiters are added. `Overwrite` causes specified header to be added to the request with the specified value. If the header is already present, the specified value overwrites the existing value. `Delete` causes the header to be deleted from the request.
    #[builder(into)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Box<String>,
    /// The name of the header to modify.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<String>,
    /// The value to append or overwrite.
    /// 
    /// ->**NOTE:** `value` is required if the `header_action` is set to `Append` or `Overwrite`.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
