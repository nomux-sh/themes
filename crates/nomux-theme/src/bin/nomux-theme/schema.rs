//! The `schema` command: the JSON Schema of a theme file, from the token set.

use nomux_theme::{ValueKind, token};

const TEMPLATE: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://raw.githubusercontent.com/nomux-sh/themes/main/schema/theme.schema.json",
  "title": "nomux theme",
  "description": "A nomux theme file: values for the semantic design tokens of the nomux UI.",
  "type": "object",
  "required": ["theme"],
  "additionalProperties": false,
  "properties": {
    "theme": {
      "description": "The identity of the theme.",
      "type": "object",
      "required": ["name", "appearance"],
      "additionalProperties": false,
      "properties": {
        "name": {
          "description": "The name that the UI shows.",
          "type": "string",
          "minLength": 1
        },
        "appearance": {
          "description": "Whether the theme is light or dark. It must be equal to the appearance of the base theme.",
          "enum": ["light", "dark"]
        },
        "extends": {
          "description": "The identifier of the base theme: its file name without `.toml`. A token that this file does not set comes from the base theme.",
          "type": "string",
          "pattern": "^[a-z0-9-]+$"
        }
      }
    },
    "tokens": {
      "description": "The values of the tokens. Put each token name in quotes.",
      "type": "object",
      "additionalProperties": false,
      "properties": {
@TOKENS@
      }
    }
  },
  "$defs": {
    "color": {
      "description": "A colour: #rgb, #rgba, #rrggbb or #rrggbbaa.",
      "type": "string",
      "pattern": "^#([0-9a-fA-F]{3,4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$"
    },
    "length": {
      "description": "A length in logical pixels.",
      "type": "string",
      "pattern": "^[0-9]+(\\.[0-9]+)?px$"
    },
    "duration": {
      "description": "A duration in milliseconds.",
      "type": "string",
      "pattern": "^[0-9]+ms$"
    },
    "fontFamily": {
      "description": "A list of font family names, the preferred family first.",
      "type": "array",
      "minItems": 1,
      "items": { "type": "string", "minLength": 1 }
    },
    "fontWeight": {
      "description": "A font weight. 400 is regular, 500 is medium.",
      "type": "integer",
      "minimum": 1,
      "maximum": 1000
    },
    "easing": {
      "description": "A cubic Bézier easing, as in CSS.",
      "type": "string",
      "pattern": "^cubic-bezier\\( *[0-9.]+ *, *-?[0-9.]+ *, *[0-9.]+ *, *-?[0-9.]+ *\\)$"
    }
  }
}
"##;

pub fn render() -> String {
    let tokens: Vec<String> = token::all()
        .map(|spec| {
            let description = json_string(&format!("{} Use: {}", spec.role, spec.usage));
            let definition = match spec.kind {
                ValueKind::Color => "color",
                ValueKind::Length => "length",
                ValueKind::Duration => "duration",
                ValueKind::FontFamily => "fontFamily",
                ValueKind::FontWeight => "fontWeight",
                ValueKind::Easing => "easing",
            };
            format!(
                "        {}: {{\n          \"description\": {description},\n          \"$ref\": \"#/$defs/{definition}\"\n        }}",
                json_string(spec.name)
            )
        })
        .collect();
    TEMPLATE.replace("@TOKENS@", &tokens.join(",\n"))
}

fn json_string(text: &str) -> String {
    let mut json = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => json.push_str("\\\""),
            '\\' => json.push_str("\\\\"),
            control if control.is_control() => json.push_str(&format!("\\u{:04x}", control as u32)),
            other => json.push(other),
        }
    }
    json.push('"');
    json
}
