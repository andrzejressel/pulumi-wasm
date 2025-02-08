/// Restricts access to Cloud Console and Google Cloud APIs for a set of users using Context-Aware Access.
///
///
/// To get more information about GcpUserAccessBinding, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/organizations.gcpUserAccessBindings)
///
/// ## Example Usage
///
/// ### Access Context Manager Gcp User Access Binding Basic
///
///
/// ```yaml
/// resources:
///   group:
///     type: gcp:cloudidentity:Group
///     properties:
///       displayName: my-identity-group
///       parent: customers/A01b123xz
///       groupKey:
///         id: my-identity-group@example.com
///       labels:
///         cloudidentity.googleapis.com/groups.discussion_forum: ""
///   accessLevelIdForUserAccessBinding:
///     type: gcp:accesscontextmanager:AccessLevel
///     name: access_level_id_for_user_access_binding
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
///               - US
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: my policy
///   gcpUserAccessBinding:
///     type: gcp:accesscontextmanager:GcpUserAccessBinding
///     name: gcp_user_access_binding
///     properties:
///       organizationId: '123456789'
///       groupKey:
///         fn::invoke:
///           function: std:trimprefix
///           arguments:
///             input: ${group.id}
///             prefix: groups/
///           return: result
///       accessLevels: ${accessLevelIdForUserAccessBinding.name}
/// ```
///
/// ## Import
///
/// GcpUserAccessBinding can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, GcpUserAccessBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/gcpUserAccessBinding:GcpUserAccessBinding default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gcp_user_access_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GcpUserAccessBindingArgs {
        /// Required. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: "accessPolicies/9522/accessLevels/device_trusted"
        #[builder(into)]
        pub access_levels: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. Immutable. Google Group id whose members are subject to this binding's restrictions. See "id" in the G Suite Directory API's Groups resource. If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: "01d520gv4vjcrht"
        #[builder(into)]
        pub group_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. ID of the parent organization.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub organization_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GcpUserAccessBindingResult {
        /// Required. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: "accessPolicies/9522/accessLevels/device_trusted"
        pub access_levels: pulumi_gestalt_rust::Output<String>,
        /// Required. Immutable. Google Group id whose members are subject to this binding's restrictions. See "id" in the G Suite Directory API's Groups resource. If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: "01d520gv4vjcrht"
        pub group_key: pulumi_gestalt_rust::Output<String>,
        /// Immutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by RFC 3986 Section 2.3). Should not be specified by the client during creation. Example: "organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Required. ID of the parent organization.
        ///
        ///
        /// - - -
        pub organization_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GcpUserAccessBindingArgs,
    ) -> GcpUserAccessBindingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_levels_binding = args.access_levels.get_output(context).get_inner();
        let group_key_binding = args.group_key.get_output(context).get_inner();
        let organization_id_binding = args
            .organization_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/gcpUserAccessBinding:GcpUserAccessBinding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessLevels".into(),
                    value: &access_levels_binding,
                },
                register_interface::ObjectField {
                    name: "groupKey".into(),
                    value: &group_key_binding,
                },
                register_interface::ObjectField {
                    name: "organizationId".into(),
                    value: &organization_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GcpUserAccessBindingResult {
            access_levels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessLevels"),
            ),
            group_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupKey"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            organization_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("organizationId"),
            ),
        }
    }
}
