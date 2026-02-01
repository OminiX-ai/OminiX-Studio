//! Converts A2UI tool calls from the AI model into A2UI JSON protocol messages.
//!
//! This bridges between the OpenAI function calling format (create_text, create_button, etc.)
//! and the A2UI JSON protocol (beginRendering, surfaceUpdate, dataModelUpdate).

use serde_json::{json, Value};

/// Builds A2UI JSON from tool calls received from the AI model.
pub struct A2uiBuilder {
    components: Vec<Value>,
    data_contents: Vec<Value>,
    root_id: Option<String>,
}

impl A2uiBuilder {
    pub fn new() -> Self {
        A2uiBuilder {
            components: Vec::new(),
            data_contents: Vec::new(),
            root_id: None,
        }
    }

    /// Process a single tool call by name and arguments.
    pub fn process_tool_call(&mut self, name: &str, args: &Value) {
        match name {
            "create_text" => self.create_text(args),
            "create_button" => self.create_button(args),
            "create_textfield" => self.create_textfield(args),
            "create_checkbox" => self.create_checkbox(args),
            "create_slider" => self.create_slider(args),
            "create_card" => self.create_card(args),
            "create_column" => self.create_column(args),
            "create_row" => self.create_row(args),
            "set_data" => self.set_data(args),
            "render_ui" => self.render_ui(args),
            _ => ::log::warn!("Unknown A2UI tool: {}", name),
        }
    }

    fn create_text(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("text");

        let text_value = if let Some(data_path) = args["dataPath"].as_str() {
            json!({"path": data_path})
        } else if let Some(text) = args["text"].as_str() {
            json!({"literalString": text})
        } else {
            json!({"literalString": ""})
        };

        let mut component = json!({
            "Text": {
                "text": text_value
            }
        });

        if let Some(style) = args["style"].as_str() {
            component["Text"]["usageHint"] = json!(style);
        }

        self.components.push(json!({
            "id": id,
            "component": component
        }));
    }

    fn create_button(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("button");
        let label = args["label"].as_str().unwrap_or("Button");
        let action = args["action"].as_str().unwrap_or("click");
        let primary = args["primary"].as_bool().unwrap_or(false);

        // Create button text component
        let text_id = format!("{}-text", id);
        self.components.push(json!({
            "id": text_id,
            "component": {
                "Text": {
                    "text": {"literalString": label}
                }
            }
        }));

        // Create button
        self.components.push(json!({
            "id": id,
            "component": {
                "Button": {
                    "child": text_id,
                    "primary": primary,
                    "action": {
                        "name": action,
                        "context": []
                    }
                }
            }
        }));
    }

    fn create_textfield(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("textfield");
        let data_path = args["dataPath"].as_str().unwrap_or("/input");
        let placeholder = args["placeholder"].as_str().unwrap_or("");

        self.components.push(json!({
            "id": id,
            "component": {
                "TextField": {
                    "text": {"path": data_path},
                    "placeholder": {"literalString": placeholder}
                }
            }
        }));
    }

    fn create_checkbox(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("checkbox");
        let label = args["label"].as_str().unwrap_or("Option");
        let data_path = args["dataPath"].as_str().unwrap_or("/checked");

        self.components.push(json!({
            "id": id,
            "component": {
                "CheckBox": {
                    "label": {"literalString": label},
                    "value": {"path": data_path}
                }
            }
        }));
    }

    fn create_slider(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("slider");
        let data_path = args["dataPath"].as_str().unwrap_or("/value");
        let min = args["min"].as_f64().unwrap_or(0.0);
        let max = args["max"].as_f64().unwrap_or(100.0);
        let step = args["step"].as_f64().unwrap_or(1.0);

        self.components.push(json!({
            "id": id,
            "component": {
                "Slider": {
                    "value": {"path": data_path},
                    "min": min,
                    "max": max,
                    "step": step
                }
            }
        }));
    }

    fn create_card(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("card");
        let child_id = args["childId"].as_str().unwrap_or("card-content");

        self.components.push(json!({
            "id": id,
            "component": {
                "Card": {
                    "child": child_id
                }
            }
        }));
    }

    fn create_column(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("column");
        let children: Vec<String> = args["children"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        self.components.push(json!({
            "id": id,
            "component": {
                "Column": {
                    "children": {"explicitList": children}
                }
            }
        }));
    }

    fn create_row(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("row");
        let children: Vec<String> = args["children"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        self.components.push(json!({
            "id": id,
            "component": {
                "Row": {
                    "children": {"explicitList": children}
                }
            }
        }));
    }

    fn set_data(&mut self, args: &Value) {
        let path = args["path"].as_str().unwrap_or("/");
        let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        if parts.is_empty() || parts[0].is_empty() {
            return;
        }

        let value = if let Some(s) = args["stringValue"].as_str() {
            json!({"valueString": s})
        } else if let Some(n) = args["numberValue"].as_f64() {
            json!({"valueNumber": n})
        } else if let Some(b) = args["booleanValue"].as_bool() {
            json!({"valueBoolean": b})
        } else if let Some(n) = args["value"].as_f64() {
            json!({"valueNumber": n})
        } else if let Some(s) = args["value"].as_str() {
            json!({"valueString": s})
        } else if let Some(b) = args["value"].as_bool() {
            json!({"valueBoolean": b})
        } else {
            json!({"valueString": ""})
        };

        let key = parts.last().unwrap_or(&"");
        let mut content = json!({"key": key});

        if let Some(obj) = value.as_object() {
            for (k, v) in obj {
                content[k] = v.clone();
            }
        }

        self.data_contents.push(content);
    }

    fn render_ui(&mut self, args: &Value) {
        if let Some(root_id) = args["rootId"].as_str() {
            self.root_id = Some(root_id.to_string());
        }
    }

    /// Build the final A2UI JSON protocol array from processed tool calls.
    pub fn build_a2ui_json(&self) -> Value {
        let root = self.root_id.as_deref().unwrap_or("root");

        json!([
            {
                "beginRendering": {
                    "surfaceId": "main",
                    "root": root
                }
            },
            {
                "surfaceUpdate": {
                    "surfaceId": "main",
                    "components": self.components
                }
            },
            {
                "dataModelUpdate": {
                    "surfaceId": "main",
                    "path": "/",
                    "contents": self.data_contents
                }
            }
        ])
    }
}

/// Convert a list of A2UI tool calls to A2UI JSON protocol string.
pub fn tool_calls_to_a2ui_json(tool_calls: &[(String, serde_json::Map<String, Value>)]) -> String {
    let mut builder = A2uiBuilder::new();

    for (name, arguments) in tool_calls {
        let args = Value::Object(arguments.clone());
        builder.process_tool_call(name, &args);
    }

    let a2ui_json = builder.build_a2ui_json();
    serde_json::to_string(&a2ui_json).unwrap_or_default()
}
