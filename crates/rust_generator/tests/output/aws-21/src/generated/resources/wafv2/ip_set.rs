/// Provides a WAFv2 IP Set Resource
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:wafv2:IpSet
///     properties:
///       name: example
///       description: Example IP set
///       scope: REGIONAL
///       ipAddressVersion: IPV4
///       addresses:
///         - 1.2.3.4/32
///         - 5.6.7.8/32
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAFv2 IP Sets using `ID/name/scope`. For example:
///
/// ```sh
/// $ pulumi import aws:wafv2/ipSet:IpSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc/example/REGIONAL
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ip_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IpSetArgs {
        /// Contains an array of strings that specifies zero or more IP addresses or blocks of IP addresses. All addresses must be specified using Classless Inter-Domain Routing (CIDR) notation. WAF supports all IPv4 and IPv6 CIDR ranges except for `/0`.
        #[builder(into, default)]
        pub addresses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A friendly description of the IP set.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify IPV4 or IPV6. Valid values are `IPV4` or `IPV6`.
        #[builder(into)]
        pub ip_address_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A friendly name of the IP set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the Region US East (N. Virginia).
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IpSetResult {
        /// Contains an array of strings that specifies zero or more IP addresses or blocks of IP addresses. All addresses must be specified using Classless Inter-Domain Routing (CIDR) notation. WAF supports all IPv4 and IPv6 CIDR ranges except for `/0`.
        pub addresses: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Amazon Resource Name (ARN) of the IP set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A friendly description of the IP set.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specify IPV4 or IPV6. Valid values are `IPV4` or `IPV6`.
        pub ip_address_version: pulumi_gestalt_rust::Output<String>,
        pub lock_token: pulumi_gestalt_rust::Output<String>,
        /// A friendly name of the IP set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the Region US East (N. Virginia).
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IpSetArgs,
    ) -> IpSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let addresses_binding_1 = args.addresses.get_output(context);
        let addresses_binding = addresses_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let ip_address_version_binding_1 = args.ip_address_version.get_output(context);
        let ip_address_version_binding = ip_address_version_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let scope_binding_1 = args.scope.get_output(context);
        let scope_binding = scope_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafv2/ipSet:IpSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addresses".into(),
                    value: &addresses_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddressVersion".into(),
                    value: &ip_address_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IpSetResult {
            addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addresses"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            ip_address_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddressVersion"),
            ),
            lock_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lockToken"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
