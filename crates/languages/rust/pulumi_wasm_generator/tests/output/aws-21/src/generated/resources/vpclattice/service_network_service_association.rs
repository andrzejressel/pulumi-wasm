/// Resource for managing an AWS VPC Lattice Service Network Service Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service_network_service_association::create(
///         "example",
///         ServiceNetworkServiceAssociationArgs::builder()
///             .service_identifier("${exampleAwsVpclatticeService.id}")
///             .service_network_identifier("${exampleAwsVpclatticeServiceNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Service Network Service Association using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/serviceNetworkServiceAssociation:ServiceNetworkServiceAssociation example snsa-05e2474658a88f6ba
/// ```
pub mod service_network_service_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceNetworkServiceAssociationArgs {
        /// The ID or Amazon Resource Identifier (ARN) of the service.
        #[builder(into)]
        pub service_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID or Amazon Resource Identifier (ARN) of the service network. You must use the ARN if the resources specified in the operation are in different accounts.
        /// The following arguments are optional:
        #[builder(into)]
        pub service_network_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceNetworkServiceAssociationResult {
        /// The ARN of the Association.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The account that created the association.
        pub created_by: pulumi_wasm_rust::Output<String>,
        /// The custom domain name of the service.
        pub custom_domain_name: pulumi_wasm_rust::Output<String>,
        /// The DNS name of the service.
        pub dns_entries: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::vpclattice::ServiceNetworkServiceAssociationDnsEntry,
            >,
        >,
        /// The ID or Amazon Resource Identifier (ARN) of the service.
        pub service_identifier: pulumi_wasm_rust::Output<String>,
        /// The ID or Amazon Resource Identifier (ARN) of the service network. You must use the ARN if the resources specified in the operation are in different accounts.
        /// The following arguments are optional:
        pub service_network_identifier: pulumi_wasm_rust::Output<String>,
        /// The operations status. Valid Values are CREATE_IN_PROGRESS | ACTIVE | DELETE_IN_PROGRESS | CREATE_FAILED | DELETE_FAILED
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceNetworkServiceAssociationArgs,
    ) -> ServiceNetworkServiceAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let service_identifier_binding = args
            .service_identifier
            .get_output(context)
            .get_inner();
        let service_network_identifier_binding = args
            .service_network_identifier
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpclattice/serviceNetworkServiceAssociation:ServiceNetworkServiceAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "serviceIdentifier".into(),
                    value: &service_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "serviceNetworkIdentifier".into(),
                    value: &service_network_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceNetworkServiceAssociationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_by: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdBy"),
            ),
            custom_domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customDomainName"),
            ),
            dns_entries: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsEntries"),
            ),
            service_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceIdentifier"),
            ),
            service_network_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceNetworkIdentifier"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
