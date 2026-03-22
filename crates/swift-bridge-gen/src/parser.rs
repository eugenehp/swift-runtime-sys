//! Parse the swift-api-digester JSON dump into our IR.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Struct,
    Class,
    Enum,
    Protocol,
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub module: String,
    pub mangled_name: Option<String>,
    pub constructors: Vec<ConstructorInfo>,
    pub methods: Vec<MethodInfo>,
    pub properties: Vec<PropertyInfo>,
}

#[derive(Debug, Clone)]
pub struct ConstructorInfo {
    pub printed_name: String,
    pub mangled_name: Option<String>,
    pub params: Vec<ParamInfo>,
    pub return_type: SwiftType,
    pub is_failable: bool,
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub printed_name: String,
    pub mangled_name: Option<String>,
    pub params: Vec<ParamInfo>,
    pub return_type: SwiftType,
    pub is_static: bool,
    pub is_mutating: bool,
}

#[derive(Debug, Clone)]
pub struct PropertyInfo {
    pub name: String,
    pub swift_type: SwiftType,
    pub has_getter: bool,
    pub has_setter: bool,
    pub is_static: bool,
}

#[derive(Debug, Clone)]
pub struct ParamInfo {
    pub label: Option<String>,
    pub swift_type: SwiftType,
}

#[derive(Debug, Clone)]
pub struct SwiftType {
    pub name: String,
    pub printed_name: String,
    pub is_optional: bool,
    pub generic_args: Vec<SwiftType>,
}

pub struct ApiDump {
    pub module_name: String,
    pub types: Vec<TypeInfo>,
}

// ── JSON deserialization ──

#[derive(Deserialize)]
struct JsonRoot {
    #[serde(rename = "ABIRoot")]
    abi_root: JsonNode,
}

#[derive(Deserialize, Clone)]
struct JsonNode {
    kind: Option<String>,
    name: Option<String>,
    #[serde(rename = "printedName")]
    printed_name: Option<String>,
    #[serde(rename = "declKind")]
    decl_kind: Option<String>,
    #[serde(rename = "mangledName")]
    mangled_name: Option<String>,
    #[serde(rename = "moduleName")]
    module_name: Option<String>,
    children: Option<Vec<JsonNode>>,
    #[serde(rename = "declAttributes")]
    decl_attributes: Option<Vec<String>>,
    #[serde(rename = "hasDefaultArg")]
    has_default_arg: Option<bool>,
}

pub fn parse_api_dump(json_str: &str) -> Result<ApiDump, String> {
    let root: JsonRoot =
        serde_json::from_str(json_str).map_err(|e| format!("JSON parse error: {e}"))?;

    let abi = &root.abi_root;
    let module_name = abi.name.clone().unwrap_or_else(|| "Unknown".to_string());

    let mut types = Vec::new();

    for child in abi.children.as_deref().unwrap_or_default() {
        if child.kind.as_deref() != Some("TypeDecl") {
            continue;
        }
        if let Some(ti) = parse_type_decl(child, &module_name) {
            types.push(ti);
        }
    }

    Ok(ApiDump { module_name, types })
}

fn parse_type_decl(node: &JsonNode, module: &str) -> Option<TypeInfo> {
    let name = node.name.as_ref()?.clone();
    let decl_kind = node.decl_kind.as_deref()?;

    let kind = match decl_kind {
        "Struct" => TypeKind::Struct,
        "Class" => TypeKind::Class,
        "Enum" => TypeKind::Enum,
        "Protocol" => TypeKind::Protocol,
        _ => return None,
    };

    // Skip private/internal types
    if name.starts_with('_') {
        return None;
    }

    let children = node.children.as_deref().unwrap_or_default();

    let mut constructors = Vec::new();
    let mut methods = Vec::new();
    let mut properties = Vec::new();

    for child in children {
        match child.decl_kind.as_deref() {
            Some("Constructor") => {
                if let Some(c) = parse_constructor(child) {
                    constructors.push(c);
                }
            }
            Some("Func") => {
                if let Some(m) = parse_method(child) {
                    methods.push(m);
                }
            }
            Some("Var") => {
                if let Some(p) = parse_property(child) {
                    properties.push(p);
                }
            }
            _ => {}
        }
        // Also check kind == "Function" (some nodes use this)
        if child.kind.as_deref() == Some("Function") && child.decl_kind.is_none() {
            if let Some(m) = parse_method(child) {
                methods.push(m);
            }
        }
    }

    Some(TypeInfo {
        name,
        kind,
        module: module.to_string(),
        mangled_name: node.mangled_name.clone(),
        constructors,
        methods,
        properties,
    })
}

