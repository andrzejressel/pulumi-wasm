/// The [Infrastructure Access Target](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/use-cases/ssh/ssh-infrastructure-access/#4-add-a-target) resource allows you to configure Infrastructure Access Targets for an account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = infrastructure_access_target::create(
///         "example",
///         InfrastructureAccessTargetArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .hostname("example-target")
///             .ip(
///                 InfrastructureAccessTargetIp::builder()
///                     .ipv4(
///                         InfrastructureAccessTargetIpIpv4::builder()
///                             .ipAddr("198.51.100.1")
///                             .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
///                             .build_struct(),
///                     )
///                     .ipv6(
///                         InfrastructureAccessTargetIpIpv6::builder()
///                             .ipAddr("2001:db8::")
///                             .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let ipv4OnlyExample = infrastructure_access_target::create(
///         "ipv4OnlyExample",
///         InfrastructureAccessTargetArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .hostname("example-ipv4-only")
///             .ip(
///                 InfrastructureAccessTargetIp::builder()
///                     .ipv4(
///                         InfrastructureAccessTargetIpIpv4::builder()
///                             .ipAddr("198.51.100.1")
///                             .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/infrastructureAccessTarget:InfrastructureAccessTarget example <account_id>
/// ```
///
pub mod infrastructure_access_target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InfrastructureAccessTargetArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A non-unique field that refers to a target.
        #[builder(into)]
        pub hostname: pulumi_wasm_rust::InputOrOutput<String>,
        /// The IPv4/IPv6 address that identifies where to reach a target.
        #[builder(into)]
        pub ip: pulumi_wasm_rust::InputOrOutput<
            super::types::InfrastructureAccessTargetIp,
        >,
    }
    #[allow(dead_code)]
    pub struct InfrastructureAccessTargetResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The date and time at which the target was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// A non-unique field that refers to a target.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The IPv4/IPv6 address that identifies where to reach a target.
        pub ip: pulumi_wasm_rust::Output<super::types::InfrastructureAccessTargetIp>,
        /// The date and time at which the target was last modified.
        pub modified_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InfrastructureAccessTargetArgs,
    ) -> InfrastructureAccessTargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let hostname_binding = args.hostname.get_output(context).get_inner();
        let ip_binding = args.ip.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/infrastructureAccessTarget:InfrastructureAccessTarget"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "ip".into(),
                    value: &ip_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InfrastructureAccessTargetResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            ip: pulumi_wasm_rust::__private::into_domain(o.extract_field("ip")),
            modified_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("modifiedAt"),
            ),
        }
    }
}
