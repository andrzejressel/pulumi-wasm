/// Manages an AWS IoT Billing Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iot:BillingGroup
///     properties:
///       name: example
///       properties:
///         description: This is my billing group
///       tags:
///         terraform: 'true'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IoT Billing Groups using the name. For example:
///
/// ```sh
/// $ pulumi import aws:iot/billingGroup:BillingGroup example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod billing_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BillingGroupArgs {
        /// The name of the Billing Group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Billing Group properties. Defined below.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iot::BillingGroupProperties>,
        >,
        /// Key-value mapping of resource tags
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BillingGroupResult {
        /// The ARN of the Billing Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub metadatas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::iot::BillingGroupMetadata>,
        >,
        /// The name of the Billing Group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Billing Group properties. Defined below.
        pub properties: pulumi_gestalt_rust::Output<
            Option<super::super::types::iot::BillingGroupProperties>,
        >,
        /// Key-value mapping of resource tags
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current version of the Billing Group record in the registry.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BillingGroupArgs,
    ) -> BillingGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let properties_binding = args.properties.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/billingGroup:BillingGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "properties".into(),
                    value: &properties_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BillingGroupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            metadatas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadatas"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("properties"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
