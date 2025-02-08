#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccessLevelsAccessLevelCustom {
    /// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language.
    /// This page details the objects and attributes that are used to the build the CEL expressions for
    /// custom access levels - https://cloud.google.com/access-context-manager/docs/custom-access-level-spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "expr")]
    pub r#expr: Box<super::super::types::accesscontextmanager::AccessLevelsAccessLevelCustomExpr>,
}
