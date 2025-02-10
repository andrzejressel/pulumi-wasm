/// Three different resources help you manage your IAM policy for Cloud Endpoints ServiceConsumers. Each of these resources serves a different use case:
///
/// * `gcp.endpoints.ConsumersIamPolicy`: Authoritative. Sets the IAM policy for the serviceconsumers and replaces any existing policy already attached.
/// * `gcp.endpoints.ConsumersIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the serviceconsumers are preserved.
/// * `gcp.endpoints.ConsumersIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the serviceconsumers are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.endpoints.ConsumersIamPolicy`: Retrieves the IAM policy for the serviceconsumers
///
/// > **Note:** `gcp.endpoints.ConsumersIamPolicy` **cannot** be used in conjunction with `gcp.endpoints.ConsumersIamBinding` and `gcp.endpoints.ConsumersIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.endpoints.ConsumersIamBinding` resources **can be** used in conjunction with `gcp.endpoints.ConsumersIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * services/{{service_name}}/consumers/{{consumer_project}}
///
/// * {{service_name}}/{{consumer_project}}
///
/// * {{consumer_project}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Endpoints serviceconsumers IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:endpoints/consumersIamBinding:ConsumersIamBinding editor "services/{{service_name}}/consumers/{{consumer_project}} roles/servicemanagement.serviceController user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:endpoints/consumersIamBinding:ConsumersIamBinding editor "services/{{service_name}}/consumers/{{consumer_project}} roles/servicemanagement.serviceController"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:endpoints/consumersIamBinding:ConsumersIamBinding editor services/{{service_name}}/consumers/{{consumer_project}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod consumers_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConsumersIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::endpoints::ConsumersIamBindingCondition>,
        >,
        #[builder(into)]
        pub consumer_project: pulumi_gestalt_rust::InputOrOutput<String>,
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
        /// The role that should be applied. Only one
        /// `gcp.endpoints.ConsumersIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConsumersIamBindingResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::endpoints::ConsumersIamBindingCondition>,
        >,
        pub consumer_project: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
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
        /// The role that should be applied. Only one
        /// `gcp.endpoints.ConsumersIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
        pub service_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConsumersIamBindingArgs,
    ) -> ConsumersIamBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let consumer_project_binding = args.consumer_project.get_output(context);
        let members_binding = args.members.get_output(context);
        let role_binding = args.role.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:endpoints/consumersIamBinding:ConsumersIamBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consumerProject".into(),
                    value: consumer_project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: service_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConsumersIamBindingResult {
            condition: o.get_field("condition"),
            consumer_project: o.get_field("consumerProject"),
            etag: o.get_field("etag"),
            members: o.get_field("members"),
            role: o.get_field("role"),
            service_name: o.get_field("serviceName"),
        }
    }
}
