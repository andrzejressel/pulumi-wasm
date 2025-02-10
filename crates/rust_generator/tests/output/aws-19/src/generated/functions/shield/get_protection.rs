#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_protection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProtectionArgs {
        /// Unique identifier for the protection.
        #[builder(into, default)]
        pub protection_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN (Amazon Resource Name) of the resource being protected.
        #[builder(into, default)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetProtectionResult {
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the protection.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the protection.
        pub protection_arn: pulumi_gestalt_rust::Output<String>,
        pub protection_id: pulumi_gestalt_rust::Output<String>,
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetProtectionArgs,
    ) -> GetProtectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let protection_id_binding = args.protection_id.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:shield/getProtection:getProtection".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protectionId".into(),
                    value: protection_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: resource_arn_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetProtectionResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            protection_arn: o.get_field("protectionArn"),
            protection_id: o.get_field("protectionId"),
            resource_arn: o.get_field("resourceArn"),
        }
    }
}
