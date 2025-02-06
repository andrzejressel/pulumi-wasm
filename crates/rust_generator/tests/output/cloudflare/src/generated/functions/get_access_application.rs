pub mod get_access_application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessApplicationArgs {
        /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The primary hostname and path that Access will secure. Must provide only one of `name`, `domain`.
        #[builder(into, default)]
        pub domain: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Friendly name of the Access Application. Must provide only one of `name`, `domain`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAccessApplicationResult {
        /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Application Audience (AUD) Tag of the application.
        pub aud: pulumi_wasm_rust::Output<String>,
        /// The primary hostname and path that Access will secure. Must provide only one of `name`, `domain`.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Friendly name of the Access Application. Must provide only one of `name`, `domain`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAccessApplicationArgs,
    ) -> GetAccessApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let domain_binding = args.domain.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getAccessApplication:getAccessApplication".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccessApplicationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            aud: pulumi_wasm_rust::__private::into_domain(o.extract_field("aud")),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
