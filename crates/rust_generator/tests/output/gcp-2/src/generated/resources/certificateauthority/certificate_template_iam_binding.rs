/// Three different resources help you manage your IAM policy for Certificate Authority Service CertificateTemplate. Each of these resources serves a different use case:
///
/// * `gcp.certificateauthority.CertificateTemplateIamPolicy`: Authoritative. Sets the IAM policy for the certificatetemplate and replaces any existing policy already attached.
/// * `gcp.certificateauthority.CertificateTemplateIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the certificatetemplate are preserved.
/// * `gcp.certificateauthority.CertificateTemplateIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the certificatetemplate are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.certificateauthority.CertificateTemplateIamPolicy`: Retrieves the IAM policy for the certificatetemplate
///
/// > **Note:** `gcp.certificateauthority.CertificateTemplateIamPolicy` **cannot** be used in conjunction with `gcp.certificateauthority.CertificateTemplateIamBinding` and `gcp.certificateauthority.CertificateTemplateIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.certificateauthority.CertificateTemplateIamBinding` resources **can be** used in conjunction with `gcp.certificateauthority.CertificateTemplateIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.certificateauthority.CertificateTemplateIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:certificateauthority:CertificateTemplateIamPolicy
///     properties:
///       certificateTemplate: ${default.id}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/privateca.templateUser
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:certificateauthority:CertificateTemplateIamPolicy
///     properties:
///       certificateTemplate: ${default.id}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/privateca.templateUser
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.certificateauthority.CertificateTemplateIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = certificate_template_iam_binding::create(
///         "binding",
///         CertificateTemplateIamBindingArgs::builder()
///             .certificate_template("${default.id}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/privateca.templateUser")
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
///     let binding = certificate_template_iam_binding::create(
///         "binding",
///         CertificateTemplateIamBindingArgs::builder()
///             .certificate_template("${default.id}")
///             .condition(
///                 CertificateTemplateIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .role("roles/privateca.templateUser")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.certificateauthority.CertificateTemplateIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = certificate_template_iam_member::create(
///         "member",
///         CertificateTemplateIamMemberArgs::builder()
///             .certificate_template("${default.id}")
///             .member("user:jane@example.com")
///             .role("roles/privateca.templateUser")
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
///     let member = certificate_template_iam_member::create(
///         "member",
///         CertificateTemplateIamMemberArgs::builder()
///             .certificate_template("${default.id}")
///             .condition(
///                 CertificateTemplateIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .role("roles/privateca.templateUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Certificate Authority Service CertificateTemplate
/// Three different resources help you manage your IAM policy for Certificate Authority Service CertificateTemplate. Each of these resources serves a different use case:
///
/// * `gcp.certificateauthority.CertificateTemplateIamPolicy`: Authoritative. Sets the IAM policy for the certificatetemplate and replaces any existing policy already attached.
/// * `gcp.certificateauthority.CertificateTemplateIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the certificatetemplate are preserved.
/// * `gcp.certificateauthority.CertificateTemplateIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the certificatetemplate are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.certificateauthority.CertificateTemplateIamPolicy`: Retrieves the IAM policy for the certificatetemplate
///
/// > **Note:** `gcp.certificateauthority.CertificateTemplateIamPolicy` **cannot** be used in conjunction with `gcp.certificateauthority.CertificateTemplateIamBinding` and `gcp.certificateauthority.CertificateTemplateIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.certificateauthority.CertificateTemplateIamBinding` resources **can be** used in conjunction with `gcp.certificateauthority.CertificateTemplateIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.certificateauthority.CertificateTemplateIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:certificateauthority:CertificateTemplateIamPolicy
///     properties:
///       certificateTemplate: ${default.id}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/privateca.templateUser
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:certificateauthority:CertificateTemplateIamPolicy
///     properties:
///       certificateTemplate: ${default.id}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/privateca.templateUser
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.certificateauthority.CertificateTemplateIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = certificate_template_iam_binding::create(
///         "binding",
///         CertificateTemplateIamBindingArgs::builder()
///             .certificate_template("${default.id}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/privateca.templateUser")
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
///     let binding = certificate_template_iam_binding::create(
///         "binding",
///         CertificateTemplateIamBindingArgs::builder()
///             .certificate_template("${default.id}")
///             .condition(
///                 CertificateTemplateIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .role("roles/privateca.templateUser")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.certificateauthority.CertificateTemplateIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = certificate_template_iam_member::create(
///         "member",
///         CertificateTemplateIamMemberArgs::builder()
///             .certificate_template("${default.id}")
///             .member("user:jane@example.com")
///             .role("roles/privateca.templateUser")
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
///     let member = certificate_template_iam_member::create(
///         "member",
///         CertificateTemplateIamMemberArgs::builder()
///             .certificate_template("${default.id}")
///             .condition(
///                 CertificateTemplateIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .role("roles/privateca.templateUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/certificateTemplates/{{name}}
///
/// * {{project}}/{{location}}/{{name}}
///
/// * {{location}}/{{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Certificate Authority Service certificatetemplate IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/certificateTemplateIamBinding:CertificateTemplateIamBinding editor "projects/{{project}}/locations/{{location}}/certificateTemplates/{{certificate_template}} roles/privateca.templateUser user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/certificateTemplateIamBinding:CertificateTemplateIamBinding editor "projects/{{project}}/locations/{{location}}/certificateTemplates/{{certificate_template}} roles/privateca.templateUser"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/certificateTemplateIamBinding:CertificateTemplateIamBinding editor projects/{{project}}/locations/{{location}}/certificateTemplates/{{certificate_template}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate_template_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateTemplateIamBindingArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub certificate_template: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::certificateauthority::CertificateTemplateIamBindingCondition,
            >,
        >,
        /// The location for the resource Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        #[builder(into)]
        pub members: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.certificateauthority.CertificateTemplateIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateTemplateIamBindingResult {
        /// Used to find the parent resource to bind the IAM policy to
        pub certificate_template: pulumi_gestalt_rust::Output<String>,
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        pub condition: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::certificateauthority::CertificateTemplateIamBindingCondition,
            >,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location for the resource Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.certificateauthority.CertificateTemplateIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateTemplateIamBindingArgs,
    ) -> CertificateTemplateIamBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_template_binding = args.certificate_template.get_output(context);
        let condition_binding = args.condition.get_output(context);
        let location_binding = args.location.get_output(context);
        let members_binding = args.members.get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:certificateauthority/certificateTemplateIamBinding:CertificateTemplateIamBinding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateTemplate".into(),
                    value: certificate_template_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateTemplateIamBindingResult {
            certificate_template: o.get_field("certificateTemplate"),
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            members: o.get_field("members"),
            project: o.get_field("project"),
            role: o.get_field("role"),
        }
    }
}
