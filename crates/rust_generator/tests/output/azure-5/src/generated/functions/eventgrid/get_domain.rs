pub mod get_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainArgs {
        /// The name of the EventGrid Domain resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group in which the EventGrid Domain exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDomainResult {
        /// The Endpoint associated with the EventGrid Domain.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as documented below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eventgrid::GetDomainIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        pub inbound_ip_rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eventgrid::GetDomainInboundIpRule>,
        >,
        /// A `input_mapping_default_values` block as defined below.
        pub input_mapping_default_values: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eventgrid::GetDomainInputMappingDefaultValue>,
        >,
        /// A `input_mapping_fields` block as defined below.
        pub input_mapping_fields: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eventgrid::GetDomainInputMappingField>,
        >,
        /// The schema in which incoming events will be published to this domain. Possible values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`.
        pub input_schema: pulumi_wasm_rust::Output<String>,
        /// The Azure Region in which this EventGrid Domain exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary access key associated with the EventGrid Domain.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// Whether or not public network access is allowed for this server.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary access key associated with the EventGrid Domain.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the EventGrid Domain.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDomainArgs,
    ) -> GetDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:eventgrid/getDomain:getDomain".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDomainResult {
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            inbound_ip_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inboundIpRules"),
            ),
            input_mapping_default_values: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inputMappingDefaultValues"),
            ),
            input_mapping_fields: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inputMappingFields"),
            ),
            input_schema: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inputSchema"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
