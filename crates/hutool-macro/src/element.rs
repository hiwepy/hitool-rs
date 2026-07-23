//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::mirror::{AnnotationMirror, AnnotationSchema, AnnotationTypeName, AnnotationValue};

pub use super::mirror::ElementHandle;

/// 元素种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Type,
    Method,
    Field,
}

/// 类型元素。
#[derive(Debug, Clone)]
pub struct TypeElement {
    pub handle: ElementHandle,
    pub name: String,
    pub annotations: Vec<Arc<AnnotationMirror>>,
    pub super_type: Option<ElementHandle>,
    pub interfaces: Vec<ElementHandle>,
    pub methods: HashMap<String, ElementHandle>,
    pub fields: HashMap<String, ElementHandle>,
}

/// 方法元素。
#[derive(Debug, Clone)]
pub struct MethodElement {
    pub handle: ElementHandle,
    pub name: String,
    pub declaring_type: ElementHandle,
    pub annotations: Vec<Arc<AnnotationMirror>>,
    pub signature: String,
}

/// 字段元素。
#[derive(Debug, Clone)]
pub struct FieldElement {
    pub handle: ElementHandle,
    pub name: String,
    pub declaring_type: ElementHandle,
    pub annotations: Vec<Arc<AnnotationMirror>>,
}

/// 被注解元素枚举。
#[derive(Debug, Clone)]
pub enum AnnotatedElement {
    Type(TypeElement),
    Method(MethodElement),
    Field(FieldElement),
}

impl AnnotatedElement {
    /// 元素句柄。
    pub fn handle(&self) -> ElementHandle {
        match self {
            Self::Type(e) => e.handle,
            Self::Method(e) => e.handle,
            Self::Field(e) => e.handle,
        }
    }

    /// 直接声明的注解。
    pub fn declared_annotations(&self) -> &[Arc<AnnotationMirror>] {
        match self {
            Self::Type(e) => &e.annotations,
            Self::Method(e) => &e.annotations,
            Self::Field(e) => &e.annotations,
        }
    }

    /// 元素种类。
    pub fn kind(&self) -> ElementKind {
        match self {
            Self::Type(_) => ElementKind::Type,
            Self::Method(_) => ElementKind::Method,
            Self::Field(_) => ElementKind::Field,
        }
    }
}

/// 全局注解/元素注册表。
#[derive(Default)]
pub struct AnnotationRegistry {
    next_id: u64,
    schemas: HashMap<AnnotationTypeName, AnnotationSchema>,
    elements: HashMap<ElementHandle, AnnotatedElement>,
}

impl AnnotationRegistry {
    /// 创建空注册表。
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册注解 schema。
    pub fn register_schema(&mut self, schema: AnnotationSchema) {
        self.schemas.insert(schema.type_name, schema);
    }

    /// 获取 schema。
    pub fn schema(&self, type_name: AnnotationTypeName) -> Option<&AnnotationSchema> {
        self.schemas.get(type_name)
    }

    fn next_handle(&mut self) -> ElementHandle {
        self.next_id += 1;
        ElementHandle::new(self.next_id)
    }

    /// 构造注解镜像。
    pub fn annotation(
        &self,
        type_name: AnnotationTypeName,
        values: HashMap<String, AnnotationValue>,
    ) -> Arc<AnnotationMirror> {
        Arc::new(AnnotationMirror::new(type_name, values))
    }

    /// 注册类型元素。
    pub fn register_type(&mut self, element: TypeElement) -> ElementHandle {
        let handle = element.handle;
        self.elements.insert(handle, AnnotatedElement::Type(element));
        handle
    }

    /// 注册方法元素。
    pub fn register_method(&mut self, element: MethodElement) -> ElementHandle {
        let handle = element.handle;
        self.elements.insert(handle, AnnotatedElement::Method(element));
        handle
    }

    /// 注册字段元素。
    pub fn register_field(&mut self, element: FieldElement) -> ElementHandle {
        let handle = element.handle;
        self.elements.insert(handle, AnnotatedElement::Field(element));
        handle
    }

    /// 获取元素。
    pub fn get(&self, handle: ElementHandle) -> Option<&AnnotatedElement> {
        self.elements.get(&handle)
    }

    /// 获取可变类型元素。
    pub fn get_type_mut(&mut self, handle: ElementHandle) -> Option<&mut TypeElement> {
        match self.elements.get_mut(&handle) {
            Some(AnnotatedElement::Type(t)) => Some(t),
            _ => None,
        }
    }

    /// 解析类型层级（广度优先）：返回 (distance, type_handle) 列表。
    pub fn type_hierarchy(&self, start: ElementHandle) -> Vec<(i32, ElementHandle)> {
        let mut result = Vec::new();
        let mut visited = HashMap::new();
        let mut queue = vec![(0i32, start)];

        while let Some((distance, handle)) = queue.first().cloned() {
            queue.remove(0);
            if visited.contains_key(&handle) {
                continue;
            }
            visited.insert(handle, distance);
            result.push((distance, handle));

            let Some(AnnotatedElement::Type(ty)) = self.get(handle) else {
                continue;
            };

            let mut next_level = Vec::new();
            if let Some(super_type) = ty.super_type {
                next_level.push(super_type);
            }
            next_level.extend(ty.interfaces.iter().copied());

            if !next_level.is_empty() {
                queue.push((distance + 1, next_level[0]));
                for item in next_level.into_iter().skip(1) {
                    if !visited.contains_key(&item) {
                        queue.push((distance + 1, item));
                    }
                }
            }
        }
        result
    }

