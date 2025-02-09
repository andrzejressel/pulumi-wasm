#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceNetworkArgs {
        /// Identifier of the service network.
        #[builder(into)]
        pub service_network_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceNetworkResult {
        /// ARN of the Service Network.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Authentication type for the service network. Either `NONE` or `AWS_IAM`.
        pub auth_type: pulumi_gestalt_rust::Output<String>,
        /// Date and time the service network was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date and time the service network was last updated.
        pub last_updated_at: pulumi_gestalt_rust::Output<String>,
        /// Name of the service network.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Number of services associated with this service network.
        pub number_of_associated_services: pulumi_gestalt_rust::Output<i32>,
        /// Number of VPCs associated with this service network.
        pub number_of_associated_vpcs: pulumi_gestalt_rust::Output<i32>,
        pub service_network_identifier: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServiceNetworkArgs,
    ) -> GetServiceNetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let service_network_identifier_binding_1 = args
            .service_network_identifier
            .get_output(context);
        let service_network_identifier_binding = service_network_identifier_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:vpclattice/getServiceNetwork:getServiceNetwork".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServiceNetworkResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auth_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authType"),
            ),
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            last_updated_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedAt"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            number_of_associated_services: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numberOfAssociatedServices"),
            ),
            number_of_associated_vpcs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numberOfAssociatedVpcs"),
            ),
            service_network_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceNetworkIdentifier"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
