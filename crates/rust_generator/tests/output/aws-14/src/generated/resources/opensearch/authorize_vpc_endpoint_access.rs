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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authorize_vpc_endpoint_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizeVpcEndpointAccessArgs {
        /// AWS account ID to grant access to.
        #[builder(into)]
        pub account: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of OpenSearch Service domain to provide access to.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AuthorizeVpcEndpointAccessResult {
        /// AWS account ID to grant access to.
        pub account: pulumi_gestalt_rust::Output<String>,
        /// Information about the Amazon Web Services account or service that was provided access to the domain. See authorized principal attribute for further details.
        pub authorized_principals: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::opensearch::AuthorizeVpcEndpointAccessAuthorizedPrincipal,
            >,
        >,
        /// Name of OpenSearch Service domain to provide access to.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AuthorizeVpcEndpointAccessArgs,
    ) -> AuthorizeVpcEndpointAccessResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_binding_1 = args.account.get_output(context);
        let account_binding = account_binding_1.get_inner();
        let domain_name_binding_1 = args.domain_name.get_output(context);
        let domain_name_binding = domain_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/authorizeVpcEndpointAccess:AuthorizeVpcEndpointAccess"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AuthorizeVpcEndpointAccessResult {
            account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("account"),
            ),
            authorized_principals: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizedPrincipals"),
            ),
            domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
        }
    }
}
