pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// Service name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID or Amazon Resource Name (ARN) of the service.
        #[builder(into, default)]
        pub service_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of tags associated with the service.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// ARN of the service.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Type of IAM policy. Either `NONE` or `AWS_IAM`.
        pub auth_type: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the certificate.
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        /// Custom domain name of the service.
        pub custom_domain_name: pulumi_wasm_rust::Output<String>,
        /// DNS name of the service.
        pub dns_entries: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vpclattice::GetServiceDnsEntry>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub service_identifier: pulumi_wasm_rust::Output<String>,
        /// Status of the service.
        pub status: pulumi_wasm_rust::Output<String>,
        /// List of tags associated with the service.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let service_identifier_binding = args
            .service_identifier
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:vpclattice/getService:getService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceIdentifier".into(),
                    value: &service_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authType".into(),
                },
                register_interface::ResultField {
                    name: "certificateArn".into(),
                },
                register_interface::ResultField {
                    name: "customDomainName".into(),
                },
                register_interface::ResultField {
                    name: "dnsEntries".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serviceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auth_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authType").unwrap(),
            ),
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            custom_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomainName").unwrap(),
            ),
            dns_entries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsEntries").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            service_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceIdentifier").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
