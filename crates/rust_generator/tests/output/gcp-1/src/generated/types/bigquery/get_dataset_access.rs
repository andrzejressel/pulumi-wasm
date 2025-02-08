#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDatasetAccess {
    /// Grants all resources of particular types in a particular dataset read access to the current dataset.
    #[builder(into)]
    #[serde(rename = "datasets")]
    pub r#datasets: Box<Vec<super::super::types::bigquery::GetDatasetAccessDataset>>,
    /// A domain to grant access to. Any users signed in with the
    /// domain specified will be granted the specified access
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Box<String>,
    /// An email address of a Google Group to grant access to.
    #[builder(into)]
    #[serde(rename = "groupByEmail")]
    pub r#group_by_email: Box<String>,
    /// Some other type of member that appears in the IAM Policy but isn't a user,
    /// group, domain, or special group. For example: 'allUsers'
    #[builder(into)]
    #[serde(rename = "iamMember")]
    pub r#iam_member: Box<String>,
    /// Describes the rights granted to the user specified by the other
    /// member of the access object. Basic, predefined, and custom roles
    /// are supported. Predefined roles that have equivalent basic roles
    /// are swapped by the API to their basic counterparts. See
    /// [official docs](https://cloud.google.com/bigquery/docs/access-control).
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: Box<String>,
    /// A routine from a different dataset to grant access to. Queries
    /// executed against that routine will have read access to tables in
    /// this dataset. The role field is not required when this field is
    /// set. If that routine is updated by any user, access to the routine
    /// needs to be granted again via an update operation.
    #[builder(into)]
    #[serde(rename = "routines")]
    pub r#routines: Box<Vec<super::super::types::bigquery::GetDatasetAccessRoutine>>,
    /// A special group to grant access to. Possible values include:
    /// * 'projectOwners': Owners of the enclosing project.
    /// * 'projectReaders': Readers of the enclosing project.
    /// * 'projectWriters': Writers of the enclosing project.
    /// * 'allAuthenticatedUsers': All authenticated BigQuery users.
    #[builder(into)]
    #[serde(rename = "specialGroup")]
    pub r#special_group: Box<String>,
    /// An email address of a user to grant access to. For example:
    /// fred@example.com
    #[builder(into)]
    #[serde(rename = "userByEmail")]
    pub r#user_by_email: Box<String>,
    /// A view from a different dataset to grant access to. Queries
    /// executed against that view will have read access to tables in
    /// this dataset. The role field is not required when this field is
    /// set. If that view is updated by any user, access to the view
    /// needs to be granted again via an update operation.
    #[builder(into)]
    #[serde(rename = "views")]
    pub r#views: Box<Vec<super::super::types::bigquery::GetDatasetAccessView>>,
}
