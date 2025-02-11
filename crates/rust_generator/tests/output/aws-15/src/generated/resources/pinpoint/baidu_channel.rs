/// Provides a Pinpoint Baidu Channel resource.
///
/// > **Note:** All arguments including the Api Key and Secret Key will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let app = app::create("app", AppArgs::builder().build_struct());
///     let channel = baidu_channel::create(
///         "channel",
///         BaiduChannelArgs::builder()
///             .api_key("")
///             .application_id("${app.applicationId}")
///             .secret_key("")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Pinpoint Baidu Channel using the `application-id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/baiduChannel:BaiduChannel channel application-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod baidu_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BaiduChannelArgs {
        /// Platform credential API key from Baidu.
        #[builder(into)]
        pub api_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether to enable the channel. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Platform credential Secret key from Baidu.
        #[builder(into)]
        pub secret_key: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BaiduChannelResult {
        /// Platform credential API key from Baidu.
        pub api_key: pulumi_gestalt_rust::Output<String>,
        /// The application ID.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether to enable the channel. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Platform credential Secret key from Baidu.
        pub secret_key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BaiduChannelArgs,
    ) -> BaiduChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_key_binding = args.api_key.get_output(context);
        let application_id_binding = args.application_id.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let secret_key_binding = args.secret_key.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/baiduChannel:BaiduChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiKey".into(),
                    value: &api_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretKey".into(),
                    value: &secret_key_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BaiduChannelResult {
            api_key: o.get_field("apiKey"),
            application_id: o.get_field("applicationId"),
            enabled: o.get_field("enabled"),
            secret_key: o.get_field("secretKey"),
        }
    }
}
