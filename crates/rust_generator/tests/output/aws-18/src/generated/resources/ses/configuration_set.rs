/// Provides an SES configuration set resource.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = configuration_set::create(
///         "test",
///         ConfigurationSetArgs::builder()
///             .name("some-configuration-set-test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Require TLS Connections
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = configuration_set::create(
///         "test",
///         ConfigurationSetArgs::builder()
///             .delivery_options(
///                 ConfigurationSetDeliveryOptions::builder()
///                     .tlsPolicy("Require")
///                     .build_struct(),
///             )
///             .name("some-configuration-set-test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Tracking Options
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = configuration_set::create(
///         "test",
///         ConfigurationSetArgs::builder()
///             .name("some-configuration-set-test")
///             .tracking_options(
///                 ConfigurationSetTrackingOptions::builder()
///                     .customRedirectDomain("sub.example.com")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES Configuration Sets using their `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ses/configurationSet:ConfigurationSet test some-configuration-set-test
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationSetArgs {
        /// Whether messages that use the configuration set are required to use TLS. See below.
        #[builder(into, default)]
        pub delivery_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ses::ConfigurationSetDeliveryOptions>,
        >,
        /// Name of the configuration set.
        ///
        /// The following argument is optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether or not Amazon SES publishes reputation metrics for the configuration set, such as bounce and complaint rates, to Amazon CloudWatch. The default value is `false`.
        #[builder(into, default)]
        pub reputation_metrics_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether email sending is enabled or disabled for the configuration set. The default value is `true`.
        #[builder(into, default)]
        pub sending_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Domain that is used to redirect email recipients to an Amazon SES-operated domain. See below. **NOTE:** This functionality is best effort.
        #[builder(into, default)]
        pub tracking_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ses::ConfigurationSetTrackingOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationSetResult {
        /// SES configuration set ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether messages that use the configuration set are required to use TLS. See below.
        pub delivery_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ses::ConfigurationSetDeliveryOptions>,
        >,
        /// Date and time at which the reputation metrics for the configuration set were last reset. Resetting these metrics is known as a fresh start.
        pub last_fresh_start: pulumi_gestalt_rust::Output<String>,
        /// Name of the configuration set.
        ///
        /// The following argument is optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether or not Amazon SES publishes reputation metrics for the configuration set, such as bounce and complaint rates, to Amazon CloudWatch. The default value is `false`.
        pub reputation_metrics_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether email sending is enabled or disabled for the configuration set. The default value is `true`.
        pub sending_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Domain that is used to redirect email recipients to an Amazon SES-operated domain. See below. **NOTE:** This functionality is best effort.
        pub tracking_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ses::ConfigurationSetTrackingOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationSetArgs,
    ) -> ConfigurationSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let delivery_options_binding = args.delivery_options.get_output(context);
        let name_binding = args.name.get_output(context);
        let reputation_metrics_enabled_binding = args
            .reputation_metrics_enabled
            .get_output(context);
        let sending_enabled_binding = args.sending_enabled.get_output(context);
        let tracking_options_binding = args.tracking_options.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/configurationSet:ConfigurationSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryOptions".into(),
                    value: delivery_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reputationMetricsEnabled".into(),
                    value: reputation_metrics_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sendingEnabled".into(),
                    value: sending_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trackingOptions".into(),
                    value: tracking_options_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigurationSetResult {
            arn: o.get_field("arn"),
            delivery_options: o.get_field("deliveryOptions"),
            last_fresh_start: o.get_field("lastFreshStart"),
            name: o.get_field("name"),
            reputation_metrics_enabled: o.get_field("reputationMetricsEnabled"),
            sending_enabled: o.get_field("sendingEnabled"),
            tracking_options: o.get_field("trackingOptions"),
        }
    }
}
