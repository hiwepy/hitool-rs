//! 对齐: `cn.hutool.core.util.XmlUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/XmlUtil.java
//!
//! Rust 版本基于 `quick-xml` 提供 DOM 风格 XML 操作。

use std::fs;
use std::io::Cursor;

use indexmap::IndexMap;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{CoreError, Result};

/// XML 文档根节点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlDocument {
    /// 根元素。
    pub root: XmlNode,
}

/// XML 元素节点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlNode {
    /// 标签名。
    pub tag: String,
    /// 属性集合。
    pub attributes: IndexMap<String, String>,
    /// 子节点。
    pub children: Vec<XmlChild>,
}

/// XML 子节点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlChild {
    /// 子元素。
    Element(XmlNode),
    /// 文本内容。
    Text(String),
}

/// 对齐 Java: `cn.hutool.core.util.XmlUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct XmlUtil;

impl XmlUtil {
    // ── XML 验证 ──

    /// 对齐 Java: `XmlUtil.isXml(String)`
    pub fn is_xml(value: &str) -> bool {
        let trimmed = value.trim();
        trimmed.starts_with("<?xml") || trimmed.starts_with("<")
    }

    // ── XML 转义 ──

    /// 对齐 Java: `XmlUtil.escape(char)`
    pub fn escape_char(c: char) -> &'static str {
        match c {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '\'' => "&apos;",
            _ => "",
        }
    }

    /// 对齐 Java: `XmlUtil.escape(String)`
    pub fn escape(value: &str) -> String {
        let mut result = String::with_capacity(value.len());
        for c in value.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&apos;"),
                _ => result.push(c),
            }
        }
        result
    }

    /// 对齐 Java: `XmlUtil.unescape(String)`
    pub fn unescape(value: &str) -> String {
        value
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
    }

    // ── XML 构建 ──

    /// 构建简单的 XML 元素
    pub fn element(tag: &str, content: &str) -> String {
        format!("<{}>{}</{}>", tag, Self::escape(content), tag)
    }

    /// 构建带属性的 XML 元素
    pub fn element_with_attrs(tag: &str, attrs: &[(&str, &str)], content: &str) -> String {
        let attr_str: String = attrs
            .iter()
            .map(|(k, v)| format!(" {}=\"{}\"", k, Self::escape(v)))
            .collect();
        format!("<{}{}>{}</{}>", tag, attr_str, Self::escape(content), tag)
    }

    /// 构建自闭合 XML 元素
    pub fn self_closing_element(tag: &str) -> String {
        format!("<{} />", tag)
    }

    /// 构建带属性的自闭合 XML 元素
    pub fn self_closing_element_with_attrs(tag: &str, attrs: &[(&str, &str)]) -> String {
        let attr_str: String = attrs
            .iter()
            .map(|(k, v)| format!(" {}=\"{}\"", k, Self::escape(v)))
            .collect();
        format!("<{}{} />", tag, attr_str)
    }

    // ── XML 声明 ──

    /// 对齐 Java: `XmlUtil.xmlHeader(String)`
    pub fn xml_header(encoding: &str) -> String {
        format!("<?xml version=\"1.0\" encoding=\"{}\"?>", encoding)
    }

    /// 默认 UTF-8 编码的 XML 声明
    pub fn xml_header_utf8() -> String {
        Self::xml_header("UTF-8")
    }

    // ── CDATA ──

    /// 对齐 Java: `XmlUtil.wrapCDATA(String)`
    pub fn wrap_cdata(content: &str) -> String {
        format!("<![CDATA[{}]]>", content)
    }

    // ── XML 解析辅助 ──

    /// 提取 XML 标签内容
    pub fn get_tag_content(xml: &str, tag: &str) -> Option<String> {
        let start_tag = format!("<{}>", tag);
        let end_tag = format!("</{}>", tag);
        if let Some(start_pos) = xml.find(&start_tag) {
            let content_start = start_pos + start_tag.len();
            if let Some(end_pos) = xml[content_start..].find(&end_tag) {
                return Some(xml[content_start..content_start + end_pos].to_string());
            }
        }
        None
    }

    /// 提取 XML 属性值
    pub fn get_attribute(xml: &str, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(start_pos) = xml.find(&pattern) {
            let value_start = start_pos + pattern.len();
            if let Some(end_pos) = xml[value_start..].find('"') {
                return Some(xml[value_start..value_start + end_pos].to_string());
            }
        }
        None
    }

    /// 对齐 Java: `XmlUtil.parseXml(String)`
    pub fn parse_xml(xml_str: &str) -> Result<XmlDocument> {
        let cleaned = Self::clean_invalid(xml_str);
        if cleaned.trim().is_empty() {
            return Err(CoreError::InvalidArgument {
                name: "xml_str",
                reason: "XML content string is empty",
            });
        }
        Self::read_xml_bytes(cleaned.as_bytes())
    }

    /// 对齐 Java: `XmlUtil.readXML(String)`
    pub fn read_xml(path_or_content: &str) -> Result<XmlDocument> {
        if path_or_content.trim_start().starts_with('<') {
            return Self::parse_xml(path_or_content);
        }
        let content = fs::read_to_string(path_or_content).map_err(CoreError::Io)?;
        Self::parse_xml(&content)
    }

    /// 对齐 Java: `XmlUtil.cleanComment(String)`
    pub fn clean_comment(xml_content: &str) -> String {
        let mut out = String::with_capacity(xml_content.len());
        let mut idx = 0;
        while let Some(start) = xml_content[idx..].find("<!--") {
            let abs = idx + start;
            out.push_str(&xml_content[idx..abs]);
            if let Some(end_rel) = xml_content[abs + 4..].find("-->") {
                idx = abs + 4 + end_rel + 3;
            } else {
                break;
            }
        }
        out.push_str(&xml_content[idx..]);
        out
    }

    /// 对齐 Java: `XmlUtil.cleanInvalid(String)`
    pub fn clean_invalid(xml_content: &str) -> String {
        xml_content
            .chars()
            .filter(|ch| {
                let code = *ch as u32;
                !(code <= 0x08 || code == 0x0B || code == 0x0C || (0x0E..=0x1F).contains(&code))
            })
            .collect()
    }

    /// 对齐 Java: `XmlUtil.elementText(Element, String)`
    pub fn element_text(element: &XmlNode, tag_name: &str) -> Option<String> {
        Self::get_element(element, tag_name).map(|node| node.text_content())
    }

    /// 对齐 Java: `XmlUtil.getElement(Element, String)`
    pub fn get_element<'a>(element: &'a XmlNode, tag_name: &str) -> Option<&'a XmlNode> {
        element.children.iter().find_map(|child| match child {
            XmlChild::Element(node) if node.tag == tag_name => Some(node),
            _ => None,
        })
    }

    /// 对齐 Java: `XmlUtil.getByXPath(String, Object, QName)`
    pub fn get_by_xpath(expression: &str, doc: &XmlDocument) -> Option<String> {
        let path = expression.strip_prefix("//").unwrap_or(expression);
        let segments: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
        let mut current = Some(&doc.root);
        for (index, segment) in segments.iter().enumerate() {
            let local = segment
                .split_once(':')
                .map(|(_, local)| local)
                .unwrap_or(segment);
            if index == 0 && current.is_some_and(|node| node.tag == local) {
                continue;
            }
            current = current.and_then(|node| Self::find_child_element(node, local));
        }
        current.map(|node| node.text_content())
    }

    /// 对齐 Java: `XmlUtil.xmlToMap(String)`
    pub fn xml_to_map(xml_str: &str) -> Result<IndexMap<String, Value>> {
        let doc = Self::parse_xml(xml_str)?;
        Ok(Self::xml_node_to_map(&doc.root))
    }

    /// 对齐 Java: `XmlUtil.mapToXml(Map, String)`
    pub fn map_to_xml(data: &IndexMap<String, Value>, root_name: &str) -> Result<XmlDocument> {
        let mut root = XmlNode {
            tag: root_name.to_string(),
            attributes: IndexMap::new(),
            children: Vec::new(),
        };
        Self::append_map(&mut root, data);
        Ok(XmlDocument { root })
    }

    /// 对齐 Java: `XmlUtil.mapToXmlStr(Map, boolean)`
    pub fn map_to_xml_str(data: &IndexMap<String, Value>, omit_xml_declaration: bool) -> Result<String> {
        let doc = Self::map_to_xml(data, "xml")?;
        Ok(Self::to_str(&doc, false, omit_xml_declaration))
    }

    /// 对齐 Java: `XmlUtil.toStr(Document, boolean)`
    pub fn to_str(doc: &XmlDocument, is_pretty: bool, omit_xml_declaration: bool) -> String {
        let mut body = String::new();
        Self::write_node(&mut body, &doc.root, 0, is_pretty);
        if omit_xml_declaration {
            body
        } else {
            format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>{body}")
        }
    }

    /// 对齐 Java: `XmlUtil.format(Document)`
    pub fn format(doc: &XmlDocument) -> String {
        Self::to_str(doc, true, false)
    }

    /// 对齐 Java: `XmlUtil.beanToXml(Object, String, boolean)`
    pub fn bean_to_xml(
        bean: &impl Serialize,
        root_name: &str,
        namespace: Option<&str>,
        ignore_null: bool,
    ) -> Result<XmlDocument> {
        let value = serde_json::to_value(bean).map_err(|err| CoreError::Codec(err.to_string()))?;
        let map = Self::value_to_map(value, ignore_null);
        let mut doc = Self::map_to_xml(&map, root_name)?;
        if let Some(ns) = namespace {
            doc.root.attributes.insert("xmlns".to_string(), ns.to_string());
        }
        Ok(doc)
    }

    /// 对齐 Java: `XmlUtil.xmlToBean(Node, Class)`
    pub fn xml_to_bean<T: DeserializeOwned>(doc: &XmlDocument) -> Result<T> {
        let map = Self::xml_node_to_map(&doc.root);
        let json_map: Map<String, Value> = map.into_iter().collect();
        serde_json::from_value(Value::Object(json_map)).map_err(|err| CoreError::Codec(err.to_string()))
    }

    /// 对齐 Java: `XmlUtil.readBySax(InputStream, ContentHandler)`
    pub fn read_by_sax(xml: &str, mut handler: impl FnMut(&str)) -> Result<()> {
        let mut reader = Reader::from_reader(Cursor::new(xml.as_bytes()));
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(event)) => {
                    let name = local_name(&event);
                    handler(&name);
                }
                Ok(Event::Empty(event)) => {
                    let name = local_name(&event);
                    handler(&name);
                }
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(err) => return Err(CoreError::Codec(err.to_string())),
            }
            buf.clear();
        }
        Ok(())
    }

    fn read_xml_bytes(bytes: &[u8]) -> Result<XmlDocument> {
        let mut reader = Reader::from_reader(Cursor::new(bytes));
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let root = loop {
            let event = match reader.read_event_into(&mut buf) {
                Ok(Event::Start(event)) => Some(event),
                Ok(Event::Empty(event)) => {
                    break XmlNode {
                        tag: local_name(&event),
                        attributes: read_attributes(&event),
                        children: Vec::new(),
                    }
                }
                Ok(Event::Eof) => {
                    return Err(CoreError::InvalidArgument {
                        name: "xml",
                        reason: "missing root element",
                    });
                }
                Ok(_) => None,
                Err(err) => return Err(CoreError::Codec(err.to_string())),
            };
            if let Some(start) = event {
                break Self::read_element(&mut reader, start)?;
            }
            buf.clear();
        };
        Ok(XmlDocument { root })
    }

    fn read_element(
        reader: &mut Reader<Cursor<&[u8]>>,
        start: BytesStart,
    ) -> Result<XmlNode> {
        let tag = local_name(&start);
        let attributes = read_attributes(&start);
        let mut children = Vec::new();
        let mut buf = Vec::new();
        loop {
            let event = match reader.read_event_into(&mut buf) {
                Ok(event) => event,
                Err(err) => return Err(CoreError::Codec(err.to_string())),
            };
            match event {
                Event::Start(event) => {
                    children.push(XmlChild::Element(Self::read_element(reader, event)?));
                }
                Event::Empty(event) => children.push(XmlChild::Element(XmlNode {
                    tag: local_name(&event),
                    attributes: read_attributes(&event),
                    children: Vec::new(),
                })),
                Event::Text(text) => {
                    let value = String::from_utf8_lossy(text.as_ref()).into_owned();
                    if !value.is_empty() {
                        children.push(XmlChild::Text(value));
                    }
                }
                Event::CData(text) => {
                    children.push(XmlChild::Text(String::from_utf8_lossy(text.as_ref()).into_owned()));
                }
                Event::End(end) if local_name_end(&end) == tag => break,
                Event::End(_) => {
                    return Err(CoreError::InvalidArgument {
                        name: "xml",
                        reason: "mismatched end tag",
                    });
                }
                Event::Eof => {
                    return Err(CoreError::InvalidArgument {
                        name: "xml",
                        reason: "unexpected EOF",
                    });
                }
                Event::Decl(_) | Event::PI(_) | Event::DocType(_) | Event::Comment(_) | Event::GeneralRef(_) => {}
            }
            buf.clear();
        }
        Ok(XmlNode { tag, attributes, children })
    }

    fn find_child_element<'a>(node: &'a XmlNode, tag_name: &str) -> Option<&'a XmlNode> {
        node.children.iter().find_map(|child| match child {
            XmlChild::Element(child_node) if child_node.tag == tag_name => Some(child_node),
            _ => None,
        })
    }

    fn xml_node_to_map(node: &XmlNode) -> IndexMap<String, Value> {
        let mut result = IndexMap::new();
        for child in &node.children {
            if let XmlChild::Element(element) = child {
                let key = element.tag.clone();
                let value = if element.children.iter().all(|item| matches!(item, XmlChild::Text(_))) {
                    Value::String(element.text_content())
                } else {
                    let nested = Self::xml_node_to_map(element);
                    if nested.is_empty() {
                        Value::String(element.text_content())
                    } else {
                        Value::Object(nested.into_iter().collect())
                    }
                };
                Self::insert_map_value(&mut result, key, value);
            }
        }
        result
    }

    fn insert_map_value(map: &mut IndexMap<String, Value>, key: String, value: Value) {
        if let Some(existing) = map.get(&key).cloned() {
            match existing {
                Value::Array(mut items) => {
                    items.push(value);
                    map.insert(key, Value::Array(items));
                }
                other => {
                    map.insert(key, Value::Array(vec![other, value]));
                }
            }
        } else {
            map.insert(key, value);
        }
    }

    fn append_map(node: &mut XmlNode, data: &IndexMap<String, Value>) {
        for (key, value) in data {
            Self::append_value(node, key, value);
        }
    }

    fn append_value(parent: &mut XmlNode, key: &str, value: &Value) {
        match value {
            Value::Null => parent.children.push(XmlChild::Element(XmlNode {
                tag: key.to_string(),
                attributes: IndexMap::new(),
                children: Vec::new(),
            })),
            Value::Bool(v) => parent.children.push(XmlChild::Element(XmlNode {
                tag: key.to_string(),
                attributes: IndexMap::new(),
                children: vec![XmlChild::Text(v.to_string())],
            })),
            Value::Number(v) => parent.children.push(XmlChild::Element(XmlNode {
                tag: key.to_string(),
                attributes: IndexMap::new(),
                children: vec![XmlChild::Text(v.to_string())],
            })),
            Value::String(v) => parent.children.push(XmlChild::Element(XmlNode {
                tag: key.to_string(),
                attributes: IndexMap::new(),
                children: vec![XmlChild::Text(v.clone())],
            })),
            Value::Array(items) => {
                for item in items {
                    Self::append_value(parent, key, item);
                }
            }
            Value::Object(map) => {
                let mut child = XmlNode {
                    tag: key.to_string(),
                    attributes: IndexMap::new(),
                    children: Vec::new(),
                };
                let ordered: IndexMap<String, Value> =
                    map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                Self::append_map(&mut child, &ordered);
                parent.children.push(XmlChild::Element(child));
            }
        }
    }

    fn write_node(out: &mut String, node: &XmlNode, depth: usize, pretty: bool) {
        if pretty {
            out.push('\n');
            out.push_str(&"  ".repeat(depth));
        }
        out.push('<');
        out.push_str(&node.tag);
        for (key, value) in &node.attributes {
            out.push(' ');
            out.push_str(key);
            out.push('=');
            out.push('"');
            out.push_str(&Self::escape(value));
            out.push('"');
        }
        if node.children.is_empty() {
            out.push_str(" />");
            return;
        }
        out.push('>');
        let only_text = node.children.iter().all(|child| matches!(child, XmlChild::Text(_)));
        if only_text {
            for child in &node.children {
                if let XmlChild::Text(text) = child {
                    out.push_str(&Self::escape(text));
                }
            }
        } else {
            for child in &node.children {
                match child {
                    XmlChild::Text(text) => out.push_str(&Self::escape(text)),
                    XmlChild::Element(element) => Self::write_node(out, element, depth + 1, pretty),
                }
            }
            if pretty {
                out.push('\n');
                out.push_str(&"  ".repeat(depth));
            }
        }
        out.push('<');
        out.push('/');
        out.push_str(&node.tag);
        out.push('>');
    }

    fn value_to_map(value: Value, ignore_null: bool) -> IndexMap<String, Value> {
        match value {
            Value::Object(map) => map
                .into_iter()
                .filter(|(_, v)| !(ignore_null && v.is_null()))
                .collect(),
            _ => IndexMap::new(),
        }
    }
}

impl XmlNode {
    /// 返回当前节点全部文本内容。
    pub fn text_content(&self) -> String {
        self.children
            .iter()
            .filter_map(|child| match child {
                XmlChild::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// 读取属性值。
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(String::as_str)
    }
}

fn local_name(event: &BytesStart) -> String {
    String::from_utf8_lossy(event.name().local_name().as_ref()).into_owned()
}

fn local_name_end(event: &BytesEnd) -> String {
    String::from_utf8_lossy(event.name().local_name().as_ref()).into_owned()
}

fn read_attributes(event: &BytesStart) -> IndexMap<String, String> {
    let mut attrs = IndexMap::new();
    for attr in event.attributes().flatten() {
        attrs.insert(
            String::from_utf8_lossy(attr.key.local_name().as_ref()).into_owned(),
            attr.unescape_value().unwrap_or_default().into_owned(),
        );
    }
    attrs
}
