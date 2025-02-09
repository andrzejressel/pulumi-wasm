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
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let service_identifier_binding = args.service_identifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:vpclattice/getService:getService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceIdentifier".into(),
                    value: service_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceResult {
            arn: o.get_field("arn"),
            auth_type: o.get_field("authType"),
            certificate_arn: o.get_field("certificateArn"),
            custom_domain_name: o.get_field("customDomainName"),
            dns_entries: o.get_field("dnsEntries"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            service_identifier: o.get_field("serviceIdentifier"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}
