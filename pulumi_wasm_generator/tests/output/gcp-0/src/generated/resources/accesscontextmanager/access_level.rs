/// An AccessLevel is a label that can be applied to requests to GCP services,
/// along with a list of requirements necessary for the label to be applied.
///
///
/// To get more information about AccessLevel, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.accessLevels)
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
/// ### Access Context Manager Access Level Basic
///
///
/// ```yaml
/// resources:
///   access-level:
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
///       parent: organizations/123456789
///       title: my policy
/// ```
///
/// ## Import
///
/// AccessLevel can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AccessLevel can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/accessLevel:AccessLevel default {{name}}
/// ```
///
pub mod access_level {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessLevelArgs {
        /// A set of predefined conditions for the access level and a combining function.
        /// Structure is documented below.
        #[builder(into, default)]
        pub basic: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::accesscontextmanager::AccessLevelBasic>,
        >,
        /// Custom access level conditions are set using the Cloud Common Expression Language to represent the necessary conditions for the level to apply to a request.
        /// See CEL spec at: https://github.com/google/cel-spec.
        /// Structure is documented below.
        #[builder(into, default)]
        pub custom: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::accesscontextmanager::AccessLevelCustom>,
        >,
        /// Description of the AccessLevel and its use. Does not affect behavior.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Resource name for the Access Level. The short_name component must begin
        /// with a letter and only include alphanumeric and '_'.
        /// Format: accessPolicies/{policy_id}/accessLevels/{short_name}
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The AccessPolicy this AccessLevel lives in.
        /// Format: accessPolicies/{policy_id}
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
        /// Human readable title. Must be unique within the Policy.
        #[builder(into)]
        pub title: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessLevelResult {
        /// A set of predefined conditions for the access level and a combining function.
        /// Structure is documented below.
        pub basic: pulumi_wasm_rust::Output<
            Option<super::super::types::accesscontextmanager::AccessLevelBasic>,
        >,
        /// Custom access level conditions are set using the Cloud Common Expression Language to represent the necessary conditions for the level to apply to a request.
        /// See CEL spec at: https://github.com/google/cel-spec.
        /// Structure is documented below.
        pub custom: pulumi_wasm_rust::Output<
            Option<super::super::types::accesscontextmanager::AccessLevelCustom>,
        >,
        /// Description of the AccessLevel and its use. Does not affect behavior.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource name for the Access Level. The short_name component must begin
        /// with a letter and only include alphanumeric and '_'.
        /// Format: accessPolicies/{policy_id}/accessLevels/{short_name}
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The AccessPolicy this AccessLevel lives in.
        /// Format: accessPolicies/{policy_id}
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Human readable title. Must be unique within the Policy.
        pub title: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccessLevelArgs,
    ) -> AccessLevelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let basic_binding = args.basic.get_output(context).get_inner();
        let custom_binding = args.custom.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let title_binding = args.title.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/accessLevel:AccessLevel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "basic".into(),
                    value: &basic_binding,
                },
                register_interface::ObjectField {
                    name: "custom".into(),
                    value: &custom_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "title".into(),
                    value: &title_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccessLevelResult {
            basic: pulumi_wasm_rust::__private::into_domain(o.extract_field("basic")),
            custom: pulumi_wasm_rust::__private::into_domain(o.extract_field("custom")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            title: pulumi_wasm_rust::__private::into_domain(o.extract_field("title")),
        }
    }
}
