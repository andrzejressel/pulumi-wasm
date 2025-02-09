use crate::{Context, Output};
use pulumi_gestalt_rust_adapter::GestaltContext;
use pulumi_gestalt_rust_adapter::GestaltOutput;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Wrapper for either static value or [Output]
pub enum InputOrOutput<T> {
    StaticValue(T),
    Output(Output<T>),
}

impl<T: Serialize> InputOrOutput<T> {
    #[doc(hidden)]
    pub fn get_output(self, engine: &Context) -> Output<T> {
        match self {
            InputOrOutput::StaticValue(value) => engine.new_output(&value),
            InputOrOutput::Output(output) => output,
        }
    }
}

impl<T> From<Output<T>> for InputOrOutput<T> {
    fn from(output: Output<T>) -> Self {
        InputOrOutput::Output(output)
    }
}

impl<T: Default + Serialize> Default for InputOrOutput<T> {
    fn default() -> Self {
        InputOrOutput::StaticValue(Default::default())
    }
}

impl<T: Serialize> From<T> for InputOrOutput<T> {
    fn from(value: T) -> InputOrOutput<T> {
        InputOrOutput::StaticValue(value)
    }
}

impl<T: Serialize> From<T> for InputOrOutput<Option<T>> {
    fn from(value: T) -> Self {
        InputOrOutput::StaticValue(Some(value))
    }
}

impl<T: Serialize + DeserializeOwned> From<Output<T>> for InputOrOutput<Option<T>> {
    fn from(output: Output<T>) -> Self {
        InputOrOutput::Output(output.map(|v| Some(v)))
    }
}

impl From<&str> for InputOrOutput<String> {
    fn from(value: &str) -> Self {
        InputOrOutput::StaticValue(value.to_string())
    }
}

impl From<&str> for InputOrOutput<Option<String>> {
    fn from(value: &str) -> Self {
        InputOrOutput::StaticValue(Some(value.to_string()))
    }
}

impl From<Vec<&str>> for InputOrOutput<Vec<String>> {
    fn from(value: Vec<&str>) -> Self {
        InputOrOutput::StaticValue(value.into_iter().map(|s| s.to_string()).collect())
    }
}

impl From<Vec<&str>> for InputOrOutput<Option<Vec<String>>> {
    fn from(value: Vec<&str>) -> Self {
        InputOrOutput::StaticValue(Some(value.into_iter().map(|s| s.to_string()).collect()))
    }
}

impl<T: Serialize, const N: usize> From<[T; N]> for InputOrOutput<Vec<T>>
where
    T: Serialize,
{
    fn from(value: [T; N]) -> Self {
        InputOrOutput::StaticValue(value.into_iter().collect())
    }
}

impl<T: Serialize, const N: usize> From<[T; N]> for InputOrOutput<Option<Vec<T>>>
where
    T: Serialize,
{
    fn from(value: [T; N]) -> Self {
        InputOrOutput::StaticValue(Some(value.into_iter().collect()))
    }
}

impl<const N: usize> From<[&str; N]> for InputOrOutput<Vec<String>> {
    fn from(value: [&str; N]) -> Self {
        InputOrOutput::StaticValue(value.into_iter().map(|s| s.to_string()).collect())
    }
}

impl<const N: usize> From<[&str; N]> for InputOrOutput<Option<Vec<String>>> {
    fn from(value: [&str; N]) -> Self {
        InputOrOutput::StaticValue(Some(value.into_iter().map(|s| s.to_string()).collect()))
    }
}
