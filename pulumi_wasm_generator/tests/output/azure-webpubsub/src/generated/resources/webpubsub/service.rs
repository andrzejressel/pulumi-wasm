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
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Whether to enable AAD auth? Defaults to `true`.
        #[builder(into, default)]
        pub aad_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the number of units associated with this Web PubSub resource. Valid values are `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `10`, `20`, `30`, `40`, `50`, `60`, `70`, `80`, `90`, `100`, `200`, `300`, `400`, `500`, `600`, `700`, `800`, `900` and `1000`.
        ///
        /// > **NOTE:** The valid capacity range for sku `Free_F1` is `1`, for sku `Premium_P2` is from `100` to `1000`, and from `1` to `100` for sku `Standard_S1` and `Premium_P1`.
        #[builder(into, default)]
        pub capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::webpubsub::ServiceIdentity>,
        >,
        /// A `live_trace` block as defined below.
        #[builder(into, default)]
        pub live_trace: pulumi_wasm_rust::Output<
            Option<super::super::types::webpubsub::ServiceLiveTrace>,
        >,
        /// Whether to enable local auth? Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the Web PubSub service exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Web PubSub service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to enable public network access? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Web PubSub service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies which SKU to use. Possible values are `Free_F1`, `Standard_S1`, `Premium_P1` and `Premium_P2`.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to request client certificate during TLS handshake? Defaults to `false`.
        #[builder(into, default)]
        pub tls_client_cert_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Whether to enable AAD auth? Defaults to `true`.
        pub aad_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the number of units associated with this Web PubSub resource. Valid values are `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `10`, `20`, `30`, `40`, `50`, `60`, `70`, `80`, `90`, `100`, `200`, `300`, `400`, `500`, `600`, `700`, `800`, `900` and `1000`.
        ///
        /// > **NOTE:** The valid capacity range for sku `Free_F1` is `1`, for sku `Premium_P2` is from `100` to `1000`, and from `1` to `100` for sku `Standard_S1` and `Premium_P1`.
        pub capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// The publicly accessible IP of the Web PubSub service.
        pub external_ip: pulumi_wasm_rust::Output<String>,
        /// The FQDN of the Web PubSub service.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::webpubsub::ServiceIdentity>,
        >,
        /// A `live_trace` block as defined below.
        pub live_trace: pulumi_wasm_rust::Output<
            Option<super::super::types::webpubsub::ServiceLiveTrace>,
        >,
        /// Whether to enable local auth? Defaults to `true`.
        pub local_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the Web PubSub service exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Web PubSub service. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary access key for the Web PubSub service.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The primary connection string for the Web PubSub service.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// Whether to enable public network access? Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The publicly accessible port of the Web PubSub service which is designed for browser/client use.
        pub public_port: pulumi_wasm_rust::Output<i32>,
        /// The name of the resource group in which to create the Web PubSub service. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary access key for the Web PubSub service.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string for the Web PubSub service.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The publicly accessible port of the Web PubSub service which is designed for customer server side use.
        pub server_port: pulumi_wasm_rust::Output<i32>,
        /// Specifies which SKU to use. Possible values are `Free_F1`, `Standard_S1`, `Premium_P1` and `Premium_P2`.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to request client certificate during TLS handshake? Defaults to `false`.
        pub tls_client_cert_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aad_auth_enabled_binding = args.aad_auth_enabled.get_inner();
        let capacity_binding = args.capacity.get_inner();
        let identity_binding = args.identity.get_inner();
        let live_trace_binding = args.live_trace.get_inner();
        let local_auth_enabled_binding = args.local_auth_enabled.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let tls_client_cert_enabled_binding = args.tls_client_cert_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:webpubsub/service:Service".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aadAuthEnabled".into(),
                    value: &aad_auth_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "liveTrace".into(),
                    value: &live_trace_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthEnabled".into(),
                    value: &local_auth_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tlsClientCertEnabled".into(),
                    value: &tls_client_cert_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aadAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "externalIp".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "liveTrace".into(),
                },
                register_interface::ResultField {
                    name: "localAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "publicPort".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "serverPort".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tlsClientCertEnabled".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceResult {
            aad_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aadAuthEnabled").unwrap(),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            external_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalIp").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            live_trace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("liveTrace").unwrap(),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionString").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            public_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicPort").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionString").unwrap(),
            ),
            server_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverPort").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tls_client_cert_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsClientCertEnabled").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}