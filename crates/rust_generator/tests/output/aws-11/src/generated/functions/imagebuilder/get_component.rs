#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_component {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetComponentArgs {
        /// ARN of the component.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the component.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetComponentResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Change description of the component.
        pub change_description: pulumi_gestalt_rust::Output<String>,
        /// Data of the component.
        pub data: pulumi_gestalt_rust::Output<String>,
        /// Date the component was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Description of the component.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Encryption status of the component.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Key Management Service (KMS) Key used to encrypt the component.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the component.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Owner of the component.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Platform of the component.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Operating Systems (OSes) supported by the component.
        pub supported_os_versions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value map of resource tags for the component.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of the component.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Version of the component.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetComponentArgs,
    ) -> GetComponentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:imagebuilder/getComponent:getComponent".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetComponentResult {
            arn: o.get_field("arn"),
            change_description: o.get_field("changeDescription"),
            data: o.get_field("data"),
            date_created: o.get_field("dateCreated"),
            description: o.get_field("description"),
            encrypted: o.get_field("encrypted"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            name: o.get_field("name"),
            owner: o.get_field("owner"),
            platform: o.get_field("platform"),
            supported_os_versions: o.get_field("supportedOsVersions"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            version: o.get_field("version"),
        }
    }
}
