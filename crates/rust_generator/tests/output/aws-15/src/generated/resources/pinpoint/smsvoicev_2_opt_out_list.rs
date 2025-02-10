/// Manages an AWS End User Messaging SMS opt-out list.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = smsvoicev_2_opt_out_list::create(
///         "example",
///         Smsvoicev2OptOutListArgs::builder().name("example-opt-out-list").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import opt-out lists using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/smsvoicev2OptOutList:Smsvoicev2OptOutList example example-opt-out-list
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod smsvoicev_2_opt_out_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Smsvoicev2OptOutListArgs {
        /// The name of the opt-out list.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct Smsvoicev2OptOutListResult {
        /// ARN of the opt-out list.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the opt-out list.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: Smsvoicev2OptOutListArgs,
    ) -> Smsvoicev2OptOutListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/smsvoicev2OptOutList:Smsvoicev2OptOutList".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        Smsvoicev2OptOutListResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
