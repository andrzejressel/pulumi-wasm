/// Provides an AWS App Mesh virtual gateway resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appmesh:VirtualGateway
///     properties:
///       name: example-virtual-gateway
///       meshName: example-service-mesh
///       spec:
///         listeners:
///           - portMapping:
///               port: 8080
///               protocol: http
///       tags:
///         Environment: test
/// ```
///
/// ### Access Logs and TLS
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = virtual_gateway::create(
///         "example",
///         VirtualGatewayArgs::builder()
///             .mesh_name("example-service-mesh")
///             .name("example-virtual-gateway")
///             .spec(
///                 VirtualGatewaySpec::builder()
///                     .listeners(
///                         vec![
///                             VirtualGatewaySpecListener::builder()
///                             .portMapping(VirtualGatewaySpecListenerPortMapping::builder()
///                             .port(8080).protocol("http").build_struct())
///                             .tls(VirtualGatewaySpecListenerTls::builder()
///                             .certificate(VirtualGatewaySpecListenerTlsCertificate::builder()
///                             .acm(VirtualGatewaySpecListenerTlsCertificateAcm::builder()
///                             .certificateArn("${exampleAwsAcmCertificate.arn}")
///                             .build_struct()).build_struct()).mode("STRICT")
///                             .build_struct()).build_struct(),
///                         ],
///                     )
///                     .logging(
///                         VirtualGatewaySpecLogging::builder()
///                             .accessLog(
///                                 VirtualGatewaySpecLoggingAccessLog::builder()
///                                     .file(
///                                         VirtualGatewaySpecLoggingAccessLogFile::builder()
///                                             .path("/var/log/access.log")
///                                             .build_struct(),
///                                     )
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
/// Using `pulumi import`, import App Mesh virtual gateway using `mesh_name` together with the virtual gateway's `name`. For example:
///
/// ```sh
/// $ pulumi import aws:appmesh/virtualGateway:VirtualGateway example mesh/gw1
/// ```
pub mod virtual_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualGatewayArgs {
        /// Name of the service mesh in which to create the virtual gateway. Must be between 1 and 255 characters in length.
        #[builder(into)]
        pub mesh_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        #[builder(into, default)]
        pub mesh_owner: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name to use for the virtual gateway. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Virtual gateway specification to apply.
        #[builder(into)]
        pub spec: pulumi_wasm_rust::InputOrOutput<
            super::super::types::appmesh::VirtualGatewaySpec,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualGatewayResult {
        /// ARN of the virtual gateway.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Creation date of the virtual gateway.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Last update date of the virtual gateway.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Name of the service mesh in which to create the virtual gateway. Must be between 1 and 255 characters in length.
        pub mesh_name: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        pub mesh_owner: pulumi_wasm_rust::Output<String>,
        /// Name to use for the virtual gateway. Must be between 1 and 255 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_wasm_rust::Output<String>,
        /// Virtual gateway specification to apply.
        pub spec: pulumi_wasm_rust::Output<
            super::super::types::appmesh::VirtualGatewaySpec,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VirtualGatewayArgs,
    ) -> VirtualGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mesh_name_binding = args.mesh_name.get_output(context).get_inner();
        let mesh_owner_binding = args.mesh_owner.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let spec_binding = args.spec.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appmesh/virtualGateway:VirtualGateway".into(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        VirtualGatewayResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedDate"),
            ),
            mesh_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("meshName"),
            ),
            mesh_owner: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("meshOwner"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_owner: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceOwner"),
            ),
            spec: pulumi_wasm_rust::__private::into_domain(o.extract_field("spec")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
