use crate::description::State::Examples;

struct Description {
    state: State,
    result_lines: Vec<String>,
}

impl Description {
    fn create() -> Self {
        Self { state: State::Initial, result_lines: Vec::new() }
    }

    fn transition(&mut self, line: &str) {
        match &self.state {
            // Initial => Self.initial_transition(line),
            // YAML => Self.yaml_transition(line),
            // Language => Self.language_transition(line),
            // Shell => Self.shell_transition(line),
            State::Initial => { Self.initial_transition(line) }
            State::Examples => { Self.example_transition(line) }
            State::YAML(yaml_lines) => { Self.yaml_transition(line, yaml_lines) }
            State::Language => { Self.language_transition(line) }
            State::Shell => { Self.shell_transition(line) }
        }
    }

    fn initial_transition(&mut self, line: &str) {
        self.state = Examples
    }
}

enum State {
    Initial,
    Examples,
    YAML(Vec<String>),
    Language,
    Shell,
}

impl State {


    fn initial_transition(&self, line: &str) -> (Self, Vec<String>) {
        match line.trim() {
            "```yaml" => (Self::YAML, vec![line.to_string()]),
            "```typescript" | "```python" | ... => (Self::Language, vec![line.to_string()]),
            "```" | "```sh" | "```shell" | "```text" => (Self::Shell, vec![line.to_string()]),
            _ => (Self::Initial, vec![line.to_string()]),
        }
    }

    fn yaml_transition(&self, line: &str) -> (Self, Vec<String>) {
        if line.trim() == "```" {
            // Process YAML lines here, e.g., generate Rust code
            let processed_lines = generate_rust_code_from_yaml(yaml_lines);
            (Self::Initial, processed_lines)
        } else {
            yaml_lines.push(line.to_string());
            (Self::YAML, vec![])
        }
    }

    fn language_transition(&self, line: &str) -> (Self, Vec<String>) {
        if line.trim() == "```" {
            // Keep the captured language-specific code
            (Self::Initial, language_code_lines.clone())
        } else {
            language_code_lines.push(line.to_string());
            (Self::Language, vec![])
        }
    }

    fn shell_transition(&self, line: &str) -> (Self, Vec<String>) {
        if line.trim() == "```" {
            // Keep the captured shell script code
            (Self::Initial, shell_code_lines.clone())
        } else {
            shell_code_lines.push(line.to_string());
            (Self::Shell, vec![])
        }
    }
}
