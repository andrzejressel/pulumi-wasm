/// Custom constraints are created by administrators to provide more granular and customizable control over the specific fields that are restricted by your organization policies.
///
///
/// To get more information about CustomConstraint, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/docs/reference/orgpolicy/rest/v2/organizations.constraints)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/resource-manager/docs/organization-policy/creating-managing-custom-constraints)
///     * [Supported Services](https://cloud.google.com/resource-manager/docs/organization-policy/custom-constraint-supported-services)
///
/// ## Example Usage
///
/// ### Org Policy Custom Constraint Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let constraint = custom_constraint::create(
///         "constraint",
///         CustomConstraintArgs::builder()
///             .action_type("ALLOW")
///             .condition("resource.management.autoUpgrade == false")
///             .method_types(vec!["CREATE", "UPDATE",])
///             .name("custom.disableGkeAutoUpgrade")
///             .parent("organizations/123456789")
///             .resource_types(vec!["container.googleapis.com/NodePool",])
///             .build_struct(),
///     );
/// }
/// ```
/// ### Org Policy Custom Constraint Full
///
///
/// ```yaml
/// resources:
///   constraint:
///     type: gcp:orgpolicy:CustomConstraint
///     properties:
///       name: custom.disableGkeAutoUpgrade
///       parent: organizations/123456789
///       displayName: Disable GKE auto upgrade
///       description: Only allow GKE NodePool resource to be created or updated if AutoUpgrade is not enabled where this custom constraint is enforced.
///       actionType: ALLOW
///       condition: resource.management.autoUpgrade == false
///       methodTypes:
///         - CREATE
///         - UPDATE
///       resourceTypes:
///         - container.googleapis.com/NodePool
///   bool:
///     type: gcp:orgpolicy:Policy
///     properties:
///       name: organizations/123456789/policies/${constraint.name}
///       parent: organizations/123456789
///       spec:
///         rules:
///           - enforce: TRUE
/// ```
///
/// ## Import
///
/// CustomConstraint can be imported using any of these accepted formats:
///
/// * `{{parent}}/customConstraints/{{name}}`
///
/// When using the `pulumi import` command, CustomConstraint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:orgpolicy/customConstraint:CustomConstraint default {{parent}}/customConstraints/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod custom_constraint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomConstraintArgs {
        /// The action to take if the condition is met.
        /// Possible values are: `ALLOW`, `DENY`.
        #[builder(into)]
        pub action_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A CEL condition that refers to a supported service resource, for example `resource.management.autoUpgrade == false`. For details about CEL usage, see [Common Expression Language](https://cloud.google.com/resource-manager/docs/organization-policy/creating-managing-custom-constraints#common_expression_language).
        #[builder(into)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A human-friendly description of the constraint to display as an error message when the policy is violated.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A human-friendly name for the constraint.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of RESTful methods for which to enforce the constraint. Can be `CREATE`, `UPDATE`, or both. Not all Google Cloud services support both methods. To see supported methods for each service, find the service in [Supported services](https://cloud.google.com/resource-manager/docs/organization-policy/custom-constraint-supported-services).
        #[builder(into)]
        pub method_types: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Immutable. The name of the custom constraint. This is unique within the organization.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Immutable. The fully qualified name of the Google Cloud REST resource containing the object and field you want to restrict. For example, `container.googleapis.com/NodePool`.
        #[builder(into)]
        pub resource_types: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct CustomConstraintResult {
        /// The action to take if the condition is met.
        /// Possible values are: `ALLOW`, `DENY`.
        pub action_type: pulumi_gestalt_rust::Output<String>,
        /// A CEL condition that refers to a supported service resource, for example `resource.management.autoUpgrade == false`. For details about CEL usage, see [Common Expression Language](https://cloud.google.com/resource-manager/docs/organization-policy/creating-managing-custom-constraints#common_expression_language).
        pub condition: pulumi_gestalt_rust::Output<String>,
        /// A human-friendly description of the constraint to display as an error message when the policy is violated.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A human-friendly name for the constraint.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of RESTful methods for which to enforce the constraint. Can be `CREATE`, `UPDATE`, or both. Not all Google Cloud services support both methods. To see supported methods for each service, find the service in [Supported services](https://cloud.google.com/resource-manager/docs/organization-policy/custom-constraint-supported-services).
        pub method_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Immutable. The name of the custom constraint. This is unique within the organization.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        ///
        ///
        /// - - -
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The fully qualified name of the Google Cloud REST resource containing the object and field you want to restrict. For example, `container.googleapis.com/NodePool`.
        pub resource_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Output only. The timestamp representing when the constraint was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomConstraintArgs,
    ) -> CustomConstraintResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_type_binding = args.action_type.get_output(context).get_inner();
        let condition_binding = args.condition.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let method_types_binding = args.method_types.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let resource_types_binding = args.resource_types.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:orgpolicy/customConstraint:CustomConstraint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actionType".into(),
                    value: &action_type_binding,
                },
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "methodTypes".into(),
                    value: &method_types_binding,
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
                    name: "resourceTypes".into(),
                    value: &resource_types_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomConstraintResult {
            action_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("actionType"),
            ),
            condition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            method_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("methodTypes"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            resource_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTypes"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
