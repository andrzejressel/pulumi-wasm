use crate::code_generation::generate_code_from_string;
use crate::description::State::{
    Examples, Initial, LanguageInExamples, LanguageOutsideExamples, Shell, Yaml,
};
use crate::model::Package;

struct DescriptionState {
    state: State,
    result_lines: Vec<String>,
}

pub(crate) struct Description<'a> {
    description_state: Wrapper,
    package: &'a Package,
}

impl<'a> Description<'a> {
    pub(crate) fn create(package: &'a Package) -> Description<'a> {
        Self {
            description_state: Wrapper::new(),
            package,
        }
    }

    pub(crate) fn transition(&mut self, line: &str) {
        self.description_state.update(|previous_state| {
            let DescriptionState {
                state,
                mut result_lines,
            } = previous_state;

            let (new_state, lines) = match state {
                Initial => Self::initial_transition(line),
                Examples => Self::examples_transition(line),
                Yaml(yaml_lines) => Self::yaml_transition(line, yaml_lines, self.package),
                LanguageInExamples => Self::language_transition(line),
                LanguageOutsideExamples => Self::language_outside_examples_transition(line),
                Shell => Self::shell_transition(line),
            };
            result_lines.extend(lines);
            DescriptionState {
                state: new_state,
                result_lines,
            }
        });
    }

    pub(crate) fn get(self) -> Vec<String> {
        self.description_state.inner.result_lines
    }

    fn initial_transition(line: &str) -> (State, Vec<String>) {
        match line.trim() {
            "<!--Start PulumiCodeChooser -->" | "{{% examples %}}" => (Examples, vec![]),
            // Rustdoc treats ``` as rust code block. Line may contain whitespace before
            "```" => (LanguageOutsideExamples, vec![line.to_string() + "sh"]),
            l if l.starts_with("```") => (LanguageOutsideExamples, vec![line.to_string()]),
            _ => (Initial, vec![line.to_string()]),
        }
    }

    fn examples_transition(line: &str) -> (State, Vec<String>) {
        match line.trim() {
            "```yaml" => (Yaml(vec![]), vec![]),
            "```typescript" | "```python" | "```java" | "```go" | "```csharp" => {
                (LanguageInExamples, vec![])
            }
            "{{% example %}}" | "{{% /example %}}" => (Examples, vec![]),
            "{{% /examples %}}" | "<!--End PulumiCodeChooser -->" => (Initial, vec![]),
            _ => (Examples, vec![line.to_string()]),
        }
    }

    fn language_transition(line: &str) -> (State, Vec<String>) {
        match line.trim() {
            "```" => (Examples, vec![]),
            _ => (LanguageInExamples, vec![]),
        }
    }

    fn shell_transition(line: &str) -> (State, Vec<String>) {
        match line.trim() {
            "```" => (Initial, vec![line.to_string()]),
            _ => (Shell, vec![line.to_string()]),
        }
    }

    fn language_outside_examples_transition(line: &str) -> (State, Vec<String>) {
        match line.trim() {
            "```" => (Initial, vec![line.to_string()]),
            _ => (LanguageOutsideExamples, vec![line.to_string()]),
        }
    }

    fn yaml_transition(
        line: &str,
        mut yaml_lines: Vec<String>,
        package: &Package,
    ) -> (State, Vec<String>) {
        match line.trim() {
            "```" => {
                let yaml_str = yaml_lines.join("\n");
                let example = generate_code_from_string(yaml_str, package);
                let mut new_lines = Vec::new();

                match example {
                    Ok(rust_example) => {
                        new_lines.push("```ignore".to_string());
                        new_lines.extend(
                            rust_example
                                .lines()
                                .map(|f| f.to_string())
                                .collect::<Vec<_>>(),
                        );
                        new_lines.push("```".to_string());
                    }
                    Err(err) => {
                        eprintln!("ERROR: {:?}", err);
                        new_lines.push("```yaml".to_string());
                        new_lines.extend(yaml_lines.clone());
                        new_lines.push("```".to_string());
                    }
                }

                (Examples, new_lines)
            }
            _ => {
                yaml_lines.push(line.to_string());
                (Yaml(yaml_lines), vec![])
            }
        }
    }
}

enum State {
    Initial,
    Examples,
    Yaml(Vec<String>),
    LanguageInExamples,
    LanguageOutsideExamples,
    Shell,
}

struct Wrapper {
    inner: DescriptionState,
}

impl Wrapper {
    fn new() -> Self {
        Wrapper {
            inner: DescriptionState {
                state: Initial,
                result_lines: vec![],
            },
        }
    }
    fn update<F>(&mut self, updater: F)
    where
        F: FnOnce(DescriptionState) -> DescriptionState,
    {
        let new_inner = updater(std::mem::replace(
            &mut self.inner,
            DescriptionState {
                state: Initial,
                result_lines: vec![],
            },
        ));
        self.inner = new_inner;
    }
}
