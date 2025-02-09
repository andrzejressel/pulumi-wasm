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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod input_security_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InputSecurityGroupArgs {
        /// A map of tags to assign to the InputSecurityGroup. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whitelist rules. See Whitelist Rules for more details.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub whitelist_rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::medialive::InputSecurityGroupWhitelistRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct InputSecurityGroupResult {
        /// ARN of the InputSecurityGroup.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The list of inputs currently using this InputSecurityGroup.
        pub inputs: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the InputSecurityGroup. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whitelist rules. See Whitelist Rules for more details.
        ///
        /// The following arguments are optional:
        pub whitelist_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::medialive::InputSecurityGroupWhitelistRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InputSecurityGroupArgs,
    ) -> InputSecurityGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let whitelist_rules_binding = args.whitelist_rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:medialive/inputSecurityGroup:InputSecurityGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "whitelistRules".into(),
                    value: whitelist_rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InputSecurityGroupResult {
            arn: o.get_field("arn"),
            inputs: o.get_field("inputs"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            whitelist_rules: o.get_field("whitelistRules"),
        }
    }
}
