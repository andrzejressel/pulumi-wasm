/// Apigee Endpoint Attachment.
///
///
/// To get more information about EndpointAttachment, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.endpointAttachments/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Example Usage
///
/// ### Apigee Endpoint Attachment Basic
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   apigeeEndpointAttachment:
///     type: gcp:apigee:EndpointAttachment
///     name: apigee_endpoint_attachment
///     properties:
///       orgId: ${apigeeOrg.id}
///       endpointAttachmentId: test1
///       location: '{google_compute_service_attachment location}'
///       serviceAttachment: '{google_compute_service_attachment id}'
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// EndpointAttachment can be imported using any of these accepted formats:
///
/// * `{{org_id}}/endpointAttachments/{{endpoint_attachment_id}}`
///
/// * `{{org_id}}/{{endpoint_attachment_id}}`
///
/// When using the `pulumi import` command, EndpointAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/endpointAttachment:EndpointAttachment default {{org_id}}/endpointAttachments/{{endpoint_attachment_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/endpointAttachment:EndpointAttachment default {{org_id}}/{{endpoint_attachment_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointAttachmentArgs {
        /// ID of the endpoint attachment.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub endpoint_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Location of the endpoint attachment.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Apigee Organization associated with the Apigee instance,
        /// in the format `organizations/{{org_name}}`.
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Format: projects/*/regions/*/serviceAttachments/*
        #[builder(into)]
        pub service_attachment: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointAttachmentResult {
        /// State of the endpoint attachment connection to the service attachment.
        pub connection_state: pulumi_gestalt_rust::Output<String>,
        /// ID of the endpoint attachment.
        ///
        ///
        /// - - -
        pub endpoint_attachment_id: pulumi_gestalt_rust::Output<String>,
        /// Host that can be used in either HTTP Target Endpoint directly, or as the host in Target Server.
        pub host: pulumi_gestalt_rust::Output<String>,
        /// Location of the endpoint attachment.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the Endpoint Attachment in the following format:
        /// organizations/{organization}/endpointAttachments/{endpointAttachment}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Apigee Organization associated with the Apigee instance,
        /// in the format `organizations/{{org_name}}`.
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// Format: projects/*/regions/*/serviceAttachments/*
        pub service_attachment: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointAttachmentArgs,
    ) -> EndpointAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_attachment_id_binding = args
            .endpoint_attachment_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let service_attachment_binding = args.service_attachment.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/endpointAttachment:EndpointAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointAttachmentId".into(),
                    value: &endpoint_attachment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAttachment".into(),
                    value: &service_attachment_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointAttachmentResult {
            connection_state: o.get_field("connectionState"),
            endpoint_attachment_id: o.get_field("endpointAttachmentId"),
            host: o.get_field("host"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            org_id: o.get_field("orgId"),
            service_attachment: o.get_field("serviceAttachment"),
        }
    }
}
