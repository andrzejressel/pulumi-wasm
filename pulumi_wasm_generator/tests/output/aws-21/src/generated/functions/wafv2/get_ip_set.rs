pub mod get_ip_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIpSetArgs {
        /// Name of the WAFv2 IP Set.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetIpSetResult {
        /// An array of strings that specifies zero or more IP addresses or blocks of IP addresses in Classless Inter-Domain Routing (CIDR) notation.
        pub addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN of the entity.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the set that helps with identification.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// IP address version of the set.
        pub ip_address_version: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetIpSetArgs,
    ) -> GetIpSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:wafv2/getIpSet:getIpSet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetIpSetResult {
            addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addresses"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ip_address_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipAddressVersion"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            scope: pulumi_wasm_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
