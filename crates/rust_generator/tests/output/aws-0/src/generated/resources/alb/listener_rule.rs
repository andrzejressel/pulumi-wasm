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
/// $ pulumi import aws:alb/listenerRule:ListenerRule front_end arn:aws:elasticloadbalancing:us-west-2:187416307283:listener-rule/app/test/8e4497da625e2d8a/9ab28ade35828f96/67b3d2d36dd7c26b
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod listener_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerRuleArgs {
        /// An Action block. Action blocks are documented below.
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::alb::ListenerRuleAction>,
        >,
        /// A Condition block. Multiple condition blocks of different types can be set and all must be satisfied for the rule to match. Condition blocks are documented below.
        #[builder(into)]
        pub conditions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::alb::ListenerRuleCondition>,
        >,
        /// The ARN of the listener to which to attach the rule.
        #[builder(into)]
        pub listener_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The priority for the rule between `1` and `50000`. Leaving it unset will automatically set the rule with next available priority after currently existing highest rule. A listener can't have multiple rules with the same priority.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ListenerRuleResult {
        /// An Action block. Action blocks are documented below.
        pub actions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alb::ListenerRuleAction>,
        >,
        /// The ARN of the rule (matches `id`)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A Condition block. Multiple condition blocks of different types can be set and all must be satisfied for the rule to match. Condition blocks are documented below.
        pub conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alb::ListenerRuleCondition>,
        >,
        /// The ARN of the listener to which to attach the rule.
        pub listener_arn: pulumi_gestalt_rust::Output<String>,
        /// The priority for the rule between `1` and `50000`. Leaving it unset will automatically set the rule with next available priority after currently existing highest rule. A listener can't have multiple rules with the same priority.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListenerRuleArgs,
    ) -> ListenerRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let conditions_binding = args.conditions.get_output(context);
        let listener_arn_binding = args.listener_arn.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:alb/listenerRule:ListenerRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: &actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conditions".into(),
                    value: &conditions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listenerArn".into(),
                    value: &listener_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ListenerRuleResult {
            actions: o.get_field("actions"),
            arn: o.get_field("arn"),
            conditions: o.get_field("conditions"),
            listener_arn: o.get_field("listenerArn"),
            priority: o.get_field("priority"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
