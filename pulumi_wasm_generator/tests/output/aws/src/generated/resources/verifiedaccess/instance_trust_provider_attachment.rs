/// Resource for managing a Verified Access Instance Trust Provider Attachment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance::create("example", InstanceArgs::builder().build_struct());
///     let exampleInstanceTrustProviderAttachment = instance_trust_provider_attachment::create(
///         "exampleInstanceTrustProviderAttachment",
///         InstanceTrustProviderAttachmentArgs::builder()
///             .verifiedaccess_instance_id("${example.id}")
///             .verifiedaccess_trust_provider_id("${exampleTrustProvider.id}")
///             .build_struct(),
///     );
///     let exampleTrustProvider = trust_provider::create(
///         "exampleTrustProvider",
///         TrustProviderArgs::builder()
///             .device_options(
///                 TrustProviderDeviceOptions::builder().tenantId("example").build_struct(),
///             )
///             .device_trust_provider_type("jamf")
///             .policy_reference_name("example")
///             .trust_provider_type("device")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Access Instance Trust Provider Attachments using the `verifiedaccess_instance_id` and `verifiedaccess_trust_provider_id` separated by a forward slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:verifiedaccess/instanceTrustProviderAttachment:InstanceTrustProviderAttachment example vai-1234567890abcdef0/vatp-8012925589
/// ```
pub mod instance_trust_provider_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceTrustProviderAttachmentArgs {
        /// The ID of the Verified Access instance to attach the Trust Provider to.
        #[builder(into)]
        pub verifiedaccess_instance_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Verified Access trust provider.
        #[builder(into)]
        pub verifiedaccess_trust_provider_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceTrustProviderAttachmentResult {
        /// The ID of the Verified Access instance to attach the Trust Provider to.
        pub verifiedaccess_instance_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Verified Access trust provider.
        pub verifiedaccess_trust_provider_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InstanceTrustProviderAttachmentArgs,
    ) -> InstanceTrustProviderAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let verifiedaccess_instance_id_binding = args
            .verifiedaccess_instance_id
            .get_inner();
        let verifiedaccess_trust_provider_id_binding = args
            .verifiedaccess_trust_provider_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedaccess/instanceTrustProviderAttachment:InstanceTrustProviderAttachment"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "verifiedaccessInstanceId".into(),
                    value: &verifiedaccess_instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "verifiedaccessTrustProviderId".into(),
                    value: &verifiedaccess_trust_provider_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "verifiedaccessInstanceId".into(),
                },
                register_interface::ResultField {
                    name: "verifiedaccessTrustProviderId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceTrustProviderAttachmentResult {
            verifiedaccess_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verifiedaccessInstanceId").unwrap(),
            ),
            verifiedaccess_trust_provider_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verifiedaccessTrustProviderId").unwrap(),
            ),
        }
    }
}
