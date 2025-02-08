/// Manages a Cost Anomaly Alert.
///
/// > **Note:** Anomaly alerts are sent based on the current access of the rule creator at the time that the email is sent. Learn more [here](https://learn.microsoft.com/en-us/azure/cost-management-billing/understand/analyze-unexpected-charges#create-an-anomaly-alert).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = anomaly_alert::create(
///         "example",
///         AnomalyAlertArgs::builder()
///             .display_name("Alert DisplayName")
///             .email_addresses(vec!["example@test.net",])
///             .email_subject("My Test Anomaly Alert")
///             .name("alertname")
///             .subscription_id("/subscriptions/00000000-0000-0000-0000-000000000000")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cost Anomaly Alerts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:costmanagement/anomalyAlert:AnomalyAlert example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.CostManagement/scheduledActions/dailyanomalybyresourcegroup
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod anomaly_alert {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnomalyAlertArgs {
        /// The display name which should be used for this Cost Anomaly Alert.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a list of email addresses which the Anomaly Alerts are send to.
        #[builder(into)]
        pub email_addresses: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The email subject of the Cost Anomaly Alerts. Maximum length of the subject is 70.
        #[builder(into)]
        pub email_subject: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The message of the Cost Anomaly Alert. Maximum length of the message is 250.
        #[builder(into, default)]
        pub message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Cost Anomaly Alert. Changing this forces a new resource to be created. The name can contain only lowercase letters, numbers and hyphens.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Subscription this Cost Anomaly Alert is scoped to. Changing this forces a new resource to be created. When not supplied this defaults to the subscription configured in the provider.
        #[builder(into, default)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AnomalyAlertResult {
        /// The display name which should be used for this Cost Anomaly Alert.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of email addresses which the Anomaly Alerts are send to.
        pub email_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The email subject of the Cost Anomaly Alerts. Maximum length of the subject is 70.
        pub email_subject: pulumi_gestalt_rust::Output<String>,
        /// The message of the Cost Anomaly Alert. Maximum length of the message is 250.
        pub message: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Cost Anomaly Alert. Changing this forces a new resource to be created. The name can contain only lowercase letters, numbers and hyphens.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subscription this Cost Anomaly Alert is scoped to. Changing this forces a new resource to be created. When not supplied this defaults to the subscription configured in the provider.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AnomalyAlertArgs,
    ) -> AnomalyAlertResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let email_addresses_binding = args
            .email_addresses
            .get_output(context)
            .get_inner();
        let email_subject_binding = args.email_subject.get_output(context).get_inner();
        let message_binding = args.message.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:costmanagement/anomalyAlert:AnomalyAlert".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddresses".into(),
                    value: &email_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "emailSubject".into(),
                    value: &email_subject_binding,
                },
                register_interface::ObjectField {
                    name: "message".into(),
                    value: &message_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AnomalyAlertResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            email_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("emailAddresses"),
            ),
            email_subject: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("emailSubject"),
            ),
            message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("message"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            subscription_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
        }
    }
}
