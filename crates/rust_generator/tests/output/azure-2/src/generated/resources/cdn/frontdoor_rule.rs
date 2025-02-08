/// Manages a Front Door (standard/premium) Rule.
///
/// !>**IMPORTANT:** The Rules resource **must** include a `depends_on` meta-argument which references the `azure.cdn.FrontdoorOrigin` and the `azure.cdn.FrontdoorOriginGroup`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-cdn-frontdoor
///       location: West Europe
///   exampleFrontdoorProfile:
///     type: azure:cdn:FrontdoorProfile
///     name: example
///     properties:
///       name: example-profile
///       resourceGroupName: ${example.name}
///       skuName: Premium_AzureFrontDoor
///   exampleFrontdoorEndpoint:
///     type: azure:cdn:FrontdoorEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       tags:
///         endpoint: contoso.com
///   exampleFrontdoorOriginGroup:
///     type: azure:cdn:FrontdoorOriginGroup
///     name: example
///     properties:
///       name: example-originGroup
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       sessionAffinityEnabled: true
///       restoreTrafficTimeToHealedOrNewEndpointInMinutes: 10
///       healthProbe:
///         intervalInSeconds: 240
///         path: /healthProbe
///         protocol: Https
///         requestType: GET
///       loadBalancing:
///         additionalLatencyInMilliseconds: 0
///         sampleSize: 16
///         successfulSamplesRequired: 3
///   exampleFrontdoorOrigin:
///     type: azure:cdn:FrontdoorOrigin
///     name: example
///     properties:
///       name: example-origin
///       cdnFrontdoorOriginGroupId: ${exampleFrontdoorOriginGroup.id}
///       enabled: true
///       certificateNameCheckEnabled: false
///       hostName: ${exampleFrontdoorEndpoint.hostName}
///       httpPort: 80
///       httpsPort: 443
///       originHostHeader: contoso.com
///       priority: 1
///       weight: 500
///   exampleFrontdoorRuleSet:
///     type: azure:cdn:FrontdoorRuleSet
///     name: example
///     properties:
///       name: exampleruleset
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///   exampleFrontdoorRule:
///     type: azure:cdn:FrontdoorRule
///     name: example
///     properties:
///       name: examplerule
///       cdnFrontdoorRuleSetId: ${exampleFrontdoorRuleSet.id}
///       order: 1
///       behaviorOnMatch: Continue
///       actions:
///         routeConfigurationOverrideAction:
///           cdnFrontdoorOriginGroupId: ${exampleFrontdoorOriginGroup.id}
///           forwardingProtocol: HttpsOnly
///           queryStringCachingBehavior: IncludeSpecifiedQueryStrings
///           queryStringParameters:
///             - foo
///             - clientIp={client_ip}
///           compressionEnabled: true
///           cacheBehavior: OverrideIfOriginMissing
///           cacheDuration: 365.23:59:59
///         urlRedirectAction:
///           redirectType: PermanentRedirect
///           redirectProtocol: MatchRequest
///           queryString: clientIp={client_ip}
///           destinationPath: /exampleredirection
///           destinationHostname: contoso.com
///           destinationFragment: UrlRedirect
///       conditions:
///         hostNameConditions:
///           - operator: Equal
///             negateCondition: false
///             matchValues:
///               - www.contoso.com
///               - images.contoso.com
///               - video.contoso.com
///             transforms:
///               - Lowercase
///               - Trim
///         isDeviceConditions:
///           - operator: Equal
///             negateCondition: false
///             matchValues: Mobile
///         postArgsConditions:
///           - postArgsName: customerName
///             operator: BeginsWith
///             matchValues:
///               - J
///               - K
///             transforms:
///               - Uppercase
///         requestMethodConditions:
///           - operator: Equal
///             negateCondition: false
///             matchValues:
///               - DELETE
///         urlFilenameConditions:
///           - operator: Equal
///             negateCondition: false
///             matchValues:
///               - media.mp4
///             transforms:
///               - Lowercase
///               - RemoveNulls
///               - Trim
///     options:
///       dependsOn:
///         - ${exampleFrontdoorOriginGroup}
///         - ${exampleFrontdoorOrigin}
/// ```
///
/// ## Specifying IP Address Ranges
///
/// When specifying IP address ranges in the `socket_address_condition` and the `remote_address_condition` `match_values` use the following format:
///
/// Use `CIDR` notation when specifying IP address blocks. This means that the syntax for an IP address block is the base IP address followed by a forward slash and the prefix size For example:
///
/// * `IPv4` example: `5.5.5.64/26` matches any requests that arrive from addresses `5.5.5.64` through `5.5.5.127`.
/// * `IPv6` example: `1:2:3:/48` matches any requests that arrive from addresses `1:2:3:0:0:0:0:0` through `1:2:3:ffff:ffff:ffff:ffff:ffff`.
///
/// When you specify multiple IP addresses and IP address blocks, `OR` logic is applied.
///
/// * `IPv4` example: if you add two IP addresses `1.2.3.4` and `10.20.30.40`, the condition is matched for any requests that arrive from either address `1.2.3.4` or `10.20.30.40`.
/// * `IPv6` example: if you add two IP addresses `1:2:3:4:5:6:7:8` and `10:20:30:40:50:60:70:80`, the condition is matched for any requests that arrive from either address `1:2:3:4:5:6:7:8` or `10:20:30:40:50:60:70:80`.
///
/// ---
///
/// ## Action Server Variables
///
/// Rule Set server variables provide access to structured information about the request. You can use server variables to dynamically change the request/response headers or URL rewrite paths/query strings, for example, when a new page load or when a form is posted.
///
/// ### Supported Action Server Variables
///
/// | Variable name | Description |
/// |---------------|-------------|
/// | `socket_ip`      | The IP address of the direct connection to Front Door Profiles edge. If the client used an HTTP proxy or a load balancer to send the request, the value of `socket_ip` is the IP address of the proxy or load balancer. |
/// | `client_ip`      | The IP address of the client that made the original request. If there was an `X-Forwarded-For` header in the request, then the client IP address is picked from the header. |
/// | `client_port`    | The IP port of the client that made the request. |
/// | `hostname`       | The host name in the request from the client. |
/// | `geo_country`    | Indicates the requester's country/region of origin through its country/region code. |
/// | `http_method`    | The method used to make the URL request, such as `GET` or `POST`. |
/// | `http_version`   | The request protocol. Usually `HTTP/1.0`, `HTTP/1.1`, or `HTTP/2.0`. |
/// | `query_string`   | The list of variable/value pairs that follows the "?" in the requested URL. For example, in the request `http://contoso.com:8080/article.aspx?id=123&title=fabrikam`, the `query_string` value will be `id=123&title=fabrikam`. |
/// | `request_scheme` | The request scheme: `http` or `https`. |
/// | `request_uri`    | The full original request URI (with arguments). For example, in the request `http://contoso.com:8080/article.aspx?id=123&title=fabrikam`, the `request_uri` value will be `/article.aspx?id=123&title=fabrikam`. |
/// | `ssl_protocol`   | The protocol of an established TLS connection. |
/// | `server_port`    | The port of the server that accepted a request. |
/// | `url_path`       | Identifies the specific resource in the host that the web client wants to access. This is the part of the request URI without the arguments. For example, in the request `http://contoso.com:8080/article.aspx?id=123&title=fabrikam`, the `uri_path` value will be `/article.aspx`. |
///
/// ### Action Server Variable Format
///
/// Server variables can be specified using the following formats:
///
/// * `{variable}` - Include the entire server variable. For example, if the client IP address is `111.222.333.444` then the `{client_ip}` token would evaluate to `111.222.333.444`.
///
/// * `{variable:offset}` - Include the server variable after a specific offset, until the end of the variable. The offset is zero-based. For example, if the client IP address is `111.222.333.444` then the `{client_ip:3}` token would evaluate to `.222.333.444`.
///
/// * `{variable:offset:length}` - Include the server variable after a specific offset, up to the specified length. The offset is zero-based. For example, if the client IP address is `111.222.333.444` then the `{client_ip:4:3}` token would evaluate to `222`.
///
/// ### Action Server Variables Support
///
/// Action Server variables are supported on the following actions:
///
/// * `route_configuration_override_action`
/// * `request_header_action`
/// * `response_header_action`
/// * `url_redirect_action`
/// * `url_rewrite_action`
///
/// ---
///
/// ## Condition Operator list
///
/// For rules that accept values from the standard operator list, the following operators are valid:
///
/// | Operator                   | Description | Condition Value |
/// |----------------------------|-------------|-----------------|
/// | Any                        |Matches when there is any value, regardless of what it is. | Any |
/// | Equal                      | Matches when the value exactly matches the specified string. | Equal |
/// | Contains                   | Matches when the value contains the specified string. | Contains |
/// | Less Than                  | Matches when the length of the value is less than the specified integer. | LessThan |
/// | Greater Than               | Matches when the length of the value is greater than the specified integer. | GreaterThan |
/// | Less Than or Equal         | Matches when the length of the value is less than or equal to the specified integer. | LessThanOrEqual |
/// | Greater Than or Equal      | Matches when the length of the value is greater than or equal to the specified integer. | GreaterThanOrEqual |
/// | Begins With                | Matches when the value begins with the specified string. | BeginsWith |
/// | Ends With                  | Matches when the value ends with the specified string. | EndsWith |
/// | RegEx                      | Matches when the value matches the specified regular expression. See below for further details. | RegEx |
/// | Not Any                    | Matches when there is no value. | Any and negateCondition = true |
/// | Not Equal                  | Matches when the value does not match the specified string. | Equal and negateCondition : true |
/// | Not Contains               | Matches when the value does not contain the specified string. | Contains and negateCondition = true |
/// | Not Less Than              | Matches when the length of the value is not less than the specified integer. | LessThan and negateCondition = true |
/// | Not Greater Than           | Matches when the length of the value is not greater than the specified integer. | GreaterThan and negateCondition = true |
/// | Not Less Than or Equal     | Matches when the length of the value is not less than or equal to the specified integer. | LessThanOrEqual and negateCondition = true |
/// | Not Greater Than or Equals | Matches when the length of the value is not greater than or equal to the specified integer. | GreaterThanOrEqual and negateCondition = true |
/// | Not Begins With            | Matches when the value does not begin with the specified string. | BeginsWith and negateCondition = true |
/// | Not Ends With              | Matches when the value does not end with the specified string. | EndsWith and negateCondition = true |
/// | Not RegEx                  | Matches when the value does not match the specified regular expression. See `Condition Regular Expressions` for further details. | RegEx and negateCondition = true |
///
/// ---
///
/// ## Condition Regular Expressions
///
/// Regular expressions **don't** support the following operations:
///
/// * Backreferences and capturing subexpressions.
/// * Arbitrary zero-width assertions.
/// * Subroutine references and recursive patterns.
/// * Conditional patterns.
/// * Backtracking control verbs.
/// * The `\C` single-byte directive.
/// * The `\R` newline match directive.
/// * The `\K` start of match reset directive.
/// * Callouts and embedded code.
/// * Atomic grouping and possessive quantifiers.
///
/// ---
///
/// ## Condition Transform List
///
/// For rules that can transform strings, the following transforms are valid:
///
/// | Transform   | Description |
/// |-------------|-------------|
/// | Lowercase   | Converts the string to the lowercase representation. |
/// | Uppercase   | Converts the string to the uppercase representation. |
/// | Trim        | Trims leading and trailing whitespace from the string. |
/// | RemoveNulls | Removes null values from the string. |
/// | URLEncode   | URL-encodes the string. |
/// | URLDecode   | URL-decodes the string. |
///
/// ---
///
/// ## Import
///
/// Front Door Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorRule:FrontdoorRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/ruleSets/ruleSet1/rules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod frontdoor_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorRuleArgs {
        /// An `actions` block as defined below.
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cdn::FrontdoorRuleActions,
        >,
        /// If this rule is a match should the rules engine continue processing the remaining rules or stop? Possible values are `Continue` and `Stop`. Defaults to `Continue`.
        #[builder(into, default)]
        pub behavior_on_match: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the Front Door Rule Set for this Front Door Rule. Changing this forces a new Front Door Rule to be created.
        #[builder(into)]
        pub cdn_frontdoor_rule_set_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `conditions` block as defined below.
        #[builder(into, default)]
        pub conditions: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cdn::FrontdoorRuleConditions>,
        >,
        /// The name which should be used for this Front Door Rule. Possible values must be between 1 and 260 characters in length, begin with a letter and may contain only letters and numbers. Changing this forces a new Front Door Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The order in which the rules will be applied for the Front Door Endpoint. The order value should be sequential and begin at `1`(e.g. `1`, `2`, `3`...). A Front Door Rule with a lesser order value will be applied before a rule with a greater order value.
        ///
        /// ->**NOTE:** If the Front Door Rule has an order value of `0` they do not require any conditions and the actions will always be applied.
        #[builder(into)]
        pub order: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct FrontdoorRuleResult {
        /// An `actions` block as defined below.
        pub actions: pulumi_gestalt_rust::Output<
            super::super::types::cdn::FrontdoorRuleActions,
        >,
        /// If this rule is a match should the rules engine continue processing the remaining rules or stop? Possible values are `Continue` and `Stop`. Defaults to `Continue`.
        pub behavior_on_match: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the Front Door Rule Set for this Front Door Rule. Changing this forces a new Front Door Rule to be created.
        pub cdn_frontdoor_rule_set_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Front Door Rule Set containing this Front Door Rule.
        pub cdn_frontdoor_rule_set_name: pulumi_gestalt_rust::Output<String>,
        /// A `conditions` block as defined below.
        pub conditions: pulumi_gestalt_rust::Output<
            Option<super::super::types::cdn::FrontdoorRuleConditions>,
        >,
        /// The name which should be used for this Front Door Rule. Possible values must be between 1 and 260 characters in length, begin with a letter and may contain only letters and numbers. Changing this forces a new Front Door Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The order in which the rules will be applied for the Front Door Endpoint. The order value should be sequential and begin at `1`(e.g. `1`, `2`, `3`...). A Front Door Rule with a lesser order value will be applied before a rule with a greater order value.
        ///
        /// ->**NOTE:** If the Front Door Rule has an order value of `0` they do not require any conditions and the actions will always be applied.
        pub order: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FrontdoorRuleArgs,
    ) -> FrontdoorRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_output(context).get_inner();
        let behavior_on_match_binding = args
            .behavior_on_match
            .get_output(context)
            .get_inner();
        let cdn_frontdoor_rule_set_id_binding = args
            .cdn_frontdoor_rule_set_id
            .get_output(context)
            .get_inner();
        let conditions_binding = args.conditions.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let order_binding = args.order.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorRule:FrontdoorRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "behaviorOnMatch".into(),
                    value: &behavior_on_match_binding,
                },
                register_interface::ObjectField {
                    name: "cdnFrontdoorRuleSetId".into(),
                    value: &cdn_frontdoor_rule_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "conditions".into(),
                    value: &conditions_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "order".into(),
                    value: &order_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FrontdoorRuleResult {
            actions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("actions"),
            ),
            behavior_on_match: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("behaviorOnMatch"),
            ),
            cdn_frontdoor_rule_set_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdnFrontdoorRuleSetId"),
            ),
            cdn_frontdoor_rule_set_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdnFrontdoorRuleSetName"),
            ),
            conditions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("conditions"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            order: pulumi_gestalt_rust::__private::into_domain(o.extract_field("order")),
        }
    }
}
