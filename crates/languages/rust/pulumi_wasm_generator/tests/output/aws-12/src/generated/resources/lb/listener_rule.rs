/// Provides a Load Balancer Listener Rule resource.
///
/// > **Note:** `aws.alb.ListenerRule` is known as `aws.lb.ListenerRule`. The functionality is identical.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   frontEnd:
///     type: aws:lb:LoadBalancer
///     name: front_end
///   frontEndListener:
///     type: aws:lb:Listener
///     name: front_end
///   static:
///     type: aws:lb:ListenerRule
///     properties:
///       listenerArn: ${frontEndListener.arn}
///       priority: 100
///       actions:
///         - type: forward
///           targetGroupArn: ${staticAwsLbTargetGroup.arn}
///       conditions:
///         - pathPattern:
///             values:
///               - /static/*
///         - hostHeader:
///             values:
///               - example.com
///   # Forward action
///   hostBasedWeightedRouting:
///     type: aws:lb:ListenerRule
///     name: host_based_weighted_routing
///     properties:
///       listenerArn: ${frontEndListener.arn}
///       priority: 99
///       actions:
///         - type: forward
///           targetGroupArn: ${staticAwsLbTargetGroup.arn}
///       conditions:
///         - hostHeader:
///             values:
///               - my-service.*.mycompany.io
///   # Weighted Forward action
///   hostBasedRouting:
///     type: aws:lb:ListenerRule
///     name: host_based_routing
///     properties:
///       listenerArn: ${frontEndListener.arn}
///       priority: 99
///       actions:
///         - type: forward
///           forward:
///             targetGroups:
///               - arn: ${main.arn}
///                 weight: 80
///               - arn: ${canary.arn}
///                 weight: 20
///             stickiness:
///               enabled: true
///               duration: 600
///       conditions:
///         - hostHeader:
///             values:
///               - my-service.*.mycompany.io
///   # Redirect action
///   redirectHttpToHttps:
///     type: aws:lb:ListenerRule
///     name: redirect_http_to_https
///     properties:
///       listenerArn: ${frontEndListener.arn}
///       actions:
///         - type: redirect
///           redirect:
///             port: '443'
///             protocol: HTTPS
///             statusCode: HTTP_301
///       conditions:
///         - httpHeader:
///             httpHeaderName: X-Forwarded-For
///             values:
///               - 192.168.1.*
///   # Fixed-response action
///   healthCheck:
///     type: aws:lb:ListenerRule
///     name: health_check
///     properties:
///       listenerArn: ${frontEndListener.arn}
///       actions:
///         - type: fixed-response
///           fixedResponse:
///             contentType: text/plain
///             messageBody: HEALTHY
///             statusCode: '200'
///       conditions:
///         - queryStrings:
///             - key: health
///               value: check
///             - value: bar
///   # Authenticate-cognito Action
///   pool:
///     type: aws:cognito:UserPool
///   client:
///     type: aws:cognito:UserPoolClient
///   domain:
///     type: aws:cognito:UserPoolDomain
///   admin:
///     type: aws:lb:ListenerRule
///     properties:
///       listenerArn: ${frontEndListener.arn}
///       actions:
///         - type: authenticate-cognito
///           authenticateCognito:
///             userPoolArn: ${pool.arn}
///             userPoolClientId: ${client.id}
///             userPoolDomain: ${domain.domain}
///         - type: forward
///           targetGroupArn: ${staticAwsLbTargetGroup.arn}
///   # Authenticate-oidc Action
///   oidc:
///     type: aws:lb:ListenerRule
///     properties:
///       listenerArn: ${frontEndListener.arn}
///       actions:
///         - type: authenticate-oidc
///           authenticateOidc:
///             authorizationEndpoint: https://example.com/authorization_endpoint
///             clientId: client_id
///             clientSecret: client_secret
///             issuer: https://example.com
///             tokenEndpoint: https://example.com/token_endpoint
///             userInfoEndpoint: https://example.com/user_info_endpoint
///         - type: forward
///           targetGroupArn: ${staticAwsLbTargetGroup.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import rules using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:lb/listenerRule:ListenerRule front_end arn:aws:elasticloadbalancing:us-west-2:187416307283:listener-rule/app/test/8e4497da625e2d8a/9ab28ade35828f96/67b3d2d36dd7c26b
/// ```
pub mod listener_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerRuleArgs {
        /// An Action block. Action blocks are documented below.
        #[builder(into)]
        pub actions: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::lb::ListenerRuleAction>,
        >,
        /// A Condition block. Multiple condition blocks of different types can be set and all must be satisfied for the rule to match. Condition blocks are documented below.
        #[builder(into)]
        pub conditions: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::lb::ListenerRuleCondition>,
        >,
        /// The ARN of the listener to which to attach the rule.
        #[builder(into)]
        pub listener_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The priority for the rule between `1` and `50000`. Leaving it unset will automatically set the rule with next available priority after currently existing highest rule. A listener can't have multiple rules with the same priority.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ListenerRuleResult {
        /// An Action block. Action blocks are documented below.
        pub actions: pulumi_wasm_rust::Output<
            Vec<super::super::types::lb::ListenerRuleAction>,
        >,
        /// The ARN of the rule (matches `id`)
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A Condition block. Multiple condition blocks of different types can be set and all must be satisfied for the rule to match. Condition blocks are documented below.
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::lb::ListenerRuleCondition>,
        >,
        /// The ARN of the listener to which to attach the rule.
        pub listener_arn: pulumi_wasm_rust::Output<String>,
        /// The priority for the rule between `1` and `50000`. Leaving it unset will automatically set the rule with next available priority after currently existing highest rule. A listener can't have multiple rules with the same priority.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ListenerRuleArgs,
    ) -> ListenerRuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_output(context).get_inner();
        let conditions_binding = args.conditions.get_output(context).get_inner();
        let listener_arn_binding = args.listener_arn.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lb/listenerRule:ListenerRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "conditions".into(),
                    value: &conditions_binding,
                },
                register_interface::ObjectField {
                    name: "listenerArn".into(),
                    value: &listener_arn_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ListenerRuleResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("actions"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            conditions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("conditions"),
            ),
            listener_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("listenerArn"),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
