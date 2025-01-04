/// Three different resources help you manage your IAM policy for a Spanner database. Each of these resources serves a different use case:
///
/// * `gcp.spanner.DatabaseIAMPolicy`: Authoritative. Sets the IAM policy for the database and replaces any existing policy already attached.
///
/// > **Warning:** It's entirely possibly to lock yourself out of your database using `gcp.spanner.DatabaseIAMPolicy`. Any permissions granted by default will be removed unless you include them in your config.
///
/// * `gcp.spanner.DatabaseIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the database are preserved.
/// * `gcp.spanner.DatabaseIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the database are preserved.
///
/// > **Note:** `gcp.spanner.DatabaseIAMPolicy` **cannot** be used in conjunction with `gcp.spanner.DatabaseIAMBinding` and `gcp.spanner.DatabaseIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.spanner.DatabaseIAMBinding` resources **can be** used in conjunction with `gcp.spanner.DatabaseIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.spanner.DatabaseIAMPolicy
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:spanner:DatabaseIAMPolicy
///     properties:
///       instance: your-instance-name
///       database: your-database-name
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/editor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:spanner:DatabaseIAMPolicy
///     properties:
///       instance: your-instance-name
///       database: your-database-name
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/editor
///             members:
///               - user:jane@example.com
///             condition:
///               title: My Role
///               description: Grant permissions on my_role
///               expression: (resource.type == "spanner.googleapis.com/DatabaseRole" && (resource.name.endsWith("/myrole")))
/// ```
///
/// ## gcp.spanner.DatabaseIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database_iam_binding::create(
///         "database",
///         DatabaseIamBindingArgs::builder()
///             .database("your-database-name")
///             .instance("your-instance-name")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/compute.networkUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database_iam_binding::create(
///         "database",
///         DatabaseIamBindingArgs::builder()
///             .condition(
///                 DatabaseIamBindingCondition::builder()
///                     .description("Grant permissions on my_role")
///                     .expression(
///                         "(resource.type == \"spanner.googleapis.com/DatabaseRole\" && (resource.name.endsWith(\"/myrole\")))",
///                     )
///                     .title("My Role")
///                     .build_struct(),
///             )
///             .database("your-database-name")
///             .instance("your-instance-name")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/compute.networkUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.spanner.DatabaseIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database_iam_member::create(
///         "database",
///         DatabaseIamMemberArgs::builder()
///             .database("your-database-name")
///             .instance("your-instance-name")
///             .member("user:jane@example.com")
///             .role("roles/compute.networkUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database_iam_member::create(
///         "database",
///         DatabaseIamMemberArgs::builder()
///             .condition(
///                 DatabaseIamMemberCondition::builder()
///                     .description("Grant permissions on my_role")
///                     .expression(
///                         "(resource.type == \"spanner.googleapis.com/DatabaseRole\" && (resource.name.endsWith(\"/myrole\")))",
///                     )
///                     .title("My Role")
///                     .build_struct(),
///             )
///             .database("your-database-name")
///             .instance("your-instance-name")
///             .member("user:jane@example.com")
///             .role("roles/compute.networkUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.spanner.DatabaseIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database_iam_binding::create(
///         "database",
///         DatabaseIamBindingArgs::builder()
///             .database("your-database-name")
///             .instance("your-instance-name")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/compute.networkUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database_iam_binding::create(
///         "database",
///         DatabaseIamBindingArgs::builder()
///             .condition(
///                 DatabaseIamBindingCondition::builder()
///                     .description("Grant permissions on my_role")
///                     .expression(
///                         "(resource.type == \"spanner.googleapis.com/DatabaseRole\" && (resource.name.endsWith(\"/myrole\")))",
///                     )
///                     .title("My Role")
///                     .build_struct(),
///             )
///             .database("your-database-name")
///             .instance("your-instance-name")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/compute.networkUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.spanner.DatabaseIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database_iam_member::create(
///         "database",
///         DatabaseIamMemberArgs::builder()
///             .database("your-database-name")
///             .instance("your-instance-name")
///             .member("user:jane@example.com")
///             .role("roles/compute.networkUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database_iam_member::create(
///         "database",
///         DatabaseIamMemberArgs::builder()
///             .condition(
///                 DatabaseIamMemberCondition::builder()
///                     .description("Grant permissions on my_role")
///                     .expression(
///                         "(resource.type == \"spanner.googleapis.com/DatabaseRole\" && (resource.name.endsWith(\"/myrole\")))",
///                     )
///                     .title("My Role")
///                     .build_struct(),
///             )
///             .database("your-database-name")
///             .instance("your-instance-name")
///             .member("user:jane@example.com")
///             .role("roles/compute.networkUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the identifier of the Spanner Database resource in question. For example:
///
/// * `{{project}}/{{instance}}/{{database}}`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = {{project}}/{{instance}}/{{database}}
///
///   to = google_spanner_database_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:spanner/databaseIAMPolicy:DatabaseIAMPolicy default {{project}}/{{instance}}/{{database}}
/// ```
///
pub mod database_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseIAMPolicyArgs {
        /// The name of the Spanner database.
        #[builder(into)]
        pub database: pulumi_wasm_rust::Output<String>,
        /// The name of the Spanner instance the database belongs to.
        #[builder(into)]
        pub instance: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatabaseIAMPolicyResult {
        /// The name of the Spanner database.
        pub database: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the database's IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The name of the Spanner instance the database belongs to.
        pub instance: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DatabaseIAMPolicyArgs) -> DatabaseIAMPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_binding = args.database.get_inner();
        let instance_binding = args.instance.get_inner();
        let policy_data_binding = args.policy_data.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:spanner/databaseIAMPolicy:DatabaseIAMPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "database".into(),
                },
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatabaseIAMPolicyResult {
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
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
        }
    }
}
