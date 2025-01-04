/// Manages an EventGrid Domain Topic
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleDomain:
///     type: azure:eventgrid:Domain
///     name: example
///     properties:
///       name: my-eventgrid-domain
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         environment: Production
///   exampleDomainTopic:
///     type: azure:eventgrid:DomainTopic
///     name: example
///     properties:
///       name: my-eventgrid-domain-topic
///       domainName: ${exampleDomain.name}
///       resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// EventGrid Domain Topics can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventgrid/domainTopic:DomainTopic topic1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/domains/domain1/topics/topic1
/// ```
///
pub mod domain_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainTopicArgs {
        /// Specifies the name of the EventGrid Domain. Changing this forces a new resource to be created.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the EventGrid Domain Topic resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainTopicResult {
        /// Specifies the name of the EventGrid Domain. Changing this forces a new resource to be created.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the EventGrid Domain Topic resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainTopicArgs) -> DomainTopicResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventgrid/domainTopic:DomainTopic".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainTopicResult {
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