    /// 解析类型名（测试注册表内必须为 static 名）。
    pub fn resolve_type_name(&self, name: &str) -> Option<AnnotationTypeName> {
        self.schemas.keys().find(|k| **k == name).copied()
    }

    /// 查找同名方法链（沿类型层级）。
    pub fn method_override_chain(&self, method: ElementHandle) -> Vec<ElementHandle> {
        let AnnotatedElement::Method(m) = self.get(method).cloned().expect("method") else {
            return vec![method];
        };
        let mut chain = vec![method];
        for (_, type_handle) in self.type_hierarchy(m.declaring_type).into_iter().skip(1) {
            if let Some(AnnotatedElement::Type(ty)) = self.get(type_handle) {
                if let Some(mh) = ty.methods.get(&m.name) {
                    chain.push(*mh);
                }
            }
        }
        chain
    }
}

/// 全局注册表访问。
static GLOBAL_REGISTRY: std::sync::OnceLock<RwLock<AnnotationRegistry>> = std::sync::OnceLock::new();

/// 获取全局注册表。
pub fn global_registry() -> &'static RwLock<AnnotationRegistry> {
    GLOBAL_REGISTRY.get_or_init(|| RwLock::new(AnnotationRegistry::new()))
}

/// 类型 builder。
pub struct TypeBuilder<'a> {
    registry: &'a mut AnnotationRegistry,
    handle: ElementHandle,
    name: String,
    annotations: Vec<Arc<AnnotationMirror>>,
    super_type: Option<ElementHandle>,
    interfaces: Vec<ElementHandle>,
}

impl<'a> TypeBuilder<'a> {
    /// 开始构建类型。
    pub fn begin(registry: &'a mut AnnotationRegistry, name: impl Into<String>) -> Self {
        let handle = registry.next_handle();
        Self {
            registry,
            handle,
            name: name.into(),
            annotations: Vec::new(),
            super_type: None,
            interfaces: Vec::new(),
        }
    }

    /// 添加注解。
    pub fn annotate(mut self, annotation: Arc<AnnotationMirror>) -> Self {
        self.annotations.push(annotation);
        self
    }

    /// 设置父类。
    pub fn super_type(mut self, handle: ElementHandle) -> Self {
        self.super_type = Some(handle);
        self
    }

    /// 添加父接口。
    pub fn interface(mut self, handle: ElementHandle) -> Self {
        self.interfaces.push(handle);
        self
    }

    /// 完成构建。
    pub fn build(self) -> ElementHandle {
        let element = TypeElement {
            handle: self.handle,
            name: self.name,
            annotations: self.annotations,
            super_type: self.super_type,
            interfaces: self.interfaces,
            methods: HashMap::new(),
            fields: HashMap::new(),
        };
        self.registry.register_type(element)
    }
}

/// 方法 builder。
pub struct MethodBuilder<'a> {
    registry: &'a mut AnnotationRegistry,
    handle: ElementHandle,
    name: String,
    declaring_type: ElementHandle,
    annotations: Vec<Arc<AnnotationMirror>>,
}

impl<'a> MethodBuilder<'a> {
    /// 开始构建方法。
    pub fn begin(
        registry: &'a mut AnnotationRegistry,
        declaring_type: ElementHandle,
        name: impl Into<String>,
    ) -> Self {
        let handle = registry.next_handle();
        Self {
            registry,
            handle,
            name: name.into(),
            declaring_type,
            annotations: Vec::new(),
        }
    }

    /// 添加注解。
    pub fn annotate(mut self, annotation: Arc<AnnotationMirror>) -> Self {
        self.annotations.push(annotation);
        self
    }

    /// 完成构建并挂到声明类。
    pub fn build(self) -> ElementHandle {
        let signature = format!("{}#{}", self.declaring_type.id(), self.name);
        let element = MethodElement {
            handle: self.handle,
            name: self.name.clone(),
            declaring_type: self.declaring_type,
            annotations: self.annotations,
            signature,
        };
        let handle = self.registry.register_method(element);
        if let Some(ty) = self.registry.get_type_mut(self.declaring_type) {
            ty.methods.insert(self.name, handle);
        }
        handle
    }
}

/// 字段 builder。
pub struct FieldBuilder<'a> {
    registry: &'a mut AnnotationRegistry,
    handle: ElementHandle,
    name: String,
    declaring_type: ElementHandle,
    annotations: Vec<Arc<AnnotationMirror>>,
}

impl<'a> FieldBuilder<'a> {
    /// 开始构建字段。
    pub fn begin(
        registry: &'a mut AnnotationRegistry,
        declaring_type: ElementHandle,
        name: impl Into<String>,
    ) -> Self {
        let handle = registry.next_handle();
        Self {
            registry,
            handle,
            name: name.into(),
            declaring_type,
            annotations: Vec::new(),
        }
    }

    /// 添加注解。
    pub fn annotate(mut self, annotation: Arc<AnnotationMirror>) -> Self {
        self.annotations.push(annotation);
        self
    }

    /// 完成构建。
    pub fn build(self) -> ElementHandle {
        let element = FieldElement {
            handle: self.handle,
            name: self.name.clone(),
            declaring_type: self.declaring_type,
            annotations: self.annotations,
        };
        let handle = self.registry.register_field(element);
        if let Some(ty) = self.registry.get_type_mut(self.declaring_type) {
            ty.fields.insert(self.name, handle);
        }
        handle
    }
}
