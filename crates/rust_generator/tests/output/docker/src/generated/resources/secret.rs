///
///
/// ## Import
///
/// ```sh
/// # Docker secret cannot be imported as the secret data, once set, is never exposed again.
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretArgs {
        /// Base64-url-safe-encoded secret data
        #[builder(into)]
        pub data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::SecretLabel>>,
        >,
        /// User-defined name of the secret
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecretResult {
        /// Base64-url-safe-encoded secret data
        pub data: pulumi_gestalt_rust::Output<String>,
        /// User-defined key/value metadata
        pub labels: pulumi_gestalt_rust::Output<Option<Vec<super::types::SecretLabel>>>,
        /// User-defined name of the secret
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecretArgs,
    ) -> SecretResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_binding = args.data.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/secret:Secret".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "data".into(),
                    value: data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecretResult {
            data: o.get_field("data"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
        }
    }
}
