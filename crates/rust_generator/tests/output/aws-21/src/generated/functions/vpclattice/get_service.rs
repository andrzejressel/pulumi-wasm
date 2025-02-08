#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// Service name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID or Amazon Resource Name (ARN) of the service.
        #[builder(into, default)]
        pub service_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of tags associated with the service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// ARN of the service.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Type of IAM policy. Either `NONE` or `AWS_IAM`.
        pub auth_type: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the certificate.
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// Custom domain name of the service.
        pub custom_domain_name: pulumi_gestalt_rust::Output<String>,
        /// DNS name of the service.
        pub dns_entries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vpclattice::GetServiceDnsEntry>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub service_identifier: pulumi_gestalt_rust::Output<String>,
        /// Status of the service.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// List of tags associated with the service.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServiceResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auth_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authType"),
            ),
            certificate_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateArn"),
            ),
            custom_domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDomainName"),
            ),
            dns_entries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsEntries"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            service_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceIdentifier"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
