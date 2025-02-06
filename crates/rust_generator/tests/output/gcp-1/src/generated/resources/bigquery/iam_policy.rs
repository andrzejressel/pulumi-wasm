/// Three different resources help you manage your IAM policy for BigQuery Table. Each of these resources serves a different use case:
///
/// * `gcp.bigquery.IamPolicy`: Authoritative. Sets the IAM policy for the table and replaces any existing policy already attached.
/// * `gcp.bigquery.IamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the table are preserved.
/// * `gcp.bigquery.IamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the table are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.bigquery.IamPolicy`: Retrieves the IAM policy for the table
///
/// > **Note:** `gcp.bigquery.IamPolicy` **cannot** be used in conjunction with `gcp.bigquery.IamBinding` and `gcp.bigquery.IamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.bigquery.IamBinding` resources **can be** used in conjunction with `gcp.bigquery.IamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.bigquery.IamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:bigquery:IamPolicy
///     properties:
///       project: ${test.project}
///       datasetId: ${test.datasetId}
///       tableId: ${test.tableId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigquery.dataOwner
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigquery.IamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = iam_binding::create(
///         "binding",
///         IamBindingArgs::builder()
///             .dataset_id("${test.datasetId}")
///             .members(vec!["user:jane@example.com",])
///             .project("${test.project}")
///             .role("roles/bigquery.dataOwner")
///             .table_id("${test.tableId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigquery.IamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = iam_member::create(
///         "member",
///         IamMemberArgs::builder()
///             .dataset_id("${test.datasetId}")
///             .member("user:jane@example.com")
///             .project("${test.project}")
///             .role("roles/bigquery.dataOwner")
///             .table_id("${test.tableId}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for BigQuery Table
/// Three different resources help you manage your IAM policy for BigQuery Table. Each of these resources serves a different use case:
///
/// * `gcp.bigquery.IamPolicy`: Authoritative. Sets the IAM policy for the table and replaces any existing policy already attached.
/// * `gcp.bigquery.IamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the table are preserved.
/// * `gcp.bigquery.IamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the table are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.bigquery.IamPolicy`: Retrieves the IAM policy for the table
///
/// > **Note:** `gcp.bigquery.IamPolicy` **cannot** be used in conjunction with `gcp.bigquery.IamBinding` and `gcp.bigquery.IamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.bigquery.IamBinding` resources **can be** used in conjunction with `gcp.bigquery.IamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.bigquery.IamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:bigquery:IamPolicy
///     properties:
///       project: ${test.project}
///       datasetId: ${test.datasetId}
///       tableId: ${test.tableId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigquery.dataOwner
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigquery.IamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = iam_binding::create(
///         "binding",
///         IamBindingArgs::builder()
///             .dataset_id("${test.datasetId}")
///             .members(vec!["user:jane@example.com",])
///             .project("${test.project}")
///             .role("roles/bigquery.dataOwner")
///             .table_id("${test.tableId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigquery.IamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = iam_member::create(
///         "member",
///         IamMemberArgs::builder()
///             .dataset_id("${test.datasetId}")
///             .member("user:jane@example.com")
///             .project("${test.project}")
///             .role("roles/bigquery.dataOwner")
///             .table_id("${test.tableId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}
///
/// * {{project}}/{{dataset_id}}/{{table_id}}
///
/// * {{dataset_id}}/{{table_id}}
///
/// * {{table_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// BigQuery table IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:bigquery/iamPolicy:IamPolicy editor "projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}} roles/bigquery.dataOwner user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:bigquery/iamPolicy:IamPolicy editor "projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}} roles/bigquery.dataOwner"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:bigquery/iamPolicy:IamPolicy editor projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IamPolicyArgs {
        #[builder(into)]
        pub dataset_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub table_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IamPolicyResult {
        pub dataset_id: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        pub table_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IamPolicyArgs,
    ) -> IamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dataset_id_binding = args.dataset_id.get_output(context).get_inner();
        let policy_data_binding = args.policy_data.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let table_id_binding = args.table_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/iamPolicy:IamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "datasetId".into(),
                    value: &dataset_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "tableId".into(),
                    value: &table_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IamPolicyResult {
            dataset_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("datasetId"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            table_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tableId"),
            ),
        }
    }
}
