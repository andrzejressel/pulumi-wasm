/// Defines an organization policy which is used to specify constraints for configurations of Google Cloud resources.
///
///
/// To get more information about Policy, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/docs/reference/orgpolicy/rest/v2/organizations.policies)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/resource-manager/docs/organization-policy/creating-managing-custom-constraints)
///     * [Supported Services](https://cloud.google.com/resource-manager/docs/organization-policy/custom-constraint-supported-services)
///
/// ## Example Usage
///
/// ### Org Policy Policy Enforce
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:orgpolicy:Policy
///     properties:
///       name: projects/${basic.projectId}/policies/iam.disableServiceAccountKeyUpload
///       parent: projects/${basic.projectId}
///       spec:
///         rules:
///           - enforce: FALSE
///   basic:
///     type: gcp:organizations:Project
///     properties:
///       projectId: id
///       name: id
///       orgId: '123456789'
///       deletionPolicy: DELETE
/// ```
/// ### Org Policy Policy Folder
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:orgpolicy:Policy
///     properties:
///       name: ${basic.name}/policies/gcp.resourceLocations
///       parent: ${basic.name}
///       spec:
///         inheritFromParent: true
///         rules:
///           - denyAll: TRUE
///   basic:
///     type: gcp:organizations:Folder
///     properties:
///       parent: organizations/123456789
///       displayName: folder
///       deletionProtection: false
/// ```
/// ### Org Policy Policy Organization
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = policy::create(
///         "primary",
///         PolicyArgs::builder()
///             .name("organizations/123456789/policies/gcp.detailedAuditLoggingMode")
///             .parent("organizations/123456789")
///             .spec(PolicySpec::builder().reset(true).build_struct())
///             .build_struct(),
///     );
/// }
/// ```
/// ### Org Policy Policy Project
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:orgpolicy:Policy
///     properties:
///       name: projects/${basic.projectId}/policies/gcp.resourceLocations
///       parent: projects/${basic.projectId}
///       spec:
///         rules:
///           - condition:
///               description: A sample condition for the policy
///               expression: resource.matchTagId('tagKeys/123', 'tagValues/345')
///               location: sample-location.log
///               title: sample-condition
///             values:
///               allowedValues:
///                 - projects/allowed-project
///               deniedValues:
///                 - projects/denied-project
///           - allowAll: TRUE
///   basic:
///     type: gcp:organizations:Project
///     properties:
///       projectId: id
///       name: id
///       orgId: '123456789'
///       deletionPolicy: DELETE
/// ```
/// ### Org Policy Policy Dry Run Spec
///
///
/// ```yaml
/// resources:
///   constraint:
///     type: gcp:orgpolicy:CustomConstraint
///     properties:
///       name: custom.disableGkeAutoUpgrade_40289
///       parent: organizations/123456789
///       displayName: Disable GKE auto upgrade
///       description: Only allow GKE NodePool resource to be created or updated if AutoUpgrade is not enabled where this custom constraint is enforced.
///       actionType: ALLOW
///       condition: resource.management.autoUpgrade == false
///       methodTypes:
///         - CREATE
///       resourceTypes:
///         - container.googleapis.com/NodePool
///   primary:
///     type: gcp:orgpolicy:Policy
///     properties:
///       name: organizations/123456789/policies/${constraint.name}
///       parent: organizations/123456789
///       spec:
///         rules:
///           - enforce: FALSE
///       dryRunSpec:
///         inheritFromParent: false
///         reset: false
///         rules:
///           - enforce: FALSE
/// ```
/// ### Org Policy Policy Parameters Enforce
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:orgpolicy:Policy
///     properties:
///       name: projects/${basic.name}/policies/compute.managed.restrictDiskCreation
///       parent: projects/${basic.name}
///       spec:
///         rules:
///           - enforce: TRUE
///             parameters:
///               fn::toJSON:
///                 isSizeLimitCheck: true
///                 allowedDiskTypes:
///                   - pd-ssd
///                   - pd-standard
///   basic:
///     type: gcp:organizations:Project
///     properties:
///       projectId: id
///       name: id
///       orgId: '123456789'
///       deletionPolicy: DELETE
/// ```
///
/// ## Import
///
/// Policy can be imported using any of these accepted formats:
///
/// * `{{parent}}/policies/{{name}}`
///
/// When using the `pulumi import` command, Policy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:orgpolicy/policy:Policy default {{parent}}/policies/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// Dry-run policy. Audit-only policy, can be used to monitor how the policy would have impacted the existing and future resources if it's enforced.
        /// Structure is documented below.
        #[builder(into, default)]
        pub dry_run_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::orgpolicy::PolicyDryRunSpec>,
        >,
        /// Immutable. The resource name of the Policy. Must be one of the following forms, where constraint_name is the name of the constraint which this Policy configures: * `projects/{project_number}/policies/{constraint_name}` * `folders/{folder_id}/policies/{constraint_name}` * `organizations/{organization_id}/policies/{constraint_name}` For example, "projects/123/policies/compute.disableSerialPortAccess". Note: `projects/{project_id}/policies/{constraint_name}` is also an acceptable name for API requests, but responses will return the name using the equivalent project number.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parent of the resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Basic information about the Organization Policy.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::orgpolicy::PolicySpec>,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// Dry-run policy. Audit-only policy, can be used to monitor how the policy would have impacted the existing and future resources if it's enforced.
        /// Structure is documented below.
        pub dry_run_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::orgpolicy::PolicyDryRunSpec>,
        >,
        /// Optional. An opaque tag indicating the current state of the policy, used for concurrency control. This 'etag' is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The resource name of the Policy. Must be one of the following forms, where constraint_name is the name of the constraint which this Policy configures: * `projects/{project_number}/policies/{constraint_name}` * `folders/{folder_id}/policies/{constraint_name}` * `organizations/{organization_id}/policies/{constraint_name}` For example, "projects/123/policies/compute.disableSerialPortAccess". Note: `projects/{project_id}/policies/{constraint_name}` is also an acceptable name for API requests, but responses will return the name using the equivalent project number.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the resource.
        ///
        ///
        /// - - -
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Basic information about the Organization Policy.
        /// Structure is documented below.
        pub spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::orgpolicy::PolicySpec>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dry_run_spec_binding = args.dry_run_spec.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let spec_binding = args.spec.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:orgpolicy/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dryRunSpec".into(),
                    value: &dry_run_spec_binding,
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
                    name: "spec".into(),
                    value: &spec_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyResult {
            dry_run_spec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dryRunSpec"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            spec: pulumi_gestalt_rust::__private::into_domain(o.extract_field("spec")),
        }
    }
}
