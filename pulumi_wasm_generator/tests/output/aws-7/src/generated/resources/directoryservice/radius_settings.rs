/// Manages a directory's multi-factor authentication (MFA) using a Remote Authentication Dial In User Service (RADIUS) server.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = radius_settings::create(
///         "example",
///         RadiusSettingsArgs::builder()
///             .authentication_protocol("PAP")
///             .directory_id("${exampleAwsDirectoryServiceDirectory.id}")
///             .display_label("example")
///             .radius_port(1812)
///             .radius_retries(4)
///             .radius_servers(vec!["10.0.1.5",])
///             .radius_timeout(1)
///             .shared_secret("12345678")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RADIUS settings using the directory ID. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/radiusSettings:RadiusSettings example d-926724cf57
/// ```
pub mod radius_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RadiusSettingsArgs {
        /// The protocol specified for your RADIUS endpoints. Valid values: `PAP`, `CHAP`, `MS-CHAPv1`, `MS-CHAPv2`.
        #[builder(into)]
        pub authentication_protocol: pulumi_wasm_rust::Output<String>,
        /// The identifier of the directory for which you want to manager RADIUS settings.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// Display label.
        #[builder(into)]
        pub display_label: pulumi_wasm_rust::Output<String>,
        /// The port that your RADIUS server is using for communications. Your self-managed network must allow inbound traffic over this port from the AWS Directory Service servers.
        #[builder(into)]
        pub radius_port: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of times that communication with the RADIUS server is attempted. Minimum value of `0`. Maximum value of `10`.
        #[builder(into)]
        pub radius_retries: pulumi_wasm_rust::Output<i32>,
        /// An array of strings that contains the fully qualified domain name (FQDN) or IP addresses of the RADIUS server endpoints, or the FQDN or IP addresses of your RADIUS server load balancer.
        #[builder(into)]
        pub radius_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The amount of time, in seconds, to wait for the RADIUS server to respond. Minimum value of `1`. Maximum value of `50`.
        #[builder(into)]
        pub radius_timeout: pulumi_wasm_rust::Output<i32>,
        /// Required for enabling RADIUS on the directory.
        #[builder(into)]
        pub shared_secret: pulumi_wasm_rust::Output<String>,
        /// Not currently used.
        #[builder(into, default)]
        pub use_same_username: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct RadiusSettingsResult {
        /// The protocol specified for your RADIUS endpoints. Valid values: `PAP`, `CHAP`, `MS-CHAPv1`, `MS-CHAPv2`.
        pub authentication_protocol: pulumi_wasm_rust::Output<String>,
        /// The identifier of the directory for which you want to manager RADIUS settings.
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// Display label.
        pub display_label: pulumi_wasm_rust::Output<String>,
        /// The port that your RADIUS server is using for communications. Your self-managed network must allow inbound traffic over this port from the AWS Directory Service servers.
        pub radius_port: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of times that communication with the RADIUS server is attempted. Minimum value of `0`. Maximum value of `10`.
        pub radius_retries: pulumi_wasm_rust::Output<i32>,
        /// An array of strings that contains the fully qualified domain name (FQDN) or IP addresses of the RADIUS server endpoints, or the FQDN or IP addresses of your RADIUS server load balancer.
        pub radius_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The amount of time, in seconds, to wait for the RADIUS server to respond. Minimum value of `1`. Maximum value of `50`.
        pub radius_timeout: pulumi_wasm_rust::Output<i32>,
        /// Required for enabling RADIUS on the directory.
        pub shared_secret: pulumi_wasm_rust::Output<String>,
        /// Not currently used.
        pub use_same_username: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RadiusSettingsArgs) -> RadiusSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_protocol_binding = args.authentication_protocol.get_inner();
        let directory_id_binding = args.directory_id.get_inner();
        let display_label_binding = args.display_label.get_inner();
        let radius_port_binding = args.radius_port.get_inner();
        let radius_retries_binding = args.radius_retries.get_inner();
        let radius_servers_binding = args.radius_servers.get_inner();
        let radius_timeout_binding = args.radius_timeout.get_inner();
        let shared_secret_binding = args.shared_secret.get_inner();
        let use_same_username_binding = args.use_same_username.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directoryservice/radiusSettings:RadiusSettings".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationProtocol".into(),
                    value: &authentication_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "displayLabel".into(),
                    value: &display_label_binding,
                },
                register_interface::ObjectField {
                    name: "radiusPort".into(),
                    value: &radius_port_binding,
                },
                register_interface::ObjectField {
                    name: "radiusRetries".into(),
                    value: &radius_retries_binding,
                },
                register_interface::ObjectField {
                    name: "radiusServers".into(),
                    value: &radius_servers_binding,
                },
                register_interface::ObjectField {
                    name: "radiusTimeout".into(),
                    value: &radius_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "sharedSecret".into(),
                    value: &shared_secret_binding,
                },
                register_interface::ObjectField {
                    name: "useSameUsername".into(),
                    value: &use_same_username_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticationProtocol".into(),
                },
                register_interface::ResultField {
                    name: "directoryId".into(),
                },
                register_interface::ResultField {
                    name: "displayLabel".into(),
                },
                register_interface::ResultField {
                    name: "radiusPort".into(),
                },
                register_interface::ResultField {
                    name: "radiusRetries".into(),
                },
                register_interface::ResultField {
                    name: "radiusServers".into(),
                },
                register_interface::ResultField {
                    name: "radiusTimeout".into(),
                },
                register_interface::ResultField {
                    name: "sharedSecret".into(),
                },
                register_interface::ResultField {
                    name: "useSameUsername".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RadiusSettingsResult {
            authentication_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationProtocol").unwrap(),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryId").unwrap(),
            ),
            display_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayLabel").unwrap(),
            ),
            radius_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("radiusPort").unwrap(),
            ),
            radius_retries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("radiusRetries").unwrap(),
            ),
            radius_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("radiusServers").unwrap(),
            ),
            radius_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("radiusTimeout").unwrap(),
            ),
            shared_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedSecret").unwrap(),
            ),
            use_same_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useSameUsername").unwrap(),
            ),
        }
    }
}
