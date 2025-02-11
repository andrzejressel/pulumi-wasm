/// Resource for managing a Verified Access Instance Trust Provider Attachment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_trust_provider_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceTrustProviderAttachmentArgs {
        /// The ID of the Verified Access instance to attach the Trust Provider to.
        #[builder(into)]
        pub verifiedaccess_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Verified Access trust provider.
        #[builder(into)]
        pub verifiedaccess_trust_provider_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceTrustProviderAttachmentResult {
        /// The ID of the Verified Access instance to attach the Trust Provider to.
        pub verifiedaccess_instance_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Verified Access trust provider.
        pub verifiedaccess_trust_provider_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceTrustProviderAttachmentArgs,
    ) -> InstanceTrustProviderAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let verifiedaccess_instance_id_binding = args
            .verifiedaccess_instance_id
            .get_output(context);
        let verifiedaccess_trust_provider_id_binding = args
            .verifiedaccess_trust_provider_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:verifiedaccess/instanceTrustProviderAttachment:InstanceTrustProviderAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "verifiedaccessInstanceId".into(),
                    value: &verifiedaccess_instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "verifiedaccessTrustProviderId".into(),
                    value: &verifiedaccess_trust_provider_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceTrustProviderAttachmentResult {
            verifiedaccess_instance_id: o.get_field("verifiedaccessInstanceId"),
            verifiedaccess_trust_provider_id: o
                .get_field("verifiedaccessTrustProviderId"),
        }
    }
}
