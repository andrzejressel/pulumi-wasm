/// AccessPolicy is a container for AccessLevels (which define the necessary
/// attributes to use GCP services) and ServicePerimeters (which define
/// regions of services able to freely pass data within a perimeter). An
/// access policy is globally visible within an organization, and the
/// restrictions it specifies apply to all projects within an organization.
///
///
/// To get more information about AccessPolicy, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies)
/// * How-to Guides
///     * [Access Policy Quickstart](https://cloud.google.com/access-context-manager/docs/quickstart)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the ACM API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Access Context Manager Access Policy Basic
///
///
/// ```yaml
/// resources:
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: Org Access Policy
/// ```
/// ### Access Context Manager Access Policy Scoped
///
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:organizations:Project
///     properties:
///       projectId: my-project-name
///       name: my-project-name
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: Scoped Access Policy
///       scopes: projects/${project.number}
/// ```
///
/// ## Import
///
/// AccessPolicy can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AccessPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/accessPolicy:AccessPolicy default {{name}}
/// ```
///
pub mod access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPolicyArgs {
        /// The parent of this AccessPolicy in the Cloud Resource Hierarchy.
        /// Format: 'organizations/{{organization_id}}'
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
        /// Folder or project on which this policy is applicable.
        /// Format: 'folders/{{folder_id}}' or 'projects/{{project_number}}'
        #[builder(into, default)]
        pub scopes: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Human readable title. Does not affect behavior.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub title: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessPolicyResult {
        /// Time the AccessPolicy was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Resource name of the AccessPolicy. Format: '{{policy_id}}'
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent of this AccessPolicy in the Cloud Resource Hierarchy.
        /// Format: 'organizations/{{organization_id}}'
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Folder or project on which this policy is applicable.
        /// Format: 'folders/{{folder_id}}' or 'projects/{{project_number}}'
        pub scopes: pulumi_wasm_rust::Output<Option<String>>,
        /// Human readable title. Does not affect behavior.
        ///
        ///
        /// - - -
        pub title: pulumi_wasm_rust::Output<String>,
        /// Time the AccessPolicy was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccessPolicyArgs,
    ) -> AccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_output(context).get_inner();
        let scopes_binding = args.scopes.get_output(context).get_inner();
        let title_binding = args.title.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/accessPolicy:AccessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
                register_interface::ObjectField {
                    name: "title".into(),
                    value: &title_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccessPolicyResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            scopes: pulumi_wasm_rust::__private::into_domain(o.extract_field("scopes")),
            title: pulumi_wasm_rust::__private::into_domain(o.extract_field("title")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
