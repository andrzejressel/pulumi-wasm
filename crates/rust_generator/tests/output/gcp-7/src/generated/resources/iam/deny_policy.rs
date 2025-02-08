/// Represents a collection of denial policies to apply to a given resource.
///
///
/// To get more information about DenyPolicy, see:
///
/// * [API documentation](https://cloud.google.com/iam/docs/reference/rest/v2/policies)
/// * How-to Guides
///     * [Permissions supported in deny policies](https://cloud.google.com/iam/docs/deny-permissions-support)
///
/// ## Example Usage
///
/// ### Iam Deny Policy Basic
///
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:organizations:Project
///     properties:
///       projectId: my-project
///       name: my-project
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   example:
///     type: gcp:iam:DenyPolicy
///     properties:
///       parent:
///         fn::invoke:
///           function: std:urlencode
///           arguments:
///             input: cloudresourcemanager.googleapis.com/projects/${project.projectId}
///           return: result
///       name: my-deny-policy
///       displayName: A deny rule
///       rules:
///         - description: First rule
///           denyRule:
///             deniedPrincipals:
///               - principalSet://goog/public:all
///             denialCondition:
///               title: Some expr
///               expression: '!resource.matchTag(''12345678/env'', ''test'')'
///             deniedPermissions:
///               - cloudresourcemanager.googleapis.com/projects.update
///         - description: Second rule
///           denyRule:
///             deniedPrincipals:
///               - principalSet://goog/public:all
///             denialCondition:
///               title: Some expr
///               expression: '!resource.matchTag(''12345678/env'', ''test'')'
///             deniedPermissions:
///               - cloudresourcemanager.googleapis.com/projects.update
///             exceptionPrincipals:
///               - principal://iam.googleapis.com/projects/-/serviceAccounts/${["test-account"].email}
///   test-account:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: svc-acc
///       displayName: Test Service Account
///       project: ${project.projectId}
/// ```
///
/// ## Import
///
/// DenyPolicy can be imported using any of these accepted formats:
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, DenyPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iam/denyPolicy:DenyPolicy default {{parent}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deny_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DenyPolicyArgs {
        /// The display name of the rule.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The attachment point is identified by its URL-encoded full resource name.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Rules to be applied.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::iam::DenyPolicyRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct DenyPolicyResult {
        /// The display name of the rule.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The hash of the resource. Used internally during updates.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The name of the policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The attachment point is identified by its URL-encoded full resource name.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Rules to be applied.
        /// Structure is documented below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::iam::DenyPolicyRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DenyPolicyArgs,
    ) -> DenyPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iam/denyPolicy:DenyPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DenyPolicyResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            rules: pulumi_gestalt_rust::__private::into_domain(o.extract_field("rules")),
        }
    }
}
