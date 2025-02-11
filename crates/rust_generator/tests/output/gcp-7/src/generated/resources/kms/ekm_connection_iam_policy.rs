/// Three different resources help you manage your IAM policy for Cloud Key Management Service EkmConnection. Each of these resources serves a different use case:
///
/// * `gcp.kms.EkmConnectionIamPolicy`: Authoritative. Sets the IAM policy for the ekmconnection and replaces any existing policy already attached.
/// * `gcp.kms.EkmConnectionIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the ekmconnection are preserved.
/// * `gcp.kms.EkmConnectionIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the ekmconnection are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.kms.EkmConnectionIamPolicy`: Retrieves the IAM policy for the ekmconnection
///
/// > **Note:** `gcp.kms.EkmConnectionIamPolicy` **cannot** be used in conjunction with `gcp.kms.EkmConnectionIamBinding` and `gcp.kms.EkmConnectionIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.kms.EkmConnectionIamBinding` resources **can be** used in conjunction with `gcp.kms.EkmConnectionIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.kms.EkmConnectionIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:kms:EkmConnectionIamPolicy
///     properties:
///       project: ${["example-ekmconnection"].project}
///       location: ${["example-ekmconnection"].location}
///       name: ${["example-ekmconnection"].name}
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
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:kms:EkmConnectionIamPolicy
///     properties:
///       project: ${["example-ekmconnection"].project}
///       location: ${["example-ekmconnection"].location}
///       name: ${["example-ekmconnection"].name}
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
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.kms.EkmConnectionIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = ekm_connection_iam_binding::create(
///         "binding",
///         EkmConnectionIamBindingArgs::builder()
///             .location("${[\"example-ekmconnection\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${[\"example-ekmconnection\"].name}")
///             .project("${[\"example-ekmconnection\"].project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = ekm_connection_iam_binding::create(
///         "binding",
///         EkmConnectionIamBindingArgs::builder()
///             .condition(
///                 EkmConnectionIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${[\"example-ekmconnection\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${[\"example-ekmconnection\"].name}")
///             .project("${[\"example-ekmconnection\"].project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.kms.EkmConnectionIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = ekm_connection_iam_member::create(
///         "member",
///         EkmConnectionIamMemberArgs::builder()
///             .location("${[\"example-ekmconnection\"].location}")
///             .member("user:jane@example.com")
///             .name("${[\"example-ekmconnection\"].name}")
///             .project("${[\"example-ekmconnection\"].project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = ekm_connection_iam_member::create(
///         "member",
///         EkmConnectionIamMemberArgs::builder()
///             .condition(
///                 EkmConnectionIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${[\"example-ekmconnection\"].location}")
///             .member("user:jane@example.com")
///             .name("${[\"example-ekmconnection\"].name}")
///             .project("${[\"example-ekmconnection\"].project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Cloud Key Management Service EkmConnection
/// Three different resources help you manage your IAM policy for Cloud Key Management Service EkmConnection. Each of these resources serves a different use case:
///
/// * `gcp.kms.EkmConnectionIamPolicy`: Authoritative. Sets the IAM policy for the ekmconnection and replaces any existing policy already attached.
/// * `gcp.kms.EkmConnectionIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the ekmconnection are preserved.
/// * `gcp.kms.EkmConnectionIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the ekmconnection are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.kms.EkmConnectionIamPolicy`: Retrieves the IAM policy for the ekmconnection
///
/// > **Note:** `gcp.kms.EkmConnectionIamPolicy` **cannot** be used in conjunction with `gcp.kms.EkmConnectionIamBinding` and `gcp.kms.EkmConnectionIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.kms.EkmConnectionIamBinding` resources **can be** used in conjunction with `gcp.kms.EkmConnectionIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.kms.EkmConnectionIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:kms:EkmConnectionIamPolicy
///     properties:
///       project: ${["example-ekmconnection"].project}
///       location: ${["example-ekmconnection"].location}
///       name: ${["example-ekmconnection"].name}
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
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:kms:EkmConnectionIamPolicy
///     properties:
///       project: ${["example-ekmconnection"].project}
///       location: ${["example-ekmconnection"].location}
///       name: ${["example-ekmconnection"].name}
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
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.kms.EkmConnectionIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = ekm_connection_iam_binding::create(
///         "binding",
///         EkmConnectionIamBindingArgs::builder()
///             .location("${[\"example-ekmconnection\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${[\"example-ekmconnection\"].name}")
///             .project("${[\"example-ekmconnection\"].project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = ekm_connection_iam_binding::create(
///         "binding",
///         EkmConnectionIamBindingArgs::builder()
///             .condition(
///                 EkmConnectionIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${[\"example-ekmconnection\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${[\"example-ekmconnection\"].name}")
///             .project("${[\"example-ekmconnection\"].project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.kms.EkmConnectionIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = ekm_connection_iam_member::create(
///         "member",
///         EkmConnectionIamMemberArgs::builder()
///             .location("${[\"example-ekmconnection\"].location}")
///             .member("user:jane@example.com")
///             .name("${[\"example-ekmconnection\"].name}")
///             .project("${[\"example-ekmconnection\"].project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = ekm_connection_iam_member::create(
///         "member",
///         EkmConnectionIamMemberArgs::builder()
///             .condition(
///                 EkmConnectionIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${[\"example-ekmconnection\"].location}")
///             .member("user:jane@example.com")
///             .name("${[\"example-ekmconnection\"].name}")
///             .project("${[\"example-ekmconnection\"].project}")
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
/// * projects/{{project}}/locations/{{location}}/ekmConnections/{{name}}
///
/// * {{project}}/{{location}}/{{name}}
///
/// * {{location}}/{{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Key Management Service ekmconnection IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:kms/ekmConnectionIamPolicy:EkmConnectionIamPolicy editor "projects/{{project}}/locations/{{location}}/ekmConnections/{{ekm_connection}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:kms/ekmConnectionIamPolicy:EkmConnectionIamPolicy editor "projects/{{project}}/locations/{{location}}/ekmConnections/{{ekm_connection}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:kms/ekmConnectionIamPolicy:EkmConnectionIamPolicy editor projects/{{project}}/locations/{{location}}/ekmConnections/{{ekm_connection}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ekm_connection_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EkmConnectionIamPolicyArgs {
        /// The location for the EkmConnection.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EkmConnectionIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location for the EkmConnection.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EkmConnectionIamPolicyArgs,
    ) -> EkmConnectionIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:kms/ekmConnectionIamPolicy:EkmConnectionIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EkmConnectionIamPolicyResult {
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
        }
    }
}
