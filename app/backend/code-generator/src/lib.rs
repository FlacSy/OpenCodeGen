use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;


#[derive(Deserialize, Serialize)]
struct OpenApiSchema {
    components: Option<Components>,
}

#[derive(Deserialize, Serialize)]
#[derive(Default)]
struct Components {
    schemas: HashMap<String, Schema>,
}

#[derive(Deserialize, Serialize)]
struct Schema {
    properties: Option<HashMap<String, Property>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOfVariant {
    Null,
    Ref { #[serde(rename = "$ref")] r#ref: String },
    Object(Property),
    Type(Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type", default)]
    pub r#type: Option<serde_json::Value>,

    #[serde(rename = "$ref", default)]
    pub r#ref: Option<String>,

    #[serde(default)]
    pub format: Option<String>,

    #[serde(default)]
    pub items: Option<Box<Property>>,

    #[serde(rename = "oneOf")]
    one_of: Option<Vec<OneOfVariant>>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub default: Option<String>,

}
fn parse_property_py(prop: &Property, schemas: &HashMap<String, Schema>) -> String {
    if let Some(r#ref) = &prop.r#ref {
        return format!("'{}'", r#ref.split('/').last().unwrap());
    }

    if let Some(one_of) = &prop.one_of {
        let mut types: Vec<String> = Vec::new();
        let mut has_null = false;
        for variant in one_of {
            match variant {
                OneOfVariant::Ref { r#ref } => {
                    types.push(format!("'{}'", r#ref.split('/').last().unwrap()));
                },
                OneOfVariant::Object(obj_prop) => {
                    let t = parse_property_py(obj_prop, schemas);
                    if t == "None" {
                        has_null = true;
                    } else {
                        types.push(t);
                    }
                },
                OneOfVariant::Type(value) => {
                    if let Some(s) = value.as_str() {
                        if s == "null" {
                            has_null = true;
                        } else {
                            let t = match s {
                                "integer" => "int".to_string(),
                                "string" => "str".to_string(),
                                "boolean" => "bool".to_string(),
                                "number" => "float".to_string(),
                                "object" => "dict".to_string(),
                                "array" => {
                                    if let Some(items) = &prop.items {
                                        format!("List[{}]", parse_property_py(items, schemas))
                                    } else {
                                        "List[Any]".to_string()
                                    }
                                },
                                _ => s.to_string(),
                            };
                            types.push(t);
                        }
                    }
                },
                OneOfVariant::Null => {
                    has_null = true;
                },
            }
        }
        types.sort();
        types.dedup();
        if has_null {
            if types.len() == 1 {
                return format!("Union[None, {}]", types[0]);
            } else if types.is_empty() {
                return "None".to_string();
            } else {
                return format!("Union[None, {}]", types.join(", "));
            }
        } else {
            if types.len() == 1 {
                return types[0].clone();
            } else {
                return format!("Union[{}]", types.join(", "));
            }
        }
    }

    if let Some(serde_json::Value::Array(arr)) = &prop.r#type {
        let mut types: Vec<String> = Vec::new();
        for v in arr {
            if let Some(s) = v.as_str() {
                if s == "null" {
                    types.push("None".to_string());
                } else if s == "array" {
                    if let Some(items) = &prop.items {
                        let item_type = parse_property_py(items, schemas);
                        types.push(format!("List[{}]", item_type));
                    } else {
                        types.push("List[Any]".to_string());
                    }
                } else {
                    types.push(match s {
                        "integer" => "int".to_string(),
                        "string" => "str".to_string(),
                        "boolean" => "bool".to_string(),
                        "number" => "float".to_string(),
                        "object" => "dict".to_string(),
                        _ => s.to_string(),
                    });
                }
            }
        }
        types.sort();
        types.dedup();
        if types.len() == 1 {
            return types[0].clone();
        } else {
            return format!("Union[{}]", types.join(", "));
        }
    }

    if let Some(serde_json::Value::String(s)) = &prop.r#type {
        if s == "null" {
            return "None".to_string();
        }
        if s == "array" {
            if let Some(items) = &prop.items {
                let item_type = parse_property_py(items, schemas);
                return format!("List[{}]", item_type);
            } else {
                return "List[Any]".to_string();
            }
        }
        return match s.as_str() {
            "integer" => "int".to_string(),
            "string" => "str".to_string(),
            "boolean" => "bool".to_string(),
            "number" => "float".to_string(),
            "object" => "dict".to_string(),
            _ => "Any".to_string(),
        };
    }

    "Any".to_string()
}

fn generate_model(name: &str, schema: &Schema, schemas: &HashMap<String, Schema>) -> String {
    let mut fields = Vec::new();
    if let Some(properties) = &schema.properties {
        for (field_name, field_info) in properties {
            let py_type = parse_property_py(field_info, schemas);
            let default_value = if let Some(default) = &field_info.default {
                format!(" = {}", default)
            } else {
                "".to_string()
            };
            
            fields.push(format!("    {}: {}{}", field_name, py_type, default_value));
        }
    }

    if fields.is_empty() {
        fields.push("    pass".to_string());
    }

    let class_str = format!(
        "class {}(BaseModel):\n{}",
        name,
        fields.join("\n")
    );
    class_str
}

fn generate_python_code(schemas: &HashMap<String, Schema>) -> String {
    let mut generated_classes = Vec::new();
    for (name, schema) in schemas {
        generated_classes.push(generate_model(name, schema, schemas));
    }

    let imports = "from typing import Any, List, Union\nfrom pydantic import BaseModel\n\n";
    format!("{}{}", imports, generated_classes.join("\n\n"))
}

fn parse_property_ts(prop: &Property, schemas: &HashMap<String, Schema>) -> String {
    if let Some(r#ref) = &prop.r#ref {
        return r#ref.split('/').last().unwrap().to_string();
    }

    if let Some(one_of) = &prop.one_of {
        let mut types: Vec<String> = Vec::new();
        for variant in one_of {
            match variant {
                OneOfVariant::Ref { r#ref } => {
                    types.push(r#ref.split('/').last().unwrap().to_string());
                },
                OneOfVariant::Object(obj_prop) => {
                    let t = parse_property_ts(obj_prop, schemas);
                    types.push(t);
                },
                OneOfVariant::Type(value) => {
                    if let Some(s) = value.as_str() {
                        types.push(match s {
                            "integer" => "number".to_string(),
                            "string" => "string".to_string(),
                            "boolean" => "boolean".to_string(),
                            "number" => "number".to_string(),
                            "object" => "Record<string, any>".to_string(),
                            _ => "any".to_string(),
                        });
                    }
                },
                OneOfVariant::Null => {
                    types.push("null".to_string());
                },
            }
        }
        types.sort();
        types.dedup();
        return types.join(" | ");
    }

    if let Some(serde_json::Value::String(s)) = &prop.r#type {
        if s == "array" {
            if let Some(items) = &prop.items {
                let item_type = parse_property_ts(items, schemas);
                return format!("Array<{}>", item_type);
            } else {
                return "Array<any>".to_string();
            }
        }
        return match s.as_str() {
            "integer" => "number".to_string(),
            "string" => "string".to_string(),
            "boolean" => "boolean".to_string(),
            "number" => "number".to_string(),
            "object" => "Record<string, any>".to_string(),
            _ => "any".to_string(),
        };
    }
    
    "any".to_string()
}

fn generate_ts_code(schemas: &HashMap<String, Schema>) -> String {
    let mut generated_classes = Vec::new();
    for (name, schema) in schemas {
        let mut fields = Vec::new();
        if let Some(properties) = &schema.properties {
            for (field_name, field_info) in properties {
                let ts_type = parse_property_ts(field_info, schemas);
                fields.push(format!("  {}: {}", field_name, ts_type));
            }
        }

        let class_str = format!(
            "interface {} {{\n{}\n}}",
            name,
            fields.join("\n")
        );
        generated_classes.push(class_str);
    }

    generated_classes.join("\n\n")
}

fn parse_property_rs(prop: &Property, schemas: &HashMap<String, Schema>) -> String {
    if let Some(r#ref) = &prop.r#ref {
        return format!("{} {}", r#ref.split('/').last().unwrap(), r#ref.split('/').last().unwrap());
    }

    if prop.r#type.is_none() {
        return "serde_json::Value".to_string();
    }

    let prop_type = match &prop.r#type {
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Array(arr)) if arr.len() > 0 => arr[0].as_str().unwrap_or_default().to_string(),
        _ => "serde_json::Value".to_string(),
    };

    if prop_type == "array" {
        let item_type = parse_property_rs(&prop.items.as_ref().unwrap(), schemas);
        return format!("Vec<{}>", item_type);
    }

    match prop_type.as_str() {
        "integer" => "i32".to_string(),
        "string" => "String".to_string(),
        "boolean" => "bool".to_string(),
        "number" => "f64".to_string(),
        "object" => "HashMap<String, serde_json::Value>".to_string(),
        _ => prop_type,
    }
}

fn is_rust_reserved_word(word: &str) -> bool {
    let reserved_words = [
        "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
        "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", 
        "loop", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return", "Self", 
        "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "virtual", 
        "where", "while", "yield"
    ];
    reserved_words.contains(&word)
}

fn generate_rust_code(schemas: &HashMap<String, Schema>) -> String {
    let mut generated_structs = Vec::new();

    for (name, schema) in schemas {
        let mut fields = Vec::new();
        if let Some(properties) = &schema.properties {
            for (field_name, field_info) in properties {
                let rust_type = parse_property_rs(field_info, schemas);

                let field_name = if is_rust_reserved_word(field_name) {
                    format!("r#{}", field_name)
                } else {
                    field_name.to_string()
                };

                fields.push(format!("    pub {}: {},", field_name, rust_type));
            }
        }

        let struct_def = format!(
            "#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{\n{}\n}}",
            name,
            fields.join("\n")
        );
        generated_structs.push(struct_def);
    }

    let imports = "#[macro_use]\nextern crate serde;\nuse serde::{Serialize, Deserialize};\n\n";
    format!("{}{}", imports, generated_structs.join("\n\n"))
}

fn parse_property_java(prop: &Property, schemas: &HashMap<String, Schema>) -> String {
    if let Some(r#ref) = &prop.r#ref {
        return format!("{}", r#ref.split('/').last().unwrap());
    }

    if prop.r#type.is_none() {
        return "Object".to_string();
    }

    let prop_type = match &prop.r#type {
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Array(arr)) if arr.len() > 0 => arr[0].as_str().unwrap_or_default().to_string(),
        _ => "Object".to_string(),
    };

