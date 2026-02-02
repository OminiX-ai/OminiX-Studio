//! A2UI (AI-to-UI) tool definitions for LLM function calling
//!
//! These tools are injected into OpenAI-compatible API requests when A2UI is enabled,
//! allowing LLMs to generate interactive user interfaces.

use serde_json::{json, Value};

/// System prompt that instructs the LLM how to use A2UI tools
pub const A2UI_SYSTEM_PROMPT: &str = r#"You are an A2UI generator assistant. Your job is to create user interfaces by calling the provided tools.

IMPORTANT RULES:
1. Create components using the tools (create_text, create_button, create_slider, etc.)
2. Use create_column for vertical layouts, create_row for horizontal layouts
3. Use create_card to wrap sections in styled containers
4. Set initial data values with set_data for any bound components
5. ALWAYS call render_ui as the LAST step with the root component ID
6. Use descriptive IDs like "title", "volume-slider", "submit-btn"
7. For sliders/checkboxes, always set initial data with set_data
8. Use emojis in text labels to make the UI visually appealing

Example flow for "create a volume control":
1. create_text(id="volume-label", text="🔊 Volume", style="body")
2. create_slider(id="volume-slider", dataPath="/volume", min=0, max=100, step=1)
3. create_text(id="volume-value", dataPath="/volumeDisplay", style="caption")
4. create_row(id="volume-row", children=["volume-label", "volume-slider", "volume-value"])
5. set_data(path="/volume", numberValue=50)
6. set_data(path="/volumeDisplay", stringValue="50%")
7. render_ui(rootId="volume-row")"#;

/// Get all A2UI tool definitions in OpenAI function calling format
pub fn get_a2ui_tools_json() -> Value {
    json!([
        {
            "type": "function",
            "function": {
                "name": "create_text",
                "description": "Create a text/label component to display static or dynamic text",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID (e.g., 'title', 'label-1')"},
                        "text": {"type": "string", "description": "Static text to display"},
                        "dataPath": {"type": "string", "description": "JSON pointer for dynamic text binding (e.g., '/user/name')"},
                        "style": {"type": "string", "enum": ["h1", "h3", "caption", "body"], "description": "Text style: h1=large title, h3=subtitle, caption=small, body=normal"}
                    },
                    "required": ["id"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_button",
                "description": "Create a clickable button that triggers an action",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "label": {"type": "string", "description": "Button text label"},
                        "action": {"type": "string", "description": "Action name triggered on click (e.g., 'submit', 'cancel')"},
                        "primary": {"type": "boolean", "description": "If true, button is highlighted as primary action"}
                    },
                    "required": ["id", "label", "action"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_textfield",
                "description": "Create a text input field for user input",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "dataPath": {"type": "string", "description": "JSON pointer for data binding (e.g., '/form/email')"},
                        "placeholder": {"type": "string", "description": "Placeholder text shown when empty"}
                    },
                    "required": ["id", "dataPath"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_checkbox",
                "description": "Create a checkbox toggle for boolean values",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "label": {"type": "string", "description": "Label text next to checkbox"},
                        "dataPath": {"type": "string", "description": "JSON pointer for boolean binding (e.g., '/settings/darkMode')"}
                    },
                    "required": ["id", "label", "dataPath"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_slider",
                "description": "Create a slider for numeric value selection",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "dataPath": {"type": "string", "description": "JSON pointer for numeric binding (e.g., '/volume')"},
                        "min": {"type": "number", "description": "Minimum value"},
                        "max": {"type": "number", "description": "Maximum value"},
                        "step": {"type": "number", "description": "Step increment (default: 1)"}
                    },
                    "required": ["id", "dataPath", "min", "max"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_card",
                "description": "Create a card container with visual styling (elevation, border)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "childId": {"type": "string", "description": "ID of the child component inside the card"}
                    },
                    "required": ["id", "childId"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_column",
                "description": "Create a vertical layout container (stacks children top to bottom)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "children": {"type": "array", "items": {"type": "string"}, "description": "Array of child component IDs in order"}
                    },
                    "required": ["id", "children"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_row",
                "description": "Create a horizontal layout container (arranges children left to right)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "children": {"type": "array", "items": {"type": "string"}, "description": "Array of child component IDs in order"}
                    },
                    "required": ["id", "children"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "set_data",
                "description": "Set initial data value in the data model",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "JSON pointer path (e.g., '/volume', '/user/name')"},
                        "stringValue": {"type": "string", "description": "String value to set"},
                        "numberValue": {"type": "number", "description": "Number value to set"},
                        "booleanValue": {"type": "boolean", "description": "Boolean value to set"}
                    },
                    "required": ["path"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "render_ui",
                "description": "Finalize and render the UI with the specified root component. Call this LAST after creating all components.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "rootId": {"type": "string", "description": "ID of the root component (usually a column or row)"},
                        "title": {"type": "string", "description": "Optional title for the UI surface"}
                    },
                    "required": ["rootId"]
                }
            }
        }
    ])
}

/// Check if a tool name is an A2UI tool
pub fn is_a2ui_tool(name: &str) -> bool {
    matches!(
        name,
        "create_text"
            | "create_button"
            | "create_textfield"
            | "create_checkbox"
            | "create_slider"
            | "create_card"
            | "create_column"
            | "create_row"
            | "set_data"
            | "render_ui"
    )
}

/// Get the list of A2UI tool names
pub fn a2ui_tool_names() -> &'static [&'static str] {
    &[
        "create_text",
        "create_button",
        "create_textfield",
        "create_checkbox",
        "create_slider",
        "create_card",
        "create_column",
        "create_row",
        "set_data",
        "render_ui",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_a2ui_tool() {
        assert!(is_a2ui_tool("create_text"));
        assert!(is_a2ui_tool("render_ui"));
        assert!(!is_a2ui_tool("get_weather"));
        assert!(!is_a2ui_tool(""));
    }

    #[test]
    fn test_tools_json_is_valid() {
        let tools = get_a2ui_tools_json();
        assert!(tools.is_array());
        assert_eq!(tools.as_array().unwrap().len(), 10);
    }
}
