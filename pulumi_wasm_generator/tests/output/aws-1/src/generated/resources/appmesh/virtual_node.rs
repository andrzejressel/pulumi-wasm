/// Provides an AWS App Mesh virtual node resource.
///
/// ## Breaking Changes
///
/// Because of backward incompatible API changes (read [here](https://github.com/awslabs/aws-app-mesh-examples/issues/92)), `aws.appmesh.VirtualNode` resource definitions created with provider versions earlier than v2.3.0 will need to be modified:
///
/// * Rename the `service_name` attribute of the `dns` object to `hostname`.
///
/// * Replace the `backends` attribute of the `spec` object with one or more `backend` configuration blocks,
/// setting `virtual_service_name` to the name of the service.
///
/// The state associated with existing resources will automatically be migrated.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serviceb1 = virtual_node::create(
///         "serviceb1",
///         VirtualNodeArgs::builder()
///             .mesh_name("${simple.id}")
///             .name("serviceBv1")
///             .spec(
///                 VirtualNodeSpec::builder()
///                     .backends(
///                         vec![
///                             VirtualNodeSpecBackend::builder()
///                             .virtualService(VirtualNodeSpecBackendVirtualService::builder()
///                             .virtualServiceName("servicea.simpleapp.local")
///                             .build_struct()).build_struct(),
///                         ],
///                     )
///                     .listeners(
///                         vec![
///                             VirtualNodeSpecListener::builder()
///                             .portMapping(VirtualNodeSpecListenerPortMapping::builder()
///                             .port(8080).protocol("http").build_struct()).build_struct(),
///                         ],
///                     )
///                     .serviceDiscovery(
///                         VirtualNodeSpecServiceDiscovery::builder()
///                             .dns(
///                                 VirtualNodeSpecServiceDiscoveryDns::builder()
///                                     .hostname("serviceb.simpleapp.local")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### AWS Cloud Map Service Discovery
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicediscovery:HttpNamespace
///     properties:
///       name: example-ns
///   serviceb1:
///     type: aws:appmesh:VirtualNode
///     properties:
///       name: serviceBv1
///       meshName: ${simple.id}
///       spec:
///         backends:
///           - virtualService:
///               virtualServiceName: servicea.simpleapp.local
///         listeners:
///           - portMapping:
///               port: 8080
///               protocol: http
///         serviceDiscovery:
///           awsCloudMap:
///             attributes:
///               stack: blue
///             serviceName: serviceb1
///             namespaceName: ${example.name}
/// ```
///
/// ### Listener Health Check
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serviceb1 = virtual_node::create(
///         "serviceb1",
///         VirtualNodeArgs::builder()
///             .mesh_name("${simple.id}")
///             .name("serviceBv1")
///             .spec(
///                 VirtualNodeSpec::builder()
///                     .backends(
///                         vec![
///                             VirtualNodeSpecBackend::builder()
///                             .virtualService(VirtualNodeSpecBackendVirtualService::builder()
///                             .virtualServiceName("servicea.simpleapp.local")
///                             .build_struct()).build_struct(),
///                         ],
///                     )
///                     .listeners(
///                         vec![
///                             VirtualNodeSpecListener::builder()
///                             .healthCheck(VirtualNodeSpecListenerHealthCheck::builder()
///                             .healthyThreshold(2).intervalMillis(5000).path("/ping")
///                             .protocol("http").timeoutMillis(2000).unhealthyThreshold(2)
///                             .build_struct())
///                             .portMapping(VirtualNodeSpecListenerPortMapping::builder()
///                             .port(8080).protocol("http").build_struct()).build_struct(),
///                         ],
///                     )
///                     .serviceDiscovery(
///                         VirtualNodeSpecServiceDiscovery::builder()
///                             .dns(
///                                 VirtualNodeSpecServiceDiscoveryDns::builder()
///                                     .hostname("serviceb.simpleapp.local")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Logging
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serviceb1 = virtual_node::create(
///         "serviceb1",
///         VirtualNodeArgs::builder()
///             .mesh_name("${simple.id}")
///             .name("serviceBv1")
///             .spec(
///                 VirtualNodeSpec::builder()
///                     .backends(
///                         vec![
///                             VirtualNodeSpecBackend::builder()
///                             .virtualService(VirtualNodeSpecBackendVirtualService::builder()
///                             .virtualServiceName("servicea.simpleapp.local")
///                             .build_struct()).build_struct(),
///                         ],
///                     )
///                     .listeners(
///                         vec![
///                             VirtualNodeSpecListener::builder()
///                             .portMapping(VirtualNodeSpecListenerPortMapping::builder()
///                             .port(8080).protocol("http").build_struct()).build_struct(),
///                         ],
///                     )
///                     .logging(
///                         VirtualNodeSpecLogging::builder()
///                             .accessLog(
///                                 VirtualNodeSpecLoggingAccessLog::builder()
///                                     .file(
///                                         VirtualNodeSpecLoggingAccessLogFile::builder()
///                                             .path("/dev/stdout")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .serviceDiscovery(
///                         VirtualNodeSpecServiceDiscovery::builder()
///                             .dns(
///                                 VirtualNodeSpecServiceDiscoveryDns::builder()
///                                     .hostname("serviceb.simpleapp.local")
///                                     .build_struct(),
///                             )
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
/// Using `pulumi import`, import App Mesh virtual nodes using `mesh_name` together with the virtual node's `name`. For example:
///
/// ```sh
/// $ pulumi import aws:appmesh/virtualNode:VirtualNode serviceb1 simpleapp/serviceBv1
/// ```
pub mod virtual_node {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNodeArgs {
        /// Name of the service mesh in which to create the virtual node. Must be between 1 and 255 characters in length.
        #[builder(into)]
        pub mesh_name: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        #[builder(into, default)]
        pub mesh_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Name to use for the virtual node. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Virtual node specification to apply.
        #[builder(into)]
        pub spec: pulumi_wasm_rust::Output<
            super::super::types::appmesh::VirtualNodeSpec,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualNodeResult {
        /// ARN of the virtual node.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Creation date of the virtual node.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Last update date of the virtual node.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Name of the service mesh in which to create the virtual node. Must be between 1 and 255 characters in length.
        pub mesh_name: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        pub mesh_owner: pulumi_wasm_rust::Output<String>,
        /// Name to use for the virtual node. Must be between 1 and 255 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_wasm_rust::Output<String>,
        /// Virtual node specification to apply.
        pub spec: pulumi_wasm_rust::Output<
            super::super::types::appmesh::VirtualNodeSpec,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VirtualNodeArgs) -> VirtualNodeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mesh_name_binding = args.mesh_name.get_inner();
        let mesh_owner_binding = args.mesh_owner.get_inner();
        let name_binding = args.name.get_inner();
        let spec_binding = args.spec.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appmesh/virtualNode:VirtualNode".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "meshName".into(),
                    value: &mesh_name_binding,
                },
                register_interface::ObjectField {
                    name: "meshOwner".into(),
                    value: &mesh_owner_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "spec".into(),
                    value: &spec_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedDate".into(),
                },
                register_interface::ResultField {
                    name: "meshName".into(),
                },
                register_interface::ResultField {
                    name: "meshOwner".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceOwner".into(),
                },
                register_interface::ResultField {
                    name: "spec".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualNodeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedDate").unwrap(),
            ),
            mesh_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("meshName").unwrap(),
            ),
            mesh_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("meshOwner").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceOwner").unwrap(),
            ),
            spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spec").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
