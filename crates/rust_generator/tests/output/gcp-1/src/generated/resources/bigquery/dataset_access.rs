/// ## Example Usage
///
/// ### Bigquery Dataset Access Basic User
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let access = dataset_access::create(
///         "access",
///         DatasetAccessArgs::builder()
///             .dataset_id("${dataset.datasetId}")
///             .role("OWNER")
///             .user_by_email("${bqowner.email}")
///             .build_struct(),
///     );
///     let bqowner = account::create(
///         "bqowner",
///         AccountArgs::builder().account_id("bqowner").build_struct(),
///     );
///     let dataset = dataset::create(
///         "dataset",
///         DatasetArgs::builder().dataset_id("example_dataset").build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Dataset Access View
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let access = dataset_access::create(
///         "access",
///         DatasetAccessArgs::builder()
///             .dataset_id("${private.datasetId}")
///             .view(
///                 DatasetAccessView::builder()
///                     .datasetId("${public.datasetId}")
///                     .projectId("${publicTable.project}")
///                     .tableId("${publicTable.tableId}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let private = dataset::create(
///         "private",
///         DatasetArgs::builder().dataset_id("example_dataset").build_struct(),
///     );
///     let public = dataset::create(
///         "public",
///         DatasetArgs::builder().dataset_id("example_dataset2").build_struct(),
///     );
///     let publicTable = table::create(
///         "publicTable",
///         TableArgs::builder()
///             .dataset_id("${public.datasetId}")
///             .deletion_protection(false)
///             .table_id("example_table")
///             .view(
///                 TableView::builder()
///                     .query("SELECT state FROM [lookerdata:cdc.project_tycho_reports]")
///                     .useLegacySql(false)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Dataset Access Authorized Dataset
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let access = dataset_access::create(
///         "access",
///         DatasetAccessArgs::builder()
///             .authorized_dataset(
///                 DatasetAccessAuthorizedDataset::builder()
///                     .dataset(
///                         DatasetAccessAuthorizedDatasetDataset::builder()
///                             .datasetId("${public.datasetId}")
///                             .projectId("${public.project}")
///                             .build_struct(),
///                     )
///                     .targetTypes(vec!["VIEWS",])
///                     .build_struct(),
///             )
///             .dataset_id("${private.datasetId}")
///             .build_struct(),
///     );
///     let private = dataset::create(
///         "private",
///         DatasetArgs::builder().dataset_id("private").build_struct(),
///     );
///     let public = dataset::create(
///         "public",
///         DatasetArgs::builder().dataset_id("public").build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Dataset Access Authorized Routine
///
///
/// ```yaml
/// resources:
///   public:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: public_dataset
///       description: This dataset is public
///   publicRoutine:
///     type: gcp:bigquery:Routine
///     name: public
///     properties:
///       datasetId: ${public.datasetId}
///       routineId: public_routine
///       routineType: TABLE_VALUED_FUNCTION
///       language: SQL
///       definitionBody: |
///         SELECT 1 + value AS value
///       arguments:
///         - name: value
///           argumentKind: FIXED_TYPE
///           dataType:
///             fn::toJSON:
///               typeKind: INT64
///       returnTableType:
///         fn::toJSON:
///           columns:
///             - name: value
///               type:
///                 typeKind: INT64
///   private:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: private_dataset
///       description: This dataset is private
///   authorizedRoutine:
///     type: gcp:bigquery:DatasetAccess
///     name: authorized_routine
///     properties:
///       datasetId: ${private.datasetId}
///       routine:
///         projectId: ${publicRoutine.project}
///         datasetId: ${publicRoutine.datasetId}
///         routineId: ${publicRoutine.routineId}
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetAccessArgs {
        /// Grants all resources of particular types in a particular dataset read access to the current dataset.
        /// Structure is documented below.
        #[builder(into, default)]
        pub authorized_dataset: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::DatasetAccessAuthorizedDataset>,
        >,
        /// A unique ID for this dataset, without the project name. The ID
        /// must contain only letters (a-z, A-Z), numbers (0-9), or
        /// underscores (_). The maximum length is 1,024 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub dataset_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A domain to grant access to. Any users signed in with the
        /// domain specified will be granted the specified access
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An email address of a Google Group to grant access to.
        #[builder(into, default)]
        pub group_by_email: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Some other type of member that appears in the IAM Policy but isn't a user,
        /// group, domain, or special group. For example: `allUsers`
        #[builder(into, default)]
        pub iam_member: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Describes the rights granted to the user specified by the other
        /// member of the access object. Basic, predefined, and custom roles are
        /// supported. Predefined roles that have equivalent basic roles are
        /// swapped by the API to their basic counterparts, and will show a diff
        /// post-create. See
        /// [official docs](https://cloud.google.com/bigquery/docs/access-control).
        #[builder(into, default)]
        pub role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A routine from a different dataset to grant access to. Queries
        /// executed against that routine will have read access to tables in
        /// this dataset. The role field is not required when this field is
        /// set. If that routine is updated by any user, access to the routine
        /// needs to be granted again via an update operation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub routine: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::DatasetAccessRoutine>,
        >,
        /// A special group to grant access to. Possible values include:
        /// * `projectOwners`: Owners of the enclosing project.
        /// * `projectReaders`: Readers of the enclosing project.
        /// * `projectWriters`: Writers of the enclosing project.
        /// * `allAuthenticatedUsers`: All authenticated BigQuery users.
        #[builder(into, default)]
        pub special_group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An email address of a user to grant access to. For example:
        /// fred@example.com
        #[builder(into, default)]
        pub user_by_email: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A view from a different dataset to grant access to. Queries
        /// executed against that view will have read access to tables in
        /// this dataset. The role field is not required when this field is
        /// set. If that view is updated by any user, access to the view
        /// needs to be granted again via an update operation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub view: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::DatasetAccessView>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatasetAccessResult {
        /// If true, represents that that the iam_member in the config was translated to a different member type by the API, and is
        /// stored in state as a different member type
        pub api_updated_member: pulumi_gestalt_rust::Output<bool>,
        /// Grants all resources of particular types in a particular dataset read access to the current dataset.
        /// Structure is documented below.
        pub authorized_dataset: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::DatasetAccessAuthorizedDataset>,
        >,
        /// A unique ID for this dataset, without the project name. The ID
        /// must contain only letters (a-z, A-Z), numbers (0-9), or
        /// underscores (_). The maximum length is 1,024 characters.
        ///
        ///
        /// - - -
        pub dataset_id: pulumi_gestalt_rust::Output<String>,
        /// A domain to grant access to. Any users signed in with the
        /// domain specified will be granted the specified access
        pub domain: pulumi_gestalt_rust::Output<Option<String>>,
        /// An email address of a Google Group to grant access to.
        pub group_by_email: pulumi_gestalt_rust::Output<Option<String>>,
        /// Some other type of member that appears in the IAM Policy but isn't a user,
        /// group, domain, or special group. For example: `allUsers`
        pub iam_member: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Describes the rights granted to the user specified by the other
        /// member of the access object. Basic, predefined, and custom roles are
        /// supported. Predefined roles that have equivalent basic roles are
        /// swapped by the API to their basic counterparts, and will show a diff
        /// post-create. See
        /// [official docs](https://cloud.google.com/bigquery/docs/access-control).
        pub role: pulumi_gestalt_rust::Output<Option<String>>,
        /// A routine from a different dataset to grant access to. Queries
        /// executed against that routine will have read access to tables in
        /// this dataset. The role field is not required when this field is
        /// set. If that routine is updated by any user, access to the routine
        /// needs to be granted again via an update operation.
        /// Structure is documented below.
        pub routine: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::DatasetAccessRoutine>,
        >,
        /// A special group to grant access to. Possible values include:
        /// * `projectOwners`: Owners of the enclosing project.
        /// * `projectReaders`: Readers of the enclosing project.
        /// * `projectWriters`: Writers of the enclosing project.
        /// * `allAuthenticatedUsers`: All authenticated BigQuery users.
        pub special_group: pulumi_gestalt_rust::Output<Option<String>>,
        /// An email address of a user to grant access to. For example:
        /// fred@example.com
        pub user_by_email: pulumi_gestalt_rust::Output<Option<String>>,
        /// A view from a different dataset to grant access to. Queries
        /// executed against that view will have read access to tables in
        /// this dataset. The role field is not required when this field is
        /// set. If that view is updated by any user, access to the view
        /// needs to be granted again via an update operation.
        /// Structure is documented below.
        pub view: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::DatasetAccessView>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatasetAccessArgs,
    ) -> DatasetAccessResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorized_dataset_binding = args.authorized_dataset.get_output(context);
        let dataset_id_binding = args.dataset_id.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let group_by_email_binding = args.group_by_email.get_output(context);
        let iam_member_binding = args.iam_member.get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let routine_binding = args.routine.get_output(context);
        let special_group_binding = args.special_group.get_output(context);
        let user_by_email_binding = args.user_by_email.get_output(context);
        let view_binding = args.view.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigquery/datasetAccess:DatasetAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizedDataset".into(),
                    value: authorized_dataset_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datasetId".into(),
                    value: dataset_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupByEmail".into(),
                    value: group_by_email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamMember".into(),
                    value: iam_member_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routine".into(),
                    value: routine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "specialGroup".into(),
                    value: special_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userByEmail".into(),
                    value: user_by_email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "view".into(),
                    value: view_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatasetAccessResult {
            api_updated_member: o.get_field("apiUpdatedMember"),
            authorized_dataset: o.get_field("authorizedDataset"),
            dataset_id: o.get_field("datasetId"),
            domain: o.get_field("domain"),
            group_by_email: o.get_field("groupByEmail"),
            iam_member: o.get_field("iamMember"),
            project: o.get_field("project"),
            role: o.get_field("role"),
            routine: o.get_field("routine"),
            special_group: o.get_field("specialGroup"),
            user_by_email: o.get_field("userByEmail"),
            view: o.get_field("view"),
        }
    }
}
