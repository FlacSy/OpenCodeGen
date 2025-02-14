use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Deserialize, Serialize)]
struct Property {
    r#type: Option<serde_json::Value>,
    items: Option<Box<Property>>,
    r#ref: Option<String>,
}

fn parse_property(prop: &Property, schemas: &HashMap<String, Schema>) -> String {
    if let Some(r#ref) = &prop.r#ref {
        return format!("'{}'", r#ref.split('/').last().unwrap());
    }

    if prop.r#type.is_none() {
        return "Any".to_string();
    }

    let prop_type = match &prop.r#type {
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Array(arr)) if arr.len() > 0 => arr[0].as_str().unwrap_or_default().to_string(),
        _ => "Any".to_string(),
    };

    if prop_type == "array" {
        let item_type = parse_property(&prop.items.as_ref().unwrap(), schemas);
        return format!("List[{}]", item_type);
    }

    match prop_type.as_str() {
        "integer" => "int".to_string(),
        "string" => "str".to_string(),
        "boolean" => "bool".to_string(),
        "number" => "float".to_string(),
        "object" => "dict".to_string(),
        _ => "Any".to_string(),
    }
}

fn generate_model(name: &str, schema: &Schema, schemas: &HashMap<String, Schema>) -> String {
    let mut fields = Vec::new();
    if let Some(properties) = &schema.properties {
        for (field_name, field_info) in properties {
            let py_type = parse_property(field_info, schemas);
            fields.push(format!("    {}: {}", field_name, py_type));
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

    let imports = "from typing import Any, List\nfrom pydantic import BaseModel\n\n";
    format!("{}{}", imports, generated_classes.join("\n\n"))
}

#[pyfunction]
fn parse_openapi(json: &str) -> PyResult<String> {
    let openapi_data: OpenApiSchema = serde_json::from_str(json).expect("Unable to parse JSON");

    let schemas = openapi_data
        .components
        .unwrap_or_default()
        .schemas;

    let python_code = generate_python_code(&schemas);

    Ok(python_code)
}

#[pymodule]
fn code_generator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_openapi, m)?)?;
    Ok(())
}