fn parse_constructor(node: &JsonNode) -> Option<ConstructorInfo> {
    let printed_name = node.printed_name.as_ref()?.clone();
    let children = node.children.as_deref().unwrap_or_default();

    // First child is the return type, rest are params
    if children.is_empty() {
        return None;
    }

    let return_type = parse_swift_type(&children[0]);
    let is_failable = return_type.is_optional;

    let params: Vec<ParamInfo> = children[1..]
        .iter()
        .map(|c| ParamInfo {
            label: c.name.clone(),
            swift_type: parse_swift_type(c),
        })
        .collect();

    Some(ConstructorInfo {
        printed_name,
        mangled_name: node.mangled_name.clone(),
        params,
        return_type,
        is_failable,
    })
}

fn parse_method(node: &JsonNode) -> Option<MethodInfo> {
    let name = node.name.as_ref()?.clone();
    let printed_name = node.printed_name.as_ref()?.clone();

    // Skip operators and special methods
    if name.starts_with("__") || name.contains("operator") {
        return None;
    }

    let children = node.children.as_deref().unwrap_or_default();
    if children.is_empty() {
        return None;
    }

    let return_type = parse_swift_type(&children[0]);
    let params: Vec<ParamInfo> = children[1..]
        .iter()
        .map(|c| ParamInfo {
            label: c.name.clone(),
            swift_type: parse_swift_type(c),
        })
        .collect();

    let attrs = node.decl_attributes.as_deref().unwrap_or_default();
    let is_static = attrs.contains(&"Final".to_string());
    let is_mutating = attrs.contains(&"Mutating".to_string());

    Some(MethodInfo {
        name,
        printed_name,
        mangled_name: node.mangled_name.clone(),
        params,
        return_type,
        is_static,
        is_mutating,
    })
}

fn parse_property(node: &JsonNode) -> Option<PropertyInfo> {
    let name = node.name.as_ref()?.clone();
    if name.starts_with('_') {
        return None;
    }

    let children = node.children.as_deref().unwrap_or_default();
    let swift_type = if !children.is_empty() {
        parse_swift_type(&children[0])
    } else {
        SwiftType {
            name: "Any".to_string(),
            printed_name: "Any".to_string(),
            is_optional: false,
            generic_args: vec![],
        }
    };

    let mut has_getter = false;
    let mut has_setter = false;
    for child in children {
        match child.decl_kind.as_deref() {
            Some("Accessor") => match child.name.as_deref() {
                Some("Get") => has_getter = true,
                Some("Set") => has_setter = true,
                _ => {}
            },
            _ => {}
        }
    }
    if !has_getter {
        has_getter = true;
    } // assume readable

    let attrs = node.decl_attributes.as_deref().unwrap_or_default();
    let is_static = attrs.contains(&"Final".to_string());

    Some(PropertyInfo {
        name,
        swift_type,
        has_getter,
        has_setter,
        is_static,
    })
}

fn parse_swift_type(node: &JsonNode) -> SwiftType {
    let name = node.name.clone().unwrap_or_else(|| "Void".to_string());
    let printed_name = node.printed_name.clone().unwrap_or_else(|| name.clone());
    let is_optional = name == "Optional";
    let children = node.children.as_deref().unwrap_or_default();
    let generic_args: Vec<SwiftType> = children
        .iter()
        .filter(|c| c.kind.as_deref() == Some("TypeNominal"))
        .map(|c| parse_swift_type(c))
        .collect();

    SwiftType {
        name,
        printed_name,
        is_optional,
        generic_args,
    }
}
