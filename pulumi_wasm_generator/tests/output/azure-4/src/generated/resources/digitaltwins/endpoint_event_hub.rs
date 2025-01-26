/// Manages a Digital Twins Event Hub Endpoint.
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
///             .name("example_resources")
///             .build_struct(),
///     );
///     let exampleAuthorizationRule = authorization_rule::create(
///         "exampleAuthorizationRule",
///         AuthorizationRuleArgs::builder()
///             .eventhub_name("${exampleEventHub.name}")
///             .listen(false)
///             .manage(false)
///             .name("example-ar")
///             .namespace_name("${exampleEventHubNamespace.name}")
///             .resource_group_name("${example.name}")
///             .send(true)
///             .build_struct(),
///     );
///     let exampleEndpointEventHub = endpoint_event_hub::create(
///         "exampleEndpointEventHub",
///         EndpointEventHubArgs::builder()
///             .digital_twins_id("${exampleInstance.id}")
///             .eventhub_primary_connection_string(
///                 "${exampleAuthorizationRule.primaryConnectionString}",
///             )
///             .eventhub_secondary_connection_string(
///                 "${exampleAuthorizationRule.secondaryConnectionString}",
///             )
///             .name("example-EH")
///             .build_struct(),
///     );
///     let exampleEventHub = event_hub::create(
///         "exampleEventHub",
///         EventHubArgs::builder()
///             .message_retention(1)
///             .name("example-eh")
///             .namespace_name("${exampleEventHubNamespace.name}")
///             .partition_count(2)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleEventHubNamespace = event_hub_namespace::create(
///         "exampleEventHubNamespace",
///         EventHubNamespaceArgs::builder()
///             .location("${example.location}")
///             .name("example-eh-ns")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleInstance = instance::create(
///         "exampleInstance",
///         InstanceArgs::builder()
///             .location("${example.location}")
///             .name("example-DT")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Digital Twins Eventhub Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:digitaltwins/endpointEventHub:EndpointEventHub example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DigitalTwins/digitalTwinsInstances/dt1/endpoints/ep1
/// ```
///
pub mod endpoint_event_hub {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointEventHubArgs {
        /// The storage secret of the dead-lettering, whose format is `https://<storageAccountname>.blob.core.windows.net/<containerName>?<SASToken>`. When an endpoint can't deliver an event within a certain time period or after trying to deliver the event a certain number of times, it can send the undelivered event to a storage account.
        #[builder(into, default)]
        pub dead_letter_storage_secret: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the Digital Twins Instance. Changing this forces a new Digital Twins Event Hub Endpoint to be created.
        #[builder(into)]
        pub digital_twins_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The primary connection string of the Event Hub Authorization Rule with a minimum of `send` permission.
        #[builder(into)]
        pub eventhub_primary_connection_string: pulumi_wasm_rust::InputOrOutput<String>,
        /// The secondary connection string of the Event Hub Authorization Rule with a minimum of `send` permission.
        #[builder(into)]
        pub eventhub_secondary_connection_string: pulumi_wasm_rust::InputOrOutput<
            String,
        >,
        /// The name which should be used for this Digital Twins Event Hub Endpoint. Changing this forces a new Digital Twins Event Hub Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EndpointEventHubResult {
        /// The storage secret of the dead-lettering, whose format is `https://<storageAccountname>.blob.core.windows.net/<containerName>?<SASToken>`. When an endpoint can't deliver an event within a certain time period or after trying to deliver the event a certain number of times, it can send the undelivered event to a storage account.
        pub dead_letter_storage_secret: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the Digital Twins Instance. Changing this forces a new Digital Twins Event Hub Endpoint to be created.
        pub digital_twins_id: pulumi_wasm_rust::Output<String>,
        /// The primary connection string of the Event Hub Authorization Rule with a minimum of `send` permission.
        pub eventhub_primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string of the Event Hub Authorization Rule with a minimum of `send` permission.
        pub eventhub_secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Digital Twins Event Hub Endpoint. Changing this forces a new Digital Twins Event Hub Endpoint to be created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EndpointEventHubArgs,
    ) -> EndpointEventHubResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dead_letter_storage_secret_binding = args
            .dead_letter_storage_secret
            .get_output(context)
            .get_inner();
        let digital_twins_id_binding = args
            .digital_twins_id
            .get_output(context)
            .get_inner();
        let eventhub_primary_connection_string_binding = args
            .eventhub_primary_connection_string
            .get_output(context)
            .get_inner();
        let eventhub_secondary_connection_string_binding = args
            .eventhub_secondary_connection_string
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:digitaltwins/endpointEventHub:EndpointEventHub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deadLetterStorageSecret".into(),
                    value: &dead_letter_storage_secret_binding,
                },
                register_interface::ObjectField {
                    name: "digitalTwinsId".into(),
                    value: &digital_twins_id_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubPrimaryConnectionString".into(),
                    value: &eventhub_primary_connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubSecondaryConnectionString".into(),
                    value: &eventhub_secondary_connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointEventHubResult {
            dead_letter_storage_secret: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deadLetterStorageSecret"),
            ),
            digital_twins_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("digitalTwinsId"),
            ),
            eventhub_primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventhubPrimaryConnectionString"),
            ),
            eventhub_secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventhubSecondaryConnectionString"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
