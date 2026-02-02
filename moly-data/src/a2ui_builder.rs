//! A2UI Builder - Converts LLM tool calls to A2UI JSON protocol
//!
//! This module accumulates A2UI tool calls from an LLM response and builds
//! the final A2UI JSON that can be rendered by an A2uiSurface widget.

use serde_json::{json, Value};

/// Builder that accumulates A2UI tool calls and generates A2UI JSON
#[derive(Debug, Clone, Default)]
pub struct A2uiBuilder {
    /// Component definitions
    components: Vec<Value>,
    /// Data model contents
    data_contents: Vec<Value>,
    /// Root component ID (set by render_ui)
    root_id: Option<String>,
    /// Surface title (optional)
    title: Option<String>,
}

impl A2uiBuilder {
    /// Create a new empty builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a single tool call and accumulate its result
    ///
    /// Returns a human-readable description of the action taken.
    pub fn process_tool_call(&mut self, name: &str, args: &Value) -> Result<String, String> {
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
            _ => Ok(format!("Unknown A2UI tool: {}", name)),
        }
    }

    /// Check if the UI is ready to render (render_ui was called)
    pub fn is_complete(&self) -> bool {
        self.root_id.is_some()
    }

    /// Get the root component ID if set
    pub fn root_id(&self) -> Option<&str> {
        self.root_id.as_deref()
    }

    /// Build the final A2UI JSON array
    ///
    /// Returns `None` if `render_ui` hasn't been called yet.
    pub fn build(&self) -> Option<String> {
        let root_id = self.root_id.as_ref()?;

        let a2ui_json = json!([
            {
                "beginRendering": {
                    "surfaceId": "canvas",
                    "root": root_id
                }
            },
            {
                "surfaceUpdate": {
                    "surfaceId": "canvas",
                    "components": self.components
                }
            },
            {
                "dataModelUpdate": {
                    "surfaceId": "canvas",
                    "path": "/",
                    "contents": self.data_contents
                }
            }
        ]);

        serde_json::to_string_pretty(&a2ui_json).ok()
    }

    /// Build the A2UI JSON as a Value (for further processing)
    pub fn build_value(&self) -> Option<Value> {
        let root_id = self.root_id.as_ref()?;

        Some(json!([
            {
                "beginRendering": {
                    "surfaceId": "canvas",
                    "root": root_id
                }
            },
            {
                "surfaceUpdate": {
                    "surfaceId": "canvas",
                    "components": self.components
                }
            },
            {
                "dataModelUpdate": {
                    "surfaceId": "canvas",
                    "path": "/",
                    "contents": self.data_contents
                }
            }
        ]))
    }

    /// Reset the builder for a new UI generation session
    pub fn reset(&mut self) {
        self.components.clear();
        self.data_contents.clear();
        self.root_id = None;
        self.title = None;
    }

    /// Get the number of components created
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    // --- Private component builders ---

    fn create_text(&mut self, args: &Value) -> Result<String, String> {
        let id = args["id"].as_str().ok_or("create_text: missing 'id'")?;

        let text_value = if let Some(path) = args.get("dataPath").and_then(|p| p.as_str()) {
            json!({"path": path})
        } else if let Some(text) = args.get("text").and_then(|t| t.as_str()) {
            json!({"literalString": text})
        } else {
            json!({"literalString": ""})
        };

        let style = args
            .get("style")
            .and_then(|s| s.as_str())
            .unwrap_or("body");

        self.components.push(json!({
            "id": id,
            "component": {
                "Text": {
                    "text": text_value,
                    "usageHint": style
                }
            }
        }));

        Ok(format!("Created text '{}'", id))
    }

    fn create_button(&mut self, args: &Value) -> Result<String, String> {
        let id = args["id"].as_str().ok_or("create_button: missing 'id'")?;
        let label = args
            .get("label")
            .and_then(|l| l.as_str())
            .unwrap_or("Button");
        let action = args.get("action").and_then(|a| a.as_str());
        let primary = args.get("primary").and_then(|p| p.as_bool()).unwrap_or(false);

        // Button needs a child text component for the label
        let text_id = format!("{}-text", id);
        self.components.push(json!({
            "id": text_id,
            "component": {
                "Text": {
                    "text": {"literalString": label}
                }
            }
        }));

        let mut button = json!({
            "id": id,
            "component": {
                "Button": {
                    "child": text_id,
                    "primary": primary
                }
            }
        });

        if let Some(action_name) = action {
            button["component"]["Button"]["action"] = json!({
                "name": action_name,
                "context": []
            });
        }

        self.components.push(button);
        Ok(format!("Created button '{}'", id))
    }

    fn create_textfield(&mut self, args: &Value) -> Result<String, String> {
        let id = args["id"]
            .as_str()
            .ok_or("create_textfield: missing 'id'")?;
        let placeholder = args.get("placeholder").and_then(|p| p.as_str());
        let data_path = args.get("dataPath").and_then(|d| d.as_str());

        let mut textfield = json!({
            "id": id,
            "component": {
                "TextField": {}
            }
        });

        if let Some(ph) = placeholder {
            textfield["component"]["TextField"]["placeholder"] = json!({"literalString": ph});
        }
        if let Some(path) = data_path {
            textfield["component"]["TextField"]["value"] = json!({"path": path});
        }

        self.components.push(textfield);
        Ok(format!("Created textfield '{}'", id))
    }

    fn create_checkbox(&mut self, args: &Value) -> Result<String, String> {
        let id = args["id"]
            .as_str()
            .ok_or("create_checkbox: missing 'id'")?;
        let label = args.get("label").and_then(|l| l.as_str());
        let data_path = args.get("dataPath").and_then(|d| d.as_str());

        let mut checkbox = json!({
            "id": id,
            "component": {
                "CheckBox": {}
            }
        });

        if let Some(lbl) = label {
            checkbox["component"]["CheckBox"]["label"] = json!({"literalString": lbl});
        }
        if let Some(path) = data_path {
            checkbox["component"]["CheckBox"]["checked"] = json!({"path": path});
        }

        self.components.push(checkbox);
        Ok(format!("Created checkbox '{}'", id))
    }

    fn create_slider(&mut self, args: &Value) -> Result<String, String> {
        let id = args["id"].as_str().ok_or("create_slider: missing 'id'")?;
        let min = args.get("min").and_then(|m| m.as_f64()).unwrap_or(0.0);
        let max = args.get("max").and_then(|m| m.as_f64()).unwrap_or(100.0);
        let data_path = args.get("dataPath").and_then(|d| d.as_str());

        let mut slider = json!({
            "id": id,
            "component": {
                "Slider": {
                    "min": {"literalNumber": min},
                    "max": {"literalNumber": max}
                }
            }
        });

        if let Some(path) = data_path {
            slider["component"]["Slider"]["value"] = json!({"path": path});
        }

        self.components.push(slider);
        Ok(format!("Created slider '{}'", id))
    }

    fn create_card(&mut self, args: &Value) -> Result<String, String> {
        let id = args["id"].as_str().ok_or("create_card: missing 'id'")?;
        let child_id = args.get("childId").and_then(|c| c.as_str());

        let mut card = json!({
            "id": id,
            "component": {
                "Card": {}
            }
        });

        if let Some(child) = child_id {
            card["component"]["Card"]["child"] = json!(child);
        }

        self.components.push(card);
        Ok(format!("Created card '{}'", id))
    }

    fn create_column(&mut self, args: &Value) -> Result<String, String> {
        let id = args["id"].as_str().ok_or("create_column: missing 'id'")?;
        let children: Vec<String> = args
            .get("children")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        self.components.push(json!({
            "id": id,
            "component": {
                "Column": {
                    "children": {"explicitList": children}
                }
            }
        }));

        Ok(format!("Created column '{}' with {} children", id, children.len()))
    }

    fn create_row(&mut self, args: &Value) -> Result<String, String> {
        let id = args["id"].as_str().ok_or("create_row: missing 'id'")?;
        let children: Vec<String> = args
            .get("children")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        self.components.push(json!({
            "id": id,
            "component": {
                "Row": {
                    "children": {"explicitList": children}
                }
            }
        }));

        Ok(format!("Created row '{}' with {} children", id, children.len()))
    }

    fn set_data(&mut self, args: &Value) -> Result<String, String> {
        let path = args["path"].as_str().ok_or("set_data: missing 'path'")?;

        let content = if let Some(s) = args.get("stringValue").and_then(|v| v.as_str()) {
            json!({"key": path, "valueString": s})
        } else if let Some(n) = args.get("numberValue").and_then(|v| v.as_f64()) {
            json!({"key": path, "valueNumber": n})
        } else if let Some(b) = args.get("booleanValue").and_then(|v| v.as_bool()) {
            json!({"key": path, "valueBoolean": b})
        } else {
            return Err("set_data: missing value (stringValue, numberValue, or booleanValue)".into());
        };

        self.data_contents.push(content);
        Ok(format!("Set data at '{}'", path))
    }

    fn render_ui(&mut self, args: &Value) -> Result<String, String> {
        let root_id = args["rootId"]
            .as_str()
            .ok_or("render_ui: missing 'rootId'")?;

        self.root_id = Some(root_id.to_string());

        if let Some(title) = args.get("title").and_then(|t| t.as_str()) {
            self.title = Some(title.to_string());
        }

        Ok(format!(
            "UI ready to render with root '{}' ({} components)",
            root_id,
            self.components.len()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_empty() {
        let builder = A2uiBuilder::new();
        assert!(!builder.is_complete());
        assert!(builder.build().is_none());
    }

    #[test]
    fn test_builder_simple_ui() {
        let mut builder = A2uiBuilder::new();

        builder
            .process_tool_call("create_text", &json!({"id": "title", "text": "Hello"}))
            .unwrap();
        builder
            .process_tool_call(
                "create_column",
                &json!({"id": "root", "children": ["title"]}),
            )
            .unwrap();
        builder
            .process_tool_call("render_ui", &json!({"rootId": "root"}))
            .unwrap();

        assert!(builder.is_complete());
        assert_eq!(builder.root_id(), Some("root"));
        assert_eq!(builder.component_count(), 2);

        let json = builder.build().unwrap();
        assert!(json.contains("beginRendering"));
        assert!(json.contains("surfaceUpdate"));
        assert!(json.contains("\"root\": \"root\""));
    }

    #[test]
    fn test_builder_with_data() {
        let mut builder = A2uiBuilder::new();

        builder
            .process_tool_call(
                "create_slider",
                &json!({"id": "vol", "dataPath": "/volume", "min": 0, "max": 100}),
            )
            .unwrap();
        builder
            .process_tool_call(
                "set_data",
                &json!({"path": "/volume", "numberValue": 50}),
            )
            .unwrap();
        builder
            .process_tool_call("create_column", &json!({"id": "root", "children": ["vol"]}))
            .unwrap();
        builder
            .process_tool_call("render_ui", &json!({"rootId": "root"}))
            .unwrap();

        let json = builder.build().unwrap();
        assert!(json.contains("dataModelUpdate"));
        assert!(json.contains("/volume"));
    }

    #[test]
    fn test_builder_reset() {
        let mut builder = A2uiBuilder::new();

        builder
            .process_tool_call("create_text", &json!({"id": "test"}))
            .unwrap();
        builder
            .process_tool_call("render_ui", &json!({"rootId": "test"}))
            .unwrap();

        assert!(builder.is_complete());

        builder.reset();

        assert!(!builder.is_complete());
        assert_eq!(builder.component_count(), 0);
    }
}
