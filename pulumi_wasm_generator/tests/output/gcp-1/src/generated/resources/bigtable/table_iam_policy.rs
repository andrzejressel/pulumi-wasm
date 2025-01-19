/// Three different resources help you manage IAM policies on bigtable tables. Each of these resources serves a different use case:
///
/// * `gcp.bigtable.TableIamPolicy`: Authoritative. Sets the IAM policy for the tables and replaces any existing policy already attached.
/// * `gcp.bigtable.TableIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the table are preserved.
/// * `gcp.bigtable.TableIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the table are preserved.
///
/// > **Note:** `gcp.bigtable.TableIamPolicy` **cannot** be used in conjunction with `gcp.bigtable.TableIamBinding` and `gcp.bigtable.TableIamMember` or they will fight over what your policy should be. In addition, be careful not to accidentally unset ownership of the table as `gcp.bigtable.TableIamPolicy` replaces the entire policy.
///
/// > **Note:** `gcp.bigtable.TableIamBinding` resources **can be** used in conjunction with `gcp.bigtable.TableIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.bigtable.TableIamPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:bigtable:TableIamPolicy
///     properties:
///       project: your-project
///       instance: your-bigtable-instance
///       table: your-bigtable-table
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigtable.user
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigtable.TableIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = table_iam_binding::create(
///         "editor",
///         TableIamBindingArgs::builder()
///             .instance("your-bigtable-instance")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/bigtable.user")
///             .table("your-bigtable-table")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigtable.TableIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = table_iam_member::create(
///         "editor",
///         TableIamMemberArgs::builder()
///             .instance("your-bigtable-instance")
///             .member("user:jane@example.com")
///             .role("roles/bigtable.user")
///             .table("your-bigtable-table")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigtable.TableIamPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:bigtable:TableIamPolicy
///     properties:
///       project: your-project
///       instance: your-bigtable-instance
///       table: your-bigtable-table
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigtable.user
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigtable.TableIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = table_iam_binding::create(
///         "editor",
///         TableIamBindingArgs::builder()
///             .instance("your-bigtable-instance")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/bigtable.user")
///             .table("your-bigtable-table")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigtable.TableIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = table_iam_member::create(
///         "editor",
///         TableIamMemberArgs::builder()
///             .instance("your-bigtable-instance")
///             .member("user:jane@example.com")
///             .role("roles/bigtable.user")
///             .table("your-bigtable-table")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the `table` identifier of the Bigtable Table resource only. For example:
///
/// * `"projects/{project}/instances/{instance}/tables/{table}"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "projects/{project}/instances/{instance}/tables/{table}"
///
///   to = google_bigtable_table_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:bigtable/tableIamPolicy:TableIamPolicy default projects/{project}/instances/{instance}/tables/{table}
/// ```
///
pub mod table_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableIamPolicyArgs {
        /// The name or relative resource id of the instance that owns the table.
        #[builder(into)]
        pub instance: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by a `gcp.organizations.getIAMPolicy` data source.
        ///
        /// - - -
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The project in which the table belongs. If it
        /// is not provided, this provider will use the provider default.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The name or relative resource id of the table to manage IAM policies for.
        ///
        /// For `gcp.bigtable.TableIamMember` or `gcp.bigtable.TableIamBinding`:
        #[builder(into)]
        pub table: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TableIamPolicyResult {
        /// (Computed) The etag of the tables's IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The name or relative resource id of the instance that owns the table.
        pub instance: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by a `gcp.organizations.getIAMPolicy` data source.
        ///
        /// - - -
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The project in which the table belongs. If it
        /// is not provided, this provider will use the provider default.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The name or relative resource id of the table to manage IAM policies for.
        ///
        /// For `gcp.bigtable.TableIamMember` or `gcp.bigtable.TableIamBinding`:
        pub table: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TableIamPolicyArgs) -> TableIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_binding = args.instance.get_inner();
        let policy_data_binding = args.policy_data.get_inner();
        let project_binding = args.project.get_inner();
        let table_binding = args.table.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigtable/tableIamPolicy:TableIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
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
                    name: "table".into(),
                    value: &table_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "instance".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "table".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TableIamPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instance").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            table: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("table").unwrap(),
            ),
        }
    }
}