    if prop_type == "array" {
        let item_type = parse_property_java(&prop.items.as_ref().unwrap(), schemas);
        return format!("List<{}>", item_type);
    }

    match prop_type.as_str() {
        "integer" => "Integer".to_string(),
        "string" => "String".to_string(),
        "boolean" => "Boolean".to_string(),
        "number" => "Double".to_string(),
        "object" => "Map<String, Object>".to_string(),
        _ => "Object".to_string(),
    }
}

fn generate_java_code(schemas: &HashMap<String, Schema>) -> String {
    let mut generated_classes = Vec::new();

    for (name, schema) in schemas {
        let mut fields = Vec::new();
        let mut getters_setters = Vec::new();

        if let Some(properties) = &schema.properties {
            for (field_name, field_info) in properties {
                let java_type = parse_property_java(field_info, schemas);
                let field_name_upper = field_name[..1].to_uppercase() + &field_name[1..];

                fields.push(format!("    @JsonProperty(\"{}\")\n    private {} {};", field_name, java_type, field_name));

                getters_setters.push(format!(
                    "    public {} get{}() {{ return {}; }}",
                    java_type, field_name_upper, field_name
                ));

                getters_setters.push(format!(
                    "    public void set{}({} {}) {{ this.{} = {}; }}",
                    field_name_upper, java_type, field_name, field_name, field_name
                ));
            }
        }

        let imports = "import com.fasterxml.jackson.annotation.JsonProperty;\nimport java.util.List;\nimport java.util.Map;\n\n";

        let class_def = format!(
            "{}public class {} {{\n{}\n\n{}\n}}",
            imports,
            name,
            fields.join("\n"),
            getters_setters.join("\n")
        );
        generated_classes.push(class_def);
    }

    generated_classes.join("\n\n")
}


#[pyfunction]
fn generate_code(json: &str, language: &str) -> PyResult<String> {
    let openapi_data: OpenApiSchema = serde_json::from_str(json).expect("Unable to parse JSON");

    let schemas = openapi_data
        .components
        .unwrap_or_default()
        .schemas;

    let code = match language.to_lowercase().as_str() {
        "python" | "py" => generate_python_code(&schemas),
        "typescript" | "ts" => generate_ts_code(&schemas),
        "rust" | "rs" => generate_rust_code(&schemas),
        "java" => generate_java_code(&schemas),
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Unsupported language")),
    };
    Ok(code)
}

#[pymodule]
fn code_generator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_code, m)?)?;
    Ok(())
}