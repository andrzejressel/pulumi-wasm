/// Manages the Registration Info for a Virtual Desktop Host Pool.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("westeurope")
///             .name("example-hostpool")
///             .build_struct(),
///     );
///     let exampleGetHostPoolRegistrationInfo = get_host_pool_registration_info::create(
///         "exampleGetHostPoolRegistrationInfo",
///         GetHostPoolRegistrationInfoArgs::builder()
///             .expiration_date("2022-01-01T23:40:52Z")
///             .hostpool_id("${exampleHostPool.id}")
///             .build_struct(),
///     );
///     let exampleHostPool = host_pool::create(
///         "exampleHostPool",
///         HostPoolArgs::builder()
///             .load_balancer_type("BreadthFirst")
///             .location("${example.location}")
///             .name("example-HP")
///             .resource_group_name("${example.name}")
///             .type_("Pooled")
///             .validate_environment(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AVD Registration Infos can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:desktopvirtualization/getHostPoolRegistrationInfo:getHostPoolRegistrationInfo example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.DesktopVirtualization/hostPools/pool1/registrationInfo/default
/// ```
///
pub mod get_host_pool_registration_info {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct getHostPoolRegistrationInfoArgs {
        /// A valid `RFC3339Time` for the expiration of the token..
        #[builder(into)]
        pub expiration_date: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Desktop Host Pool to link the Registration Info to. Changing this forces a new Registration Info resource to be created. Only a single virtual_desktop_host_pool_registration_info resource should be associated with a given hostpool. Assigning multiple resources will produce inconsistent results.
        #[builder(into)]
        pub hostpool_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct getHostPoolRegistrationInfoResult {
        /// A valid `RFC3339Time` for the expiration of the token..
        pub expiration_date: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Desktop Host Pool to link the Registration Info to. Changing this forces a new Registration Info resource to be created. Only a single virtual_desktop_host_pool_registration_info resource should be associated with a given hostpool. Assigning multiple resources will produce inconsistent results.
        pub hostpool_id: pulumi_wasm_rust::Output<String>,
        /// The registration token generated by the Virtual Desktop Host Pool for registration of session hosts.
        pub token: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: getHostPoolRegistrationInfoArgs,
    ) -> getHostPoolRegistrationInfoResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let expiration_date_binding = args.expiration_date.get_inner();
        let hostpool_id_binding = args.hostpool_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/getHostPoolRegistrationInfo:getHostPoolRegistrationInfo"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "expirationDate".into(),
                    value: &expiration_date_binding,
                },
                register_interface::ObjectField {
                    name: "hostpoolId".into(),
                    value: &hostpool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "expirationDate".into(),
                },
                register_interface::ResultField {
                    name: "hostpoolId".into(),
                },
                register_interface::ResultField {
                    name: "token".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        getHostPoolRegistrationInfoResult {
            expiration_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationDate").unwrap(),
            ),
            hostpool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostpoolId").unwrap(),
            ),
            token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("token").unwrap(),
            ),
        }
    }
}