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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetProtectionArgs,
    ) -> GetProtectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let protection_id_binding = args.protection_id.get_output(context).get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:shield/getProtection:getProtection".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "protectionId".into(),
                    value: &protection_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetProtectionResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            protection_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protectionArn"),
            ),
            protection_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protectionId"),
            ),
            resource_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
        }
    }
}
