pub mod get_resolver_firewall_domain_list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallDomainListArgs {
        /// The ID of the domain list.
        ///
        /// The following attribute is additionally exported:
        #[builder(into)]
        pub firewall_domain_list_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverFirewallDomainListResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub creation_time: pulumi_wasm_rust::Output<String>,
        pub creator_request_id: pulumi_wasm_rust::Output<String>,
        pub domain_count: pulumi_wasm_rust::Output<i32>,
        pub firewall_domain_list_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub managed_owner_name: pulumi_wasm_rust::Output<String>,
        pub modification_time: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub status_message: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetResolverFirewallDomainListArgs,
    ) -> GetResolverFirewallDomainListResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let firewall_domain_list_id_binding = args.firewall_domain_list_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverFirewallDomainList:getResolverFirewallDomainList"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "firewallDomainListId".into(),
                    value: &firewall_domain_list_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "creatorRequestId".into(),
                },
                register_interface::ResultField {
                    name: "domainCount".into(),
                },
                register_interface::ResultField {
                    name: "firewallDomainListId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "managedOwnerName".into(),
                },
                register_interface::ResultField {
                    name: "modificationTime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusMessage".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResolverFirewallDomainListResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            creator_request_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creatorRequestId").unwrap(),
            ),
            domain_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainCount").unwrap(),
            ),
            firewall_domain_list_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallDomainListId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            managed_owner_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedOwnerName").unwrap(),
            ),
            modification_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modificationTime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusMessage").unwrap(),
            ),
        }
    }
}
