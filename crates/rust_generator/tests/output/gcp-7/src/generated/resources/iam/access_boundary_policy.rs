/// Represents a collection of access boundary policies to apply to a given resource.
/// **NOTE**: This is a private feature and users should contact GCP support
/// if they would like to test it.
///
///
///
/// ## Example Usage
///
/// ### Iam Access Boundary Policy Basic
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
///   test-access:
///     type: gcp:accesscontextmanager:AccessLevel
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       name: accessPolicies/${["access-policy"].name}/accessLevels/chromeos_no_lock
///       title: chromeos_no_lock
///       basic:
///         conditions:
///           - devicePolicy:
///               requireScreenLock: true
///               osConstraints:
///                 - osType: DESKTOP_CHROME_OS
///             regions:
///               - CH
///               - IT
///               - US
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/${project.orgId}
///       title: my policy
///   example:
///     type: gcp:iam:AccessBoundaryPolicy
///     properties:
///       parent:
///         fn::invoke:
///           function: std:urlencode
///           arguments:
///             input: cloudresourcemanager.googleapis.com/projects/${project.projectId}
///           return: result
///       name: my-ab-policy
///       displayName: My AB policy
///       rules:
///         - description: AB rule
///           accessBoundaryRule:
///             availableResource: '*'
///             availablePermissions:
///               - '*'
///             availabilityCondition:
///               title: Access level expr
///               expression: request.matchAccessLevels('${project.orgId}', ['${["test-access"].name}'])
/// ```
///
/// ## Import
///
/// AccessBoundaryPolicy can be imported using any of these accepted formats:
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, AccessBoundaryPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iam/accessBoundaryPolicy:AccessBoundaryPolicy default {{parent}}/{{name}}
/// ```
///
pub mod access_boundary_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessBoundaryPolicyArgs {
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
            Vec<super::super::types::iam::AccessBoundaryPolicyRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessBoundaryPolicyResult {
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
            Vec<super::super::types::iam::AccessBoundaryPolicyRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccessBoundaryPolicyArgs,
    ) -> AccessBoundaryPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iam/accessBoundaryPolicy:AccessBoundaryPolicy".into(),
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
        AccessBoundaryPolicyResult {
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
