/// Resource for managing an AWS OpenSearch Authorize Vpc Endpoint Access.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:opensearch:AuthorizeVpcEndpointAccess
///     properties:
///       domainName: ${testAwsOpensearchDomain.domainName}
///       account: ${current.accountId}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearch Authorize Vpc Endpoint Access using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/authorizeVpcEndpointAccess:AuthorizeVpcEndpointAccess example authorize_vpc_endpoint_access-id-12345678
/// ```
pub mod authorize_vpc_endpoint_access {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizeVpcEndpointAccessArgs {
        /// AWS account ID to grant access to.
        #[builder(into)]
        pub account: pulumi_wasm_rust::Output<String>,
        /// Name of OpenSearch Service domain to provide access to.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AuthorizeVpcEndpointAccessResult {
        /// AWS account ID to grant access to.
        pub account: pulumi_wasm_rust::Output<String>,
        /// Information about the Amazon Web Services account or service that was provided access to the domain. See authorized principal attribute for further details.
        pub authorized_principals: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::opensearch::AuthorizeVpcEndpointAccessAuthorizedPrincipal,
            >,
        >,
        /// Name of OpenSearch Service domain to provide access to.
        pub domain_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AuthorizeVpcEndpointAccessArgs,
    ) -> AuthorizeVpcEndpointAccessResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_binding = args.account.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/authorizeVpcEndpointAccess:AuthorizeVpcEndpointAccess"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "account".into(),
                    value: &account_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "account".into(),
                },
                register_interface::ResultField {
                    name: "authorizedPrincipals".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthorizeVpcEndpointAccessResult {
            account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("account").unwrap(),
            ),
            authorized_principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedPrincipals").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
        }
    }
}
