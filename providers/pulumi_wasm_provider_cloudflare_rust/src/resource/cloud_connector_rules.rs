//! The Cloud Connector Rules resource allows you to create and manage cloud connector rules for a zone.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Java
//! ```java
//! package generated_program;
//! 
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.cloudflare.CloudConnectorRules;
//! import com.pulumi.cloudflare.CloudConnectorRulesArgs;
//! import com.pulumi.cloudflare.inputs.CloudConnectorRulesRuleArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//! 
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//! 
//!     public static void stack(Context ctx) {
//!         var example = new CloudConnectorRules("example", CloudConnectorRulesArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .rules(CloudConnectorRulesRuleArgs.builder()
//!                 .description("connect aws bucket")
//!                 .enabled(true)
//!                 .expression("http.uri")
//!                 .provider("aws_s3")
//!                 .parameters(CloudConnectorRulesRuleParametersArgs.builder()
//!                     .host("mystorage.s3.ams.amazonaws.com")
//!                     .build())
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:CloudConnectorRules
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       rules:
//!         - description: connect aws bucket
//!           enabled: true
//!           expression: http.uri
//!           provider: aws_s3
//!           parameters:
//!             - host: mystorage.s3.ams.amazonaws.com
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct CloudConnectorRulesArgs {
    /// List of Cloud Connector Rules
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::CloudConnectorRulesRule>>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CloudConnectorRulesResult {
    /// List of Cloud Connector Rules
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::CloudConnectorRulesRule>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: CloudConnectorRulesArgs) -> CloudConnectorRulesResult {

    let result = crate::bindings::pulumi::cloudflare::cloud_connector_rules::invoke(name, &crate::bindings::pulumi::cloudflare::cloud_connector_rules::Args {
        rules: &args.rules.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    CloudConnectorRulesResult {
        rules: crate::into_domain(result.rules),
        zone_id: crate::into_domain(result.zone_id),
    }
}
