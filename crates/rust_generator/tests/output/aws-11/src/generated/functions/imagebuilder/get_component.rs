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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetComponentArgs,
    ) -> GetComponentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getComponent:getComponent".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetComponentResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            change_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("changeDescription"),
            ),
            data: pulumi_gestalt_rust::__private::into_domain(o.extract_field("data")),
            date_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            supported_os_versions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedOsVersions"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
