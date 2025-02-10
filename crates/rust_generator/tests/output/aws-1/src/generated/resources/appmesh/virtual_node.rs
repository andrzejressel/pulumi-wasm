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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_node {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNodeArgs {
        /// Name of the service mesh in which to create the virtual node. Must be between 1 and 255 characters in length.
        #[builder(into)]
        pub mesh_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        #[builder(into, default)]
        pub mesh_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name to use for the virtual node. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Virtual node specification to apply.
        #[builder(into)]
        pub spec: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appmesh::VirtualNodeSpec,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualNodeResult {
        /// ARN of the virtual node.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the virtual node.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Last update date of the virtual node.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Name of the service mesh in which to create the virtual node. Must be between 1 and 255 characters in length.
        pub mesh_name: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        pub mesh_owner: pulumi_gestalt_rust::Output<String>,
        /// Name to use for the virtual node. Must be between 1 and 255 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// Virtual node specification to apply.
        pub spec: pulumi_gestalt_rust::Output<
            super::super::types::appmesh::VirtualNodeSpec,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualNodeArgs,
    ) -> VirtualNodeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mesh_name_binding = args.mesh_name.get_output(context);
        let mesh_owner_binding = args.mesh_owner.get_output(context);
        let name_binding = args.name.get_output(context);
        let spec_binding = args.spec.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appmesh/virtualNode:VirtualNode".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "meshName".into(),
                    value: mesh_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "meshOwner".into(),
                    value: mesh_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spec".into(),
                    value: spec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualNodeResult {
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            mesh_name: o.get_field("meshName"),
            mesh_owner: o.get_field("meshOwner"),
            name: o.get_field("name"),
            resource_owner: o.get_field("resourceOwner"),
            spec: o.get_field("spec"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
