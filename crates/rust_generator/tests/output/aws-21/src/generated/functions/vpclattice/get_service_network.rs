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
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceNetworkArgs,
    ) -> GetServiceNetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let service_network_identifier_binding = args
            .service_network_identifier
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:vpclattice/getServiceNetwork:getServiceNetwork".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceNetworkIdentifier".into(),
                    value: &service_network_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceNetworkResult {
            arn: o.get_field("arn"),
            auth_type: o.get_field("authType"),
            created_at: o.get_field("createdAt"),
            id: o.get_field("id"),
            last_updated_at: o.get_field("lastUpdatedAt"),
            name: o.get_field("name"),
            number_of_associated_services: o.get_field("numberOfAssociatedServices"),
            number_of_associated_vpcs: o.get_field("numberOfAssociatedVpcs"),
            service_network_identifier: o.get_field("serviceNetworkIdentifier"),
            tags: o.get_field("tags"),
        }
    }
}
