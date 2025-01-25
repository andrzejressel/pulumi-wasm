/// Resource for managing an AWS MediaLive InputSecurityGroup.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:medialive:InputSecurityGroup
///     properties:
///       whitelistRules:
///         - cidr: 10.0.0.8/32
///       tags:
///         ENVIRONMENT: prod
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MediaLive InputSecurityGroup using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:medialive/inputSecurityGroup:InputSecurityGroup example 123456
/// ```
pub mod input_security_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InputSecurityGroupArgs {
        /// A map of tags to assign to the InputSecurityGroup. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whitelist rules. See Whitelist Rules for more details.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub whitelist_rules: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::medialive::InputSecurityGroupWhitelistRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct InputSecurityGroupResult {
        /// ARN of the InputSecurityGroup.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The list of inputs currently using this InputSecurityGroup.
        pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the InputSecurityGroup. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whitelist rules. See Whitelist Rules for more details.
        ///
        /// The following arguments are optional:
        pub whitelist_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::medialive::InputSecurityGroupWhitelistRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InputSecurityGroupArgs,
    ) -> InputSecurityGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_output(context).get_inner();
        let whitelist_rules_binding = args
            .whitelist_rules
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:medialive/inputSecurityGroup:InputSecurityGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "whitelistRules".into(),
                    value: &whitelist_rules_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "inputs".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "whitelistRules".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InputSecurityGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            inputs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputs").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            whitelist_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("whitelistRules").unwrap(),
            ),
        }
    }
}
