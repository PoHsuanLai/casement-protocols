//! The XML files as data, for the introspection tests: a reader over the subset of the
//! Wayland protocol schema the six files use.

pub mod frozen;

use std::path::PathBuf;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};

/// The six protocol files, by file name.
pub const FILES: &[&str] = &[
    "quire-desktop-v1.xml",
    "quire-shell-v1.xml",
    "quire-window-v1.xml",
    "quire-sensitive-v1.xml",
    "quire-agent-v1.xml",
    "quire-trusted-surface-v1.xml",
];

/// One `<protocol>`.
#[derive(Debug, Default)]
pub struct Protocol {
    pub file: String,
    pub name: String,
    pub copyright: String,
    pub interfaces: Vec<Interface>,
}

/// One `<interface>`.
#[derive(Debug, Default)]
pub struct Interface {
    pub name: String,
    pub version: String,
    pub requests: Vec<Message>,
    pub events: Vec<Message>,
    pub enums: Vec<XmlEnum>,
}

/// One `<request>` or `<event>`; `kind` is its `type` attribute.
#[derive(Debug, Default)]
pub struct Message {
    pub name: String,
    pub kind: Option<String>,
    pub args: Vec<Arg>,
}

/// One `<arg>`, attributes as written.
#[derive(Debug, Default)]
pub struct Arg {
    pub name: String,
    pub kind: String,
    pub interface: Option<String>,
    pub enumeration: Option<String>,
    pub allow_null: Option<String>,
}

/// One `<enum>`: its entries as (name, value as written).
#[derive(Debug, Default)]
pub struct XmlEnum {
    pub name: String,
    pub bitfield: Option<String>,
    pub entries: Vec<(String, String)>,
}

impl Message {
    /// `name(arg: type, ...)`, with `new_id IFACE`, `IFACE` or `IFACE?` for objects,
    /// `uint<enum>` for enum arguments, and ` [destructor]` for a destructor.
    pub fn signature(&self) -> String {
        let args: Vec<String> = self.args.iter().map(Arg::signature).collect();
        let tail = match self.kind.as_deref() {
            Some(kind) => format!(" [{kind}]"),
            None => String::new(),
        };
        format!("{}({}){tail}", self.name, args.join(", "))
    }
}

impl Arg {
    fn signature(&self) -> String {
        let null = if self.allow_null.as_deref() == Some("true") {
            "?"
        } else {
            ""
        };
        let kind = match (self.kind.as_str(), &self.interface, &self.enumeration) {
            ("new_id", Some(iface), _) => format!("new_id {iface}"),
            ("object", Some(iface), _) => format!("{iface}{null}"),
            (kind, _, Some(enumeration)) => format!("{kind}<{enumeration}>"),
            (kind, _, None) => format!("{kind}{null}"),
        };
        format!("{}: {kind}", self.name)
    }
}

/// The directory holding the XML files.
pub fn protocols_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("protocols")
}

/// Every protocol file, parsed.
pub fn protocols() -> Vec<Protocol> {
    FILES.iter().map(|file| parse(file)).collect()
}

/// One protocol file, parsed. Panics with the file's name on malformed XML.
pub fn parse(file: &str) -> Protocol {
    let text = std::fs::read_to_string(protocols_dir().join(file))
        .unwrap_or_else(|error| panic!("{file}: {error}"));
    let mut reader = Reader::from_str(&text);
    reader.config_mut().trim_text(true);
    reader.config_mut().expand_empty_elements = true;

    let mut protocol = Protocol {
        file: file.to_owned(),
        ..Protocol::default()
    };
    let mut stack: Vec<String> = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(element)) => {
                let tag = String::from_utf8_lossy(element.name().as_ref()).into_owned();
                open(&mut protocol, &stack, &tag, &element);
                stack.push(tag);
            }
            Ok(Event::End(_)) => {
                stack.pop();
            }
            Ok(Event::Text(text)) if stack.last().map(String::as_str) == Some("copyright") => {
                let text = text
                    .decode()
                    .unwrap_or_else(|error| panic!("{file}: {error}"));
                protocol.copyright.push_str(&text);
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(error) => panic!("{file}: {error}"),
        }
    }
    assert!(stack.is_empty(), "{file}: unclosed elements {stack:?}");
    protocol
}

/// Records an opening tag in the protocol being built.
fn open(protocol: &mut Protocol, stack: &[String], tag: &str, element: &BytesStart) {
    let attr = |key: &str| attribute(element, key);
    let parent = stack.last().map(String::as_str);
    match (tag, parent) {
        ("protocol", None) => protocol.name = attr("name").unwrap_or_default(),
        ("interface", Some("protocol")) => protocol.interfaces.push(Interface {
            name: attr("name").unwrap_or_default(),
            version: attr("version").unwrap_or_default(),
            ..Interface::default()
        }),
        ("request" | "event", Some("interface")) => {
            let message = Message {
                name: attr("name").unwrap_or_default(),
                kind: attr("type"),
                args: Vec::new(),
            };
            let interface = last_interface(protocol);
            match tag {
                "request" => interface.requests.push(message),
                _ => interface.events.push(message),
            }
        }
        ("arg", Some(owner @ ("request" | "event"))) => {
            let interface = last_interface(protocol);
            let messages = match owner {
                "request" => &mut interface.requests,
                _ => &mut interface.events,
            };
            let message = messages.last_mut().expect("an arg sits in a message");
            message.args.push(Arg {
                name: attr("name").unwrap_or_default(),
                kind: attr("type").unwrap_or_default(),
                interface: attr("interface"),
                enumeration: attr("enum"),
                allow_null: attr("allow-null"),
            });
        }
        ("enum", Some("interface")) => last_interface(protocol).enums.push(XmlEnum {
            name: attr("name").unwrap_or_default(),
            bitfield: attr("bitfield"),
            entries: Vec::new(),
        }),
        ("entry", Some("enum")) => {
            let enumeration = last_interface(protocol)
                .enums
                .last_mut()
                .expect("an entry sits in an enum");
            enumeration.entries.push((
                attr("name").unwrap_or_default(),
                attr("value").unwrap_or_default(),
            ));
        }
        ("copyright" | "description", _) => {}
        (tag, parent) => panic!("{}: unexpected <{tag}> in {parent:?}", protocol.file),
    }
}

fn last_interface(protocol: &mut Protocol) -> &mut Interface {
    protocol
        .interfaces
        .last_mut()
        .expect("an element sits in an interface")
}

fn attribute(element: &BytesStart, key: &str) -> Option<String> {
    element
        .attributes()
        .map(|attr| attr.expect("well-formed attribute"))
        .find(|attr| attr.key.as_ref() == key.as_bytes())
        .map(|attr| String::from_utf8_lossy(&attr.value).into_owned())
}
