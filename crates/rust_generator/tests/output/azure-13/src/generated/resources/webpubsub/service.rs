/// Manages an Azure Web PubSub Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: terraform-webpubsub
///       location: east us
///   exampleService:
///     type: azure:webpubsub:Service
///     name: example
///     properties:
///       name: tfex-webpubsub
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard_S1
///       capacity: 1
///       publicNetworkAccessEnabled: false
///       liveTrace:
///         enabled: true
///         messagingLogsEnabled: true
///         connectivityLogsEnabled: false
///       identity:
///         type: SystemAssigned
/// ```
///
/// ## Import
///
/// Web PubSub services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:webpubsub/service:Service example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/webPubSub/pubsub1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Whether to enable AAD auth? Defaults to `true`.
        #[builder(into, default)]
        pub aad_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the number of units associated with this Web PubSub resource. Valid values are `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `10`, `20`, `30`, `40`, `50`, `60`, `70`, `80`, `90`, `100`, `200`, `300`, `400`, `500`, `600`, `700`, `800`, `900` and `1000`.
        ///
        /// > **NOTE:** The valid capacity range for sku `Free_F1` is `1`, for sku `Premium_P2` is from `100` to `1000`, and from `1` to `100` for sku `Standard_S1` and `Premium_P1`.
        #[builder(into, default)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::webpubsub::ServiceIdentity>,
        >,
        /// A `live_trace` block as defined below.
        #[builder(into, default)]
        pub live_trace: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::webpubsub::ServiceLiveTrace>,
        >,
        /// Whether to enable local auth? Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the Web PubSub service exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Web PubSub service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to enable public network access? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the Web PubSub service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies which SKU to use. Possible values are `Free_F1`, `Standard_S1`, `Premium_P1` and `Premium_P2`.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to request client certificate during TLS handshake? Defaults to `false`.
        #[builder(into, default)]
        pub tls_client_cert_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Whether to enable AAD auth? Defaults to `true`.
        pub aad_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the number of units associated with this Web PubSub resource. Valid values are `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `10`, `20`, `30`, `40`, `50`, `60`, `70`, `80`, `90`, `100`, `200`, `300`, `400`, `500`, `600`, `700`, `800`, `900` and `1000`.
        ///
        /// > **NOTE:** The valid capacity range for sku `Free_F1` is `1`, for sku `Premium_P2` is from `100` to `1000`, and from `1` to `100` for sku `Standard_S1` and `Premium_P1`.
        pub capacity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The publicly accessible IP of the Web PubSub service.
        pub external_ip: pulumi_gestalt_rust::Output<String>,
        /// The FQDN of the Web PubSub service.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::webpubsub::ServiceIdentity>,
        >,
        /// A `live_trace` block as defined below.
        pub live_trace: pulumi_gestalt_rust::Output<
            Option<super::super::types::webpubsub::ServiceLiveTrace>,
        >,
        /// Whether to enable local auth? Defaults to `true`.
        pub local_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the Web PubSub service exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Web PubSub service. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary access key for the Web PubSub service.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string for the Web PubSub service.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable public network access? Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The publicly accessible port of the Web PubSub service which is designed for browser/client use.
        pub public_port: pulumi_gestalt_rust::Output<i32>,
        /// The name of the resource group in which to create the Web PubSub service. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key for the Web PubSub service.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string for the Web PubSub service.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The publicly accessible port of the Web PubSub service which is designed for customer server side use.
        pub server_port: pulumi_gestalt_rust::Output<i32>,
        /// Specifies which SKU to use. Possible values are `Free_F1`, `Standard_S1`, `Premium_P1` and `Premium_P2`.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to request client certificate during TLS handshake? Defaults to `false`.
        pub tls_client_cert_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aad_auth_enabled_binding = args.aad_auth_enabled.get_output(context);
        let capacity_binding = args.capacity.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let live_trace_binding = args.live_trace.get_output(context);
        let local_auth_enabled_binding = args.local_auth_enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tls_client_cert_enabled_binding = args
            .tls_client_cert_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:webpubsub/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aadAuthEnabled".into(),
                    value: aad_auth_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "liveTrace".into(),
                    value: live_trace_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthEnabled".into(),
                    value: local_auth_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: public_network_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsClientCertEnabled".into(),
                    value: tls_client_cert_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceResult {
            aad_auth_enabled: o.get_field("aadAuthEnabled"),
            capacity: o.get_field("capacity"),
            external_ip: o.get_field("externalIp"),
            hostname: o.get_field("hostname"),
            identity: o.get_field("identity"),
            live_trace: o.get_field("liveTrace"),
            local_auth_enabled: o.get_field("localAuthEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            primary_access_key: o.get_field("primaryAccessKey"),
            primary_connection_string: o.get_field("primaryConnectionString"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            public_port: o.get_field("publicPort"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
            secondary_connection_string: o.get_field("secondaryConnectionString"),
            server_port: o.get_field("serverPort"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            tls_client_cert_enabled: o.get_field("tlsClientCertEnabled"),
            version: o.get_field("version"),
        }
    }
}
