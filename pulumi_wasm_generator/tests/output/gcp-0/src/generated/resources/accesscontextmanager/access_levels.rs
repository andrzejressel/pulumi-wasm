/// ## Example Usage
///
/// ### Access Context Manager Access Levels Basic
///
///
/// ```yaml
/// resources:
///   access-levels:
///     type: gcp:accesscontextmanager:AccessLevels
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       accessLevels:
///         - name: accessPolicies/${["access-policy"].name}/accessLevels/chromeos_no_lock
///           title: chromeos_no_lock
///           basic:
///             conditions:
///               - devicePolicy:
///                   requireScreenLock: true
///                   osConstraints:
///                     - osType: DESKTOP_CHROME_OS
///                 regions:
///                   - CH
///                   - IT
///                   - US
///         - name: accessPolicies/${["access-policy"].name}/accessLevels/mac_no_lock
///           title: mac_no_lock
///           basic:
///             conditions:
///               - devicePolicy:
///                   requireScreenLock: true
///                   osConstraints:
///                     - osType: DESKTOP_MAC
///                 regions:
///                   - CH
///                   - IT
///                   - US
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: my policy
/// ```
///
/// ## Import
///
/// AccessLevels can be imported using any of these accepted formats:
///
/// * `{{parent}}/accessLevels`
///
/// * `{{parent}}`
///
/// When using the `pulumi import` command, AccessLevels can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/accessLevels:AccessLevels default {{parent}}/accessLevels
/// ```
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/accessLevels:AccessLevels default {{parent}}
/// ```
///
pub mod access_levels {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessLevelsArgs {
        /// The desired Access Levels that should replace all existing Access Levels in the Access Policy.
        /// Structure is documented below.
        #[builder(into, default)]
        pub access_levels: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::accesscontextmanager::AccessLevelsAccessLevel>,
            >,
        >,
        /// The AccessPolicy this AccessLevel lives in.
        /// Format: accessPolicies/{policy_id}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessLevelsResult {
        /// The desired Access Levels that should replace all existing Access Levels in the Access Policy.
        /// Structure is documented below.
        pub access_levels: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::accesscontextmanager::AccessLevelsAccessLevel>,
            >,
        >,
        /// The AccessPolicy this AccessLevel lives in.
        /// Format: accessPolicies/{policy_id}
        ///
        ///
        /// - - -
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccessLevelsArgs,
    ) -> AccessLevelsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_levels_binding = args.access_levels.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/accessLevels:AccessLevels".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessLevels".into(),
                    value: &access_levels_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessLevels".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessLevelsResult {
            access_levels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessLevels").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
        }
    }
}
