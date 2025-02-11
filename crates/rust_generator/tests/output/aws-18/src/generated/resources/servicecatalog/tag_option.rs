/// Manages a Service Catalog Tag Option.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tag_option::create(
///         "example",
///         TagOptionArgs::builder().key("nyckel").value("värde").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_tag_option` using the tag option ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/tagOption:TagOption example tag-pjtvagohlyo3m
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag_option {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagOptionArgs {
        /// Whether tag option is active. Default is `true`.
        #[builder(into, default)]
        pub active: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Tag option key.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tag option value.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagOptionResult {
        /// Whether tag option is active. Default is `true`.
        pub active: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Tag option key.
        pub key: pulumi_gestalt_rust::Output<String>,
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Tag option value.
        ///
        /// The following arguments are optional:
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TagOptionArgs,
    ) -> TagOptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let active_binding = args.active.get_output(context);
        let key_binding = args.key.get_output(context);
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicecatalog/tagOption:TagOption".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "active".into(),
                    value: &active_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: &key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: &value_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TagOptionResult {
            active: o.get_field("active"),
            key: o.get_field("key"),
            owner: o.get_field("owner"),
            value: o.get_field("value"),
        }
    }
}
