/// Manages a Fluid Relay Server.
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Fluid Relay Servers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:fluidrelay/server:Server example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.FluidRelay/fluidRelayServers/server1
/// ```
///
pub mod server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerArgs {
        /// A `customer_managed_key` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::fluidrelay::ServerCustomerManagedKey>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::fluidrelay::ServerIdentity>,
        >,
        /// The Azure Region where the Fluid Relay Server should exist. Changing this forces a new Fluid Relay Server to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Fluid Relay Server. Changing this forces a new Fluid Relay Server to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Fluid Relay Server should exist. Changing this forces a new Fluid Relay Server to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Sku of the storage associated with the resource, Possible values are `standard` and `basic`. Changing this forces a new Fluid Relay Server to be created.
        #[builder(into, default)]
        pub storage_sku: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Fluid Relay Server.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServerResult {
        /// A `customer_managed_key` block as defined below. Changing this forces a new resource to be created.
        pub customer_managed_key: pulumi_wasm_rust::Output<
            Option<super::super::types::fluidrelay::ServerCustomerManagedKey>,
        >,
        /// The Fluid tenantId for this server.
        pub frs_tenant_id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::fluidrelay::ServerIdentity>,
        >,
        /// The Azure Region where the Fluid Relay Server should exist. Changing this forces a new Fluid Relay Server to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Fluid Relay Server. Changing this forces a new Fluid Relay Server to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An array of the Fluid Relay Orderer endpoints. This will be deprecated in future version of fluid relay server and will always be empty, [more details](https://learn.microsoft.com/en-us/azure/azure-fluid-relay/concepts/version-compatibility).
        pub orderer_endpoints: pulumi_wasm_rust::Output<Vec<String>>,
        /// The primary key for this server.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Fluid Relay Server should exist. Changing this forces a new Fluid Relay Server to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary key for this server.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        /// An array of service endpoints for this Fluid Relay Server.
        pub service_endpoints: pulumi_wasm_rust::Output<Vec<String>>,
        /// An array of storage endpoints for this Fluid Relay Server. This will be deprecated in future version of fluid relay server and will always be empty, [more details](https://learn.microsoft.com/en-us/azure/azure-fluid-relay/concepts/version-compatibility).
        pub storage_endpoints: pulumi_wasm_rust::Output<Vec<String>>,
        /// Sku of the storage associated with the resource, Possible values are `standard` and `basic`. Changing this forces a new Fluid Relay Server to be created.
        pub storage_sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Fluid Relay Server.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServerArgs,
    ) -> ServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let customer_managed_key_binding = args
            .customer_managed_key
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let storage_sku_binding = args.storage_sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:fluidrelay/server:Server".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customerManagedKey".into(),
                    value: &customer_managed_key_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageSku".into(),
                    value: &storage_sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServerResult {
            customer_managed_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerManagedKey"),
            ),
            frs_tenant_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("frsTenantId"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            orderer_endpoints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ordererEndpoints"),
            ),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryKey"),
            ),
            service_endpoints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceEndpoints"),
            ),
            storage_endpoints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageEndpoints"),
            ),
            storage_sku: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageSku"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
