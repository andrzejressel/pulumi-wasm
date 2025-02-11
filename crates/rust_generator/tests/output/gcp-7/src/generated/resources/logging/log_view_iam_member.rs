/// Three different resources help you manage your IAM policy for Cloud (Stackdriver) Logging LogView. Each of these resources serves a different use case:
///
/// * `gcp.logging.LogViewIamPolicy`: Authoritative. Sets the IAM policy for the logview and replaces any existing policy already attached.
/// * `gcp.logging.LogViewIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the logview are preserved.
/// * `gcp.logging.LogViewIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the logview are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.logging.LogViewIamPolicy`: Retrieves the IAM policy for the logview
///
/// > **Note:** `gcp.logging.LogViewIamPolicy` **cannot** be used in conjunction with `gcp.logging.LogViewIamBinding` and `gcp.logging.LogViewIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.logging.LogViewIamBinding` resources **can be** used in conjunction with `gcp.logging.LogViewIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.logging.LogViewIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:logging:LogViewIamPolicy
///     properties:
///       parent: ${loggingLogView.parent}
///       location: ${loggingLogView.location}
///       bucket: ${loggingLogView.bucket}
///       name: ${loggingLogView.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/logging.admin
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:logging:LogViewIamPolicy
///     properties:
///       parent: ${loggingLogView.parent}
///       location: ${loggingLogView.location}
///       bucket: ${loggingLogView.bucket}
///       name: ${loggingLogView.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/logging.admin
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.logging.LogViewIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = log_view_iam_binding::create(
///         "binding",
///         LogViewIamBindingArgs::builder()
///             .bucket("${loggingLogView.bucket}")
///             .location("${loggingLogView.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${loggingLogView.name}")
///             .parent("${loggingLogView.parent}")
///             .role("roles/logging.admin")
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
///     let binding = log_view_iam_binding::create(
///         "binding",
///         LogViewIamBindingArgs::builder()
///             .bucket("${loggingLogView.bucket}")
///             .condition(
///                 LogViewIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${loggingLogView.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${loggingLogView.name}")
///             .parent("${loggingLogView.parent}")
///             .role("roles/logging.admin")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.logging.LogViewIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = log_view_iam_member::create(
///         "member",
///         LogViewIamMemberArgs::builder()
///             .bucket("${loggingLogView.bucket}")
///             .location("${loggingLogView.location}")
///             .member("user:jane@example.com")
///             .name("${loggingLogView.name}")
///             .parent("${loggingLogView.parent}")
///             .role("roles/logging.admin")
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
///     let member = log_view_iam_member::create(
///         "member",
///         LogViewIamMemberArgs::builder()
///             .bucket("${loggingLogView.bucket}")
///             .condition(
///                 LogViewIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${loggingLogView.location}")
///             .member("user:jane@example.com")
///             .name("${loggingLogView.name}")
///             .parent("${loggingLogView.parent}")
///             .role("roles/logging.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## > **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
/// -
///
/// # IAM policy for Cloud (Stackdriver) Logging LogView
/// Three different resources help you manage your IAM policy for Cloud (Stackdriver) Logging LogView. Each of these resources serves a different use case:
///
/// * `gcp.logging.LogViewIamPolicy`: Authoritative. Sets the IAM policy for the logview and replaces any existing policy already attached.
/// * `gcp.logging.LogViewIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the logview are preserved.
/// * `gcp.logging.LogViewIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the logview are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.logging.LogViewIamPolicy`: Retrieves the IAM policy for the logview
///
/// > **Note:** `gcp.logging.LogViewIamPolicy` **cannot** be used in conjunction with `gcp.logging.LogViewIamBinding` and `gcp.logging.LogViewIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.logging.LogViewIamBinding` resources **can be** used in conjunction with `gcp.logging.LogViewIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.logging.LogViewIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:logging:LogViewIamPolicy
///     properties:
///       parent: ${loggingLogView.parent}
///       location: ${loggingLogView.location}
///       bucket: ${loggingLogView.bucket}
///       name: ${loggingLogView.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/logging.admin
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:logging:LogViewIamPolicy
///     properties:
///       parent: ${loggingLogView.parent}
///       location: ${loggingLogView.location}
///       bucket: ${loggingLogView.bucket}
///       name: ${loggingLogView.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/logging.admin
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.logging.LogViewIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = log_view_iam_binding::create(
///         "binding",
///         LogViewIamBindingArgs::builder()
///             .bucket("${loggingLogView.bucket}")
///             .location("${loggingLogView.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${loggingLogView.name}")
///             .parent("${loggingLogView.parent}")
///             .role("roles/logging.admin")
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
///     let binding = log_view_iam_binding::create(
///         "binding",
///         LogViewIamBindingArgs::builder()
///             .bucket("${loggingLogView.bucket}")
///             .condition(
///                 LogViewIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${loggingLogView.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${loggingLogView.name}")
///             .parent("${loggingLogView.parent}")
///             .role("roles/logging.admin")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.logging.LogViewIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = log_view_iam_member::create(
///         "member",
///         LogViewIamMemberArgs::builder()
///             .bucket("${loggingLogView.bucket}")
///             .location("${loggingLogView.location}")
///             .member("user:jane@example.com")
///             .name("${loggingLogView.name}")
///             .parent("${loggingLogView.parent}")
///             .role("roles/logging.admin")
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
///     let member = log_view_iam_member::create(
///         "member",
///         LogViewIamMemberArgs::builder()
///             .bucket("${loggingLogView.bucket}")
///             .condition(
///                 LogViewIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .location("${loggingLogView.location}")
///             .member("user:jane@example.com")
///             .name("${loggingLogView.name}")
///             .parent("${loggingLogView.parent}")
///             .role("roles/logging.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * {{parent}}/locations/{{location}}/buckets/{{bucket}}/views/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud (Stackdriver) Logging logview IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:logging/logViewIamMember:LogViewIamMember editor "{{parent}}/locations/{{location}}/buckets/{{bucket}}/views/{{log_view}} roles/logging.admin user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:logging/logViewIamMember:LogViewIamMember editor "{{parent}}/locations/{{location}}/buckets/{{bucket}}/views/{{log_view}} roles/logging.admin"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:logging/logViewIamMember:LogViewIamMember editor {{parent}}/locations/{{location}}/buckets/{{bucket}}/views/{{log_view}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_view_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogViewIamMemberArgs {
        /// The bucket of the resource Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logging::LogViewIamMemberCondition>,
        >,
        /// The location of the resource. The supported locations are: global, us-central1, us-east1, us-west1, asia-east1, europe-west1. Used to find the parent resource to bind the IAM policy to. If not specified,
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
        pub member: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parent of the resource. Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The role that should be applied. Only one
        /// `gcp.logging.LogViewIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogViewIamMemberResult {
        /// The bucket of the resource Used to find the parent resource to bind the IAM policy to
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::logging::LogViewIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location of the resource. The supported locations are: global, us-central1, us-east1, us-west1, asia-east1, europe-west1. Used to find the parent resource to bind the IAM policy to. If not specified,
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
        pub member: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the resource. Used to find the parent resource to bind the IAM policy to
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.logging.LogViewIamBinding` can be used per role. Note that custom roles must be of the format
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
        args: LogViewIamMemberArgs,
    ) -> LogViewIamMemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let condition_binding = args.condition.get_output(context);
        let location_binding = args.location.get_output(context);
        let member_binding = args.member.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:logging/logViewIamMember:LogViewIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "member".into(),
                    value: &member_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogViewIamMemberResult {
            bucket: o.get_field("bucket"),
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            member: o.get_field("member"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            role: o.get_field("role"),
        }
    }
}
