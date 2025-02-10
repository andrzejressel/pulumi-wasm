#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// Domain of the certificate to look up. If set and no certificate is found with this name, an error will be returned.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of key algorithms to filter certificates. By default, ACM does not return all certificate types when searching. See the [ACM API Reference](https://docs.aws.amazon.com/acm/latest/APIReference/API_CertificateDetail.html#ACM-Type-CertificateDetail-KeyAlgorithm) for supported key algorithms.
        #[builder(into, default)]
        pub key_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// If set to true, it sorts the certificates matched by previous criteria by the NotBefore field, returning only the most recent one. If set to false, it returns an error if more than one certificate is found. Defaults to false.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of statuses on which to filter the returned list. Valid values are `PENDING_VALIDATION`, `ISSUED`,
        /// `INACTIVE`, `EXPIRED`, `VALIDATION_TIMED_OUT`, `REVOKED` and `FAILED`. If no value is specified, only certificates in the `ISSUED` state
        /// are returned.
        #[builder(into, default)]
        pub statuses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A mapping of tags, each pair of which must exactly match a pair on the desired certificates.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of types on which to filter the returned list. Valid values are `AMAZON_ISSUED`, `PRIVATE`, and `IMPORTED`.
        #[builder(into, default)]
        pub types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        /// ARN of the found certificate, suitable for referencing in other resources that support ACM certificates.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ACM-issued certificate.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// Certificates forming the requested ACM-issued certificate's chain of trust. The chain consists of the certificate of the issuing CA and the intermediate certificates of any other subordinate CAs.
        pub certificate_chain: pulumi_gestalt_rust::Output<String>,
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Status of the found certificate.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub statuses: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Mapping of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_binding = args.domain.get_output(context);
        let key_types_binding = args.key_types.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let statuses_binding = args.statuses.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let types_binding = args.types.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:acm/getCertificate:getCertificate".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyTypes".into(),
                    value: key_types_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: most_recent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statuses".into(),
                    value: statuses_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "types".into(),
                    value: types_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCertificateResult {
            arn: o.get_field("arn"),
            certificate: o.get_field("certificate"),
            certificate_chain: o.get_field("certificateChain"),
            domain: o.get_field("domain"),
            id: o.get_field("id"),
            key_types: o.get_field("keyTypes"),
            most_recent: o.get_field("mostRecent"),
            status: o.get_field("status"),
            statuses: o.get_field("statuses"),
            tags: o.get_field("tags"),
            types: o.get_field("types"),
        }
    }
}
