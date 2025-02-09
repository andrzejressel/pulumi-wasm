/// Provides a Service Discovery Instance resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///       enableDnsSupport: true
///       enableDnsHostnames: true
///   examplePrivateDnsNamespace:
///     type: aws:servicediscovery:PrivateDnsNamespace
///     name: example
///     properties:
///       name: example.domain.local
///       description: example
///       vpc: ${example.id}
///   exampleService:
///     type: aws:servicediscovery:Service
///     name: example
///     properties:
///       name: example
///       dnsConfig:
///         namespaceId: ${examplePrivateDnsNamespace.id}
///         dnsRecords:
///           - ttl: 10
///             type: A
///         routingPolicy: MULTIVALUE
///       healthCheckCustomConfig:
///         failureThreshold: 1
///   exampleInstance:
///     type: aws:servicediscovery:Instance
///     name: example
///     properties:
///       instanceId: example-instance-id
///       serviceId: ${exampleService.id}
///       attributes:
///         AWS_INSTANCE_IPV4: 172.18.0.1
///         custom_attribute: custom
/// ```
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicediscovery:HttpNamespace
///     properties:
///       name: example.domain.test
///       description: example
///   exampleService:
///     type: aws:servicediscovery:Service
///     name: example
///     properties:
///       name: example
///       namespaceId: ${example.id}
///   exampleInstance:
///     type: aws:servicediscovery:Instance
///     name: example
///     properties:
///       instanceId: example-instance-id
///       serviceId: ${exampleService.id}
///       attributes:
///         AWS_EC2_INSTANCE_ID: i-0abdg374kd892cj6dl
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Service Discovery Instance using the service ID and instance ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicediscovery/instance:Instance example 0123456789/i-0123
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// A map contains the attributes of the instance. Check the [doc](https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html#API_RegisterInstance_RequestSyntax) for the supported attributes and syntax.
        #[builder(into)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the service instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the service that you want to use to create the instance.
        #[builder(into)]
        pub service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// A map contains the attributes of the instance. Check the [doc](https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html#API_RegisterInstance_RequestSyntax) for the supported attributes and syntax.
        pub attributes: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the service instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the service that you want to use to create the instance.
        pub service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let attributes_binding_1 = args.attributes.get_output(context);
        let attributes_binding = attributes_binding_1.get_inner();
        let instance_id_binding_1 = args.instance_id.get_output(context);
        let instance_id_binding = instance_id_binding_1.get_inner();
        let service_id_binding_1 = args.service_id.get_output(context);
        let service_id_binding = service_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicediscovery/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceResult {
            attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
        }
    }
}
