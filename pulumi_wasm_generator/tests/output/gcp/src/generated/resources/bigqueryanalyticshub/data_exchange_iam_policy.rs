/// Three different resources help you manage your IAM policy for Bigquery Analytics Hub DataExchange. Each of these resources serves a different use case:
///
/// * `gcp.bigqueryanalyticshub.DataExchangeIamPolicy`: Authoritative. Sets the IAM policy for the dataexchange and replaces any existing policy already attached.
/// * `gcp.bigqueryanalyticshub.DataExchangeIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the dataexchange are preserved.
/// * `gcp.bigqueryanalyticshub.DataExchangeIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the dataexchange are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.bigqueryanalyticshub.DataExchangeIamPolicy`: Retrieves the IAM policy for the dataexchange
///
/// > **Note:** `gcp.bigqueryanalyticshub.DataExchangeIamPolicy` **cannot** be used in conjunction with `gcp.bigqueryanalyticshub.DataExchangeIamBinding` and `gcp.bigqueryanalyticshub.DataExchangeIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.bigqueryanalyticshub.DataExchangeIamBinding` resources **can be** used in conjunction with `gcp.bigqueryanalyticshub.DataExchangeIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.bigqueryanalyticshub.DataExchangeIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:bigqueryanalyticshub:DataExchangeIamPolicy
///     properties:
///       project: ${dataExchange.project}
///       location: ${dataExchange.location}
///       dataExchangeId: ${dataExchange.dataExchangeId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigqueryanalyticshub.DataExchangeIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = data_exchange_iam_binding::create(
///         "binding",
///         DataExchangeIamBindingArgs::builder()
///             .data_exchange_id("${dataExchange.dataExchangeId}")
///             .location("${dataExchange.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${dataExchange.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigqueryanalyticshub.DataExchangeIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = data_exchange_iam_member::create(
///         "member",
///         DataExchangeIamMemberArgs::builder()
///             .data_exchange_id("${dataExchange.dataExchangeId}")
///             .location("${dataExchange.location}")
///             .member("user:jane@example.com")
///             .project("${dataExchange.project}")
///             .role("roles/viewer")
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
/// # IAM policy for Bigquery Analytics Hub DataExchange
/// Three different resources help you manage your IAM policy for Bigquery Analytics Hub DataExchange. Each of these resources serves a different use case:
///
/// * `gcp.bigqueryanalyticshub.DataExchangeIamPolicy`: Authoritative. Sets the IAM policy for the dataexchange and replaces any existing policy already attached.
/// * `gcp.bigqueryanalyticshub.DataExchangeIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the dataexchange are preserved.
/// * `gcp.bigqueryanalyticshub.DataExchangeIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the dataexchange are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.bigqueryanalyticshub.DataExchangeIamPolicy`: Retrieves the IAM policy for the dataexchange
///
/// > **Note:** `gcp.bigqueryanalyticshub.DataExchangeIamPolicy` **cannot** be used in conjunction with `gcp.bigqueryanalyticshub.DataExchangeIamBinding` and `gcp.bigqueryanalyticshub.DataExchangeIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.bigqueryanalyticshub.DataExchangeIamBinding` resources **can be** used in conjunction with `gcp.bigqueryanalyticshub.DataExchangeIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.bigqueryanalyticshub.DataExchangeIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:bigqueryanalyticshub:DataExchangeIamPolicy
///     properties:
///       project: ${dataExchange.project}
///       location: ${dataExchange.location}
///       dataExchangeId: ${dataExchange.dataExchangeId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigqueryanalyticshub.DataExchangeIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = data_exchange_iam_binding::create(
///         "binding",
///         DataExchangeIamBindingArgs::builder()
///             .data_exchange_id("${dataExchange.dataExchangeId}")
///             .location("${dataExchange.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${dataExchange.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigqueryanalyticshub.DataExchangeIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = data_exchange_iam_member::create(
///         "member",
///         DataExchangeIamMemberArgs::builder()
///             .data_exchange_id("${dataExchange.dataExchangeId}")
///             .location("${dataExchange.location}")
///             .member("user:jane@example.com")
///             .project("${dataExchange.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/dataExchanges/{{data_exchange_id}}
///
/// * {{project}}/{{location}}/{{data_exchange_id}}
///
/// * {{location}}/{{data_exchange_id}}
///
/// * {{data_exchange_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Bigquery Analytics Hub dataexchange IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/dataExchangeIamPolicy:DataExchangeIamPolicy editor "projects/{{project}}/locations/{{location}}/dataExchanges/{{data_exchange_id}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/dataExchangeIamPolicy:DataExchangeIamPolicy editor "projects/{{project}}/locations/{{location}}/dataExchanges/{{data_exchange_id}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/dataExchangeIamPolicy:DataExchangeIamPolicy editor projects/{{project}}/locations/{{location}}/dataExchanges/{{data_exchange_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod data_exchange_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataExchangeIamPolicyArgs {
        /// The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces. Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub data_exchange_id: pulumi_wasm_rust::Output<String>,
        /// The name of the location this data exchange.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataExchangeIamPolicyResult {
        /// The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces. Used to find the parent resource to bind the IAM policy to
        pub data_exchange_id: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The name of the location this data exchange.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DataExchangeIamPolicyArgs,
    ) -> DataExchangeIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_exchange_id_binding = args.data_exchange_id.get_inner();
        let location_binding = args.location.get_inner();
        let policy_data_binding = args.policy_data.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigqueryanalyticshub/dataExchangeIamPolicy:DataExchangeIamPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataExchangeId".into(),
                    value: &data_exchange_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "dataExchangeId".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
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
        DataExchangeIamPolicyResult {
            data_exchange_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataExchangeId").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
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
