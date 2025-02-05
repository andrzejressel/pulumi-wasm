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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RadiusSettingsArgs {
        /// The protocol specified for your RADIUS endpoints. Valid values: `PAP`, `CHAP`, `MS-CHAPv1`, `MS-CHAPv2`.
        #[builder(into)]
        pub authentication_protocol: pulumi_wasm_rust::InputOrOutput<String>,
        /// The identifier of the directory for which you want to manager RADIUS settings.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Display label.
        #[builder(into)]
        pub display_label: pulumi_wasm_rust::InputOrOutput<String>,
        /// The port that your RADIUS server is using for communications. Your self-managed network must allow inbound traffic over this port from the AWS Directory Service servers.
        #[builder(into)]
        pub radius_port: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The maximum number of times that communication with the RADIUS server is attempted. Minimum value of `0`. Maximum value of `10`.
        #[builder(into)]
        pub radius_retries: pulumi_wasm_rust::InputOrOutput<i32>,
        /// An array of strings that contains the fully qualified domain name (FQDN) or IP addresses of the RADIUS server endpoints, or the FQDN or IP addresses of your RADIUS server load balancer.
        #[builder(into)]
        pub radius_servers: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The amount of time, in seconds, to wait for the RADIUS server to respond. Minimum value of `1`. Maximum value of `50`.
        #[builder(into)]
        pub radius_timeout: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Required for enabling RADIUS on the directory.
        #[builder(into)]
        pub shared_secret: pulumi_wasm_rust::InputOrOutput<String>,
        /// Not currently used.
        #[builder(into, default)]
        pub use_same_username: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RadiusSettingsArgs,
    ) -> RadiusSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_protocol_binding = args
            .authentication_protocol
            .get_output(context)
            .get_inner();
        let directory_id_binding = args.directory_id.get_output(context).get_inner();
        let display_label_binding = args.display_label.get_output(context).get_inner();
        let radius_port_binding = args.radius_port.get_output(context).get_inner();
        let radius_retries_binding = args.radius_retries.get_output(context).get_inner();
        let radius_servers_binding = args.radius_servers.get_output(context).get_inner();
        let radius_timeout_binding = args.radius_timeout.get_output(context).get_inner();
        let shared_secret_binding = args.shared_secret.get_output(context).get_inner();
        let use_same_username_binding = args
            .use_same_username
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directoryservice/radiusSettings:RadiusSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        RadiusSettingsResult {
            authentication_protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authenticationProtocol"),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("directoryId"),
            ),
            display_label: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayLabel"),
            ),
            radius_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("radiusPort"),
            ),
            radius_retries: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("radiusRetries"),
            ),
            radius_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("radiusServers"),
            ),
            radius_timeout: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("radiusTimeout"),
            ),
            shared_secret: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sharedSecret"),
            ),
            use_same_username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("useSameUsername"),
            ),
        }
    }
}
