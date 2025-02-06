#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentAgentActionGroupActionGroupExecutor {
    /// Custom control method for handling the information elicited from the user. Valid values: `RETURN_CONTROL`.
    /// To skip using a Lambda function and instead return the predicted action group, in addition to the parameters and information required for it, in the `InvokeAgent` response, specify `RETURN_CONTROL`.
    /// Only one of `custom_control` or `lambda` can be specified.
    #[builder(into, default)]
    #[serde(rename = "customControl")]
    pub r#custom_control: Box<Option<String>>,
    /// ARN of the Lambda function containing the business logic that is carried out upon invoking the action.
    /// Only one of `lambda` or `custom_control` can be specified.
    #[builder(into, default)]
    #[serde(rename = "lambda")]
    pub r#lambda: Box<Option<String>>,
}
