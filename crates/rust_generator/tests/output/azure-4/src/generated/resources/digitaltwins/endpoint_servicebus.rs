/// Manages a Digital Twins Service Bus Endpoint.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example_resources")
///             .build_struct(),
///     );
///     let exampleEndpointServicebus = endpoint_servicebus::create(
///         "exampleEndpointServicebus",
///         EndpointServicebusArgs::builder()
///             .digital_twins_id("${exampleInstance.id}")
///             .name("example-EndpointSB")
///             .servicebus_primary_connection_string(
///                 "${exampleTopicAuthorizationRule.primaryConnectionString}",
///             )
///             .servicebus_secondary_connection_string(
///                 "${exampleTopicAuthorizationRule.secondaryConnectionString}",
///             )
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
///     let exampleNamespace = namespace::create(
///         "exampleNamespace",
///         NamespaceArgs::builder()
///             .location("${example.location}")
///             .name("exampleservicebusnamespace")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleTopic = topic::create(
///         "exampleTopic",
///         TopicArgs::builder()
///             .name("exampleservicebustopic")
///             .namespace_id("${exampleNamespace.id}")
///             .build_struct(),
///     );
///     let exampleTopicAuthorizationRule = topic_authorization_rule::create(
///         "exampleTopicAuthorizationRule",
///         TopicAuthorizationRuleArgs::builder()
///             .listen(false)
///             .manage(false)
///             .name("example-rule")
///             .send(true)
///             .topic_id("${exampleTopic.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Digital Twins Service Bus Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:digitaltwins/endpointServicebus:EndpointServicebus example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DigitalTwins/digitalTwinsInstances/dt1/endpoints/ep1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_servicebus {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointServicebusArgs {
        /// The storage secret of the dead-lettering, whose format is `https://<storageAccountname>.blob.core.windows.net/<containerName>?<SASToken>`. When an endpoint can't deliver an event within a certain time period or after trying to deliver the event a certain number of times, it can send the undelivered event to a storage account.
        #[builder(into, default)]
        pub dead_letter_storage_secret: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Digital Twins Instance. Changing this forces a new Digital Twins Service Bus Endpoint to be created.
        #[builder(into)]
        pub digital_twins_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Digital Twins Service Bus Endpoint. Changing this forces a new Digital Twins Service Bus Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The primary connection string of the Service Bus Topic Authorization Rule with a minimum of `send` permission. .
        #[builder(into)]
        pub servicebus_primary_connection_string: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The secondary connection string of the Service Bus Topic Authorization Rule with a minimum of `send` permission.
        #[builder(into)]
        pub servicebus_secondary_connection_string: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct EndpointServicebusResult {
        /// The storage secret of the dead-lettering, whose format is `https://<storageAccountname>.blob.core.windows.net/<containerName>?<SASToken>`. When an endpoint can't deliver an event within a certain time period or after trying to deliver the event a certain number of times, it can send the undelivered event to a storage account.
        pub dead_letter_storage_secret: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Digital Twins Instance. Changing this forces a new Digital Twins Service Bus Endpoint to be created.
        pub digital_twins_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Digital Twins Service Bus Endpoint. Changing this forces a new Digital Twins Service Bus Endpoint to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the Service Bus Topic Authorization Rule with a minimum of `send` permission. .
        pub servicebus_primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the Service Bus Topic Authorization Rule with a minimum of `send` permission.
        pub servicebus_secondary_connection_string: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EndpointServicebusArgs,
    ) -> EndpointServicebusResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dead_letter_storage_secret_binding_1 = args
            .dead_letter_storage_secret
            .get_output(context);
        let dead_letter_storage_secret_binding = dead_letter_storage_secret_binding_1
            .get_inner();
        let digital_twins_id_binding_1 = args.digital_twins_id.get_output(context);
        let digital_twins_id_binding = digital_twins_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let servicebus_primary_connection_string_binding_1 = args
            .servicebus_primary_connection_string
            .get_output(context);
        let servicebus_primary_connection_string_binding = servicebus_primary_connection_string_binding_1
            .get_inner();
        let servicebus_secondary_connection_string_binding_1 = args
            .servicebus_secondary_connection_string
            .get_output(context);
        let servicebus_secondary_connection_string_binding = servicebus_secondary_connection_string_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:digitaltwins/endpointServicebus:EndpointServicebus".into(),
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "servicebusPrimaryConnectionString".into(),
                    value: &servicebus_primary_connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "servicebusSecondaryConnectionString".into(),
                    value: &servicebus_secondary_connection_string_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointServicebusResult {
            dead_letter_storage_secret: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deadLetterStorageSecret"),
            ),
            digital_twins_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("digitalTwinsId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            servicebus_primary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("servicebusPrimaryConnectionString"),
            ),
            servicebus_secondary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("servicebusSecondaryConnectionString"),
            ),
        }
    }
}
