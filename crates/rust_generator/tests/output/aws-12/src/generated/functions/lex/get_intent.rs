#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_intent {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIntentArgs {
        /// Name of the intent. The name is case sensitive.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the intent.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetIntentResult {
        /// ARN of the Lex intent.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Checksum identifying the version of the intent that was created. The checksum is not
        /// included as an argument because the resource will add it automatically when updating the intent.
        pub checksum: pulumi_gestalt_rust::Output<String>,
        /// Date when the intent version was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the intent.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date when the $LATEST version of this intent was updated.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Name of the intent, not case sensitive.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the built-in intent to base this
        /// intent on. To find the signature for an intent, see
        /// [Standard Built-in Intents](https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents)
        /// in the Alexa Skills Kit.
        pub parent_intent_signature: pulumi_gestalt_rust::Output<String>,
        /// Version of the bot.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetIntentArgs,
    ) -> GetIntentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lex/getIntent:getIntent".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetIntentResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            checksum: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("checksum"),
            ),
            created_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            last_updated_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedDate"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent_intent_signature: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentIntentSignature"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
