/// Subscribes to a Security Hub product.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:securityhub:Account
///   exampleProductSubscription:
///     type: aws:securityhub:ProductSubscription
///     name: example
///     properties:
///       productArn: arn:aws:securityhub:${current.name}:733251395267:product/alertlogic/althreatmanagement
///     options:
///       dependsOn:
///         - ${example}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Hub product subscriptions using `product_arn,arn`. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/productSubscription:ProductSubscription example arn:aws:securityhub:eu-west-1:733251395267:product/alertlogic/althreatmanagement,arn:aws:securityhub:eu-west-1:123456789012:product-subscription/alertlogic/althreatmanagement
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod product_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductSubscriptionArgs {
        /// The ARN of the product that generates findings that you want to import into Security Hub - see below.
        ///
        /// Amazon maintains a list of [Product integrations in AWS Security Hub](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-findings-providers.html) that changes over time. Any of the products on the linked [Available AWS service integrations](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-internal-providers.html) or [Available third-party partner product integrations](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-partner-providers.html) can be configured using `aws.securityhub.ProductSubscription`.
        ///
        /// Available products can also be listed by running the AWS CLI command `aws securityhub describe-products`.
        ///
        /// A subset of currently available products (remember to replace `${var.region}` as appropriate) includes:
        ///
        /// * `arn:aws:securityhub:${var.region}::product/aws/guardduty`
        /// * `arn:aws:securityhub:${var.region}::product/aws/inspector`
        /// * `arn:aws:securityhub:${var.region}::product/aws/macie`
        /// * `arn:aws:securityhub:${var.region}::product/alertlogic/althreatmanagement`
        /// * `arn:aws:securityhub:${var.region}::product/armordefense/armoranywhere`
        /// * `arn:aws:securityhub:${var.region}::product/barracuda/cloudsecurityguardian`
        /// * `arn:aws:securityhub:${var.region}::product/checkpoint/cloudguard-iaas`
        /// * `arn:aws:securityhub:${var.region}::product/checkpoint/dome9-arc`
        /// * `arn:aws:securityhub:${var.region}::product/crowdstrike/crowdstrike-falcon`
        /// * `arn:aws:securityhub:${var.region}::product/cyberark/cyberark-pta`
        /// * `arn:aws:securityhub:${var.region}::product/f5networks/f5-advanced-waf`
        /// * `arn:aws:securityhub:${var.region}::product/fortinet/fortigate`
        /// * `arn:aws:securityhub:${var.region}::product/guardicore/aws-infection-monkey`
        /// * `arn:aws:securityhub:${var.region}::product/guardicore/guardicore`
        /// * `arn:aws:securityhub:${var.region}::product/ibm/qradar-siem`
        /// * `arn:aws:securityhub:${var.region}::product/imperva/imperva-attack-analytics`
        /// * `arn:aws:securityhub:${var.region}::product/mcafee-skyhigh/mcafee-mvision-cloud-aws`
        /// * `arn:aws:securityhub:${var.region}::product/paloaltonetworks/redlock`
        /// * `arn:aws:securityhub:${var.region}::product/paloaltonetworks/vm-series`
        /// * `arn:aws:securityhub:${var.region}::product/qualys/qualys-pc`
        /// * `arn:aws:securityhub:${var.region}::product/qualys/qualys-vm`
        /// * `arn:aws:securityhub:${var.region}::product/rapid7/insightvm`
        /// * `arn:aws:securityhub:${var.region}::product/sophos/sophos-server-protection`
        /// * `arn:aws:securityhub:${var.region}::product/splunk/splunk-enterprise`
        /// * `arn:aws:securityhub:${var.region}::product/splunk/splunk-phantom`
        /// * `arn:aws:securityhub:${var.region}::product/sumologicinc/sumologic-mda`
        /// * `arn:aws:securityhub:${var.region}::product/symantec-corp/symantec-cwp`
        /// * `arn:aws:securityhub:${var.region}::product/tenable/tenable-io`
        /// * `arn:aws:securityhub:${var.region}::product/trend-micro/deep-security`
        /// * `arn:aws:securityhub:${var.region}::product/turbot/turbot`
        /// * `arn:aws:securityhub:${var.region}::product/twistlock/twistlock-enterprise`
        #[builder(into)]
        pub product_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProductSubscriptionResult {
        /// The ARN of a resource that represents your subscription to the product that generates the findings that you want to import into Security Hub.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the product that generates findings that you want to import into Security Hub - see below.
        ///
        /// Amazon maintains a list of [Product integrations in AWS Security Hub](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-findings-providers.html) that changes over time. Any of the products on the linked [Available AWS service integrations](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-internal-providers.html) or [Available third-party partner product integrations](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-partner-providers.html) can be configured using `aws.securityhub.ProductSubscription`.
        ///
        /// Available products can also be listed by running the AWS CLI command `aws securityhub describe-products`.
        ///
        /// A subset of currently available products (remember to replace `${var.region}` as appropriate) includes:
        ///
        /// * `arn:aws:securityhub:${var.region}::product/aws/guardduty`
        /// * `arn:aws:securityhub:${var.region}::product/aws/inspector`
        /// * `arn:aws:securityhub:${var.region}::product/aws/macie`
        /// * `arn:aws:securityhub:${var.region}::product/alertlogic/althreatmanagement`
        /// * `arn:aws:securityhub:${var.region}::product/armordefense/armoranywhere`
        /// * `arn:aws:securityhub:${var.region}::product/barracuda/cloudsecurityguardian`
        /// * `arn:aws:securityhub:${var.region}::product/checkpoint/cloudguard-iaas`
        /// * `arn:aws:securityhub:${var.region}::product/checkpoint/dome9-arc`
        /// * `arn:aws:securityhub:${var.region}::product/crowdstrike/crowdstrike-falcon`
        /// * `arn:aws:securityhub:${var.region}::product/cyberark/cyberark-pta`
        /// * `arn:aws:securityhub:${var.region}::product/f5networks/f5-advanced-waf`
        /// * `arn:aws:securityhub:${var.region}::product/fortinet/fortigate`
        /// * `arn:aws:securityhub:${var.region}::product/guardicore/aws-infection-monkey`
        /// * `arn:aws:securityhub:${var.region}::product/guardicore/guardicore`
        /// * `arn:aws:securityhub:${var.region}::product/ibm/qradar-siem`
        /// * `arn:aws:securityhub:${var.region}::product/imperva/imperva-attack-analytics`
        /// * `arn:aws:securityhub:${var.region}::product/mcafee-skyhigh/mcafee-mvision-cloud-aws`
        /// * `arn:aws:securityhub:${var.region}::product/paloaltonetworks/redlock`
        /// * `arn:aws:securityhub:${var.region}::product/paloaltonetworks/vm-series`
        /// * `arn:aws:securityhub:${var.region}::product/qualys/qualys-pc`
        /// * `arn:aws:securityhub:${var.region}::product/qualys/qualys-vm`
        /// * `arn:aws:securityhub:${var.region}::product/rapid7/insightvm`
        /// * `arn:aws:securityhub:${var.region}::product/sophos/sophos-server-protection`
        /// * `arn:aws:securityhub:${var.region}::product/splunk/splunk-enterprise`
        /// * `arn:aws:securityhub:${var.region}::product/splunk/splunk-phantom`
        /// * `arn:aws:securityhub:${var.region}::product/sumologicinc/sumologic-mda`
        /// * `arn:aws:securityhub:${var.region}::product/symantec-corp/symantec-cwp`
        /// * `arn:aws:securityhub:${var.region}::product/tenable/tenable-io`
        /// * `arn:aws:securityhub:${var.region}::product/trend-micro/deep-security`
        /// * `arn:aws:securityhub:${var.region}::product/turbot/turbot`
        /// * `arn:aws:securityhub:${var.region}::product/twistlock/twistlock-enterprise`
        pub product_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProductSubscriptionArgs,
    ) -> ProductSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let product_arn_binding = args.product_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securityhub/productSubscription:ProductSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productArn".into(),
                    value: product_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProductSubscriptionResult {
            arn: o.get_field("arn"),
            product_arn: o.get_field("productArn"),
        }
    }
}
