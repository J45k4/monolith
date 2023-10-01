use flexscript::Value;

use crate::js::JSNode;

#[derive(Debug, Clone)]
pub struct Html {
    pub head: Head,
    pub body: HtmlEl
}

impl Default for Html {
    fn default() -> Self {
        Html {
            head: Head {
                title: "".to_string(),
                scripts: vec![]
            },
            body: HtmlEl {
                typ: HtmlElType::Body,
                style: CSSProps::default(),
                children: vec![]
            }
        }
    }
}

impl ToString for Html {
    fn to_string(&self) -> String {
        format!("<html>\n{}\n{}\n</html>", self.head.to_string(), self.body.to_string())
    }
}

impl From<Value> for Html {
    fn from(value: Value) -> Self {
        match value {
            Value::Obj(obj) => {
                let mut html = Html::default();

                for prop in &obj.props {
                    match prop.name.as_ref() {
                        "head" => {
                            html.head = Head::from(prop.value.clone());
                        },
                        "body" => {
                            let mut el = HtmlEl {
                                typ: HtmlElType::Body,
                                style: CSSProps::default(),
                                children: vec![]
                            };

                            match &prop.value {
                                Value::List(list) => {
                                    for item in list {
                                        match item {
                                            Value::Str(s) => {
                                                el.children.push(Child::Text(s.to_string()));
                                            },
                                            Value::Int(i) => {
                                                el.children.push(Child::Text(i.to_string()));
                                            },
                                            _ => {
                                                el.children.push(Child::HtmlEl(HtmlEl::from(item.clone())));
                                            }
                                        }
                                        // if let Value::Str(s) = item {
                                        //     el.children.push(Child::Text(s.to_string()));
                                        // } else {
                                        //     el.children.push(Child::HtmlEl(HtmlEl::from(item.clone())));
                                        // }
                                    }
                                },
                                _ => {
                                    el.children.push(Child::HtmlEl(HtmlEl::from(prop.value.clone())));
                                }
                            }

                            html.body = el;
                        },
                        _ => todo!()
                    }
                }

                html
            },
            // Value::List(list) => {

            // },
            _ => todo!("{:?}", value)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Script {
    pub content: Option<JSNode>
}

#[derive(Debug, Clone)]
pub struct Head {
    pub title: String,
    pub scripts: Vec<Script>
}

impl Default for Head {
    fn default() -> Self {
        Head {
            title: "".to_string(),
            scripts: vec![]
        }
    }
}

impl ToString for Head {
    fn to_string(&self) -> String {
        let mut scripts = vec![];
        for script in &self.scripts {
            if let Some(content) = &script.content {
                scripts.push(content.to_string());
            }
        }

        let scripts = scripts.iter()
            .map(|p| format!("<script>{}</script>", p))
            .collect::<Vec<String>>()
            .join("\n");

        format!("<head><title>{}</title>{}</head>", self.title, scripts)
    }
}

impl From<Value> for Head {
    fn from(value: Value) -> Self {
        match value {
            Value::Obj(obj) => {
                let mut head = Head::default();

                for prop in &obj.props {
                    match prop.name.as_ref() {
                        "title" => {
                            if let Value::Str(s) = &prop.value {
                                head.title = s.to_string();
                            }
                        },
                        _ => todo!()
                    }
                }

                head
            },
            _ => todo!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum HtmlElType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Div,
    Body,
    Button,
    Input,
    Head
}

#[derive(Debug, Clone)]
pub enum Child {
    HtmlEl(HtmlEl),
    Text(String)
}

impl ToString for Child {
    fn to_string(&self) -> String {
        match self {
            Child::HtmlEl(el) => el.to_string(),
            Child::Text(s) => s.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Display {
    Flex,
    Initial
}

impl Default for Display {
    fn default() -> Self {
        Display::Initial
    }
}

impl TryFrom<String> for Display {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Display::try_from(&value)
    }
}

impl TryFrom<&String> for Display {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "flex" => Ok(Display::Flex),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub enum FlexDirection {
    Row,
    Column,
    RowReserve,
    ColumnReserve,
    None
}

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::None
    }
}

impl TryFrom<&String> for FlexDirection {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "row" => Ok(FlexDirection::Row),
            "column" => Ok(FlexDirection::Column),
            "row-reverse" => Ok(FlexDirection::RowReserve),
            "column-reverse" => Ok(FlexDirection::ColumnReserve),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CSSProps {
    display: Display,
    flex_direction: FlexDirection
}

impl ToString for CSSProps {
    fn to_string(&self) -> String {
        let mut s = String::new();

        match self.display {
            Display::Flex => {
                s.push_str("display: flex;");
            },
            _ => {}
        }

        match self.flex_direction {
            FlexDirection::Row => {
                s.push_str("flex-direction: row;");
            },
            FlexDirection::Column => {
                s.push_str("flex-direction: column;");
            },
            FlexDirection::RowReserve => {
                s.push_str("flex-direction: row-reverse;");
            },
            FlexDirection::ColumnReserve => {
                s.push_str("flex-direction: column-reverse;");
            },
            _ => {}
        }

        s
    }
}

#[derive(Debug, Clone)]
pub struct  HtmlEl {
    pub typ: HtmlElType,
    pub style: CSSProps,
    pub children: Vec<Child>
}

impl ToString for HtmlEl {
    fn to_string(&self) -> String {
        let children = self.children.iter()
            .map(|child| child.to_string())
            .collect::<Vec<String>>().join("\n");

        let style = self.style.to_string();

        match self.typ {
            HtmlElType::H1 => format!("<h1>{}</h1>", children),
            HtmlElType::H2 => format!("<h2>{}</h2>", children),
            HtmlElType::H3 => format!("<h3>{}</h3>", children),
            HtmlElType::H4 => format!("<h4>{}</h4>", children),
            HtmlElType::H5 => format!("<h5>{}</h5>", children),
            HtmlElType::H6 => format!("<h6>{}</h6>", children),
            HtmlElType::Div => format!(r#"<div style="{}">{}</div>"#, style, children),
            HtmlElType::Body => format!("<body>{}</body>", children),
            HtmlElType::Button => format!("<button>{}</button>", children),
            HtmlElType::Input => format!("<input>{}</input>", children),
            HtmlElType::Head => format!("<head>{}</head>", children),
        }
    }
}

impl From<Value> for HtmlEl {
    fn from(value: Value) -> Self {
        match value {
            Value::Obj(obj) => {
                let name = obj.name.unwrap();
                let typ = match name.as_str() {
                    "H1" => HtmlElType::H1,
                    "H2" => HtmlElType::H2,
                    "H3" => HtmlElType::H3,
                    "H4" => HtmlElType::H4,
                    "H5" => HtmlElType::H5,
                    "H6" => HtmlElType::H6,
                    "Button" => HtmlElType::Button,
                    "Div" => HtmlElType::Div,
                    "Input" => HtmlElType::Input,
                    "Head" => HtmlElType::Head,
                    _ => todo!("{:?}", name)
                };


                let mut el = HtmlEl {
                    typ: typ,
                    style: CSSProps::default(),
                    children: vec![]
                };

                for prop in &obj.props {
                    match prop.name.as_ref() {
                        "type" => {
                            
                        },
                        "children" => {
                            if let Value::List(list) = &prop.value {
                                for item in list {
                                    if let Value::Str(s) = item {
                                        el.children.push(Child::Text(s.to_string()));
                                    } else {
                                        el.children.push(Child::HtmlEl(HtmlEl::from(item.clone())));
                                    }
                                }
                            }
                        },
                        "text" => {
                            match &prop.value {
                                Value::Str(s) => {
                                    el.children.push(Child::Text(s.to_string()));
                                },
                                Value::Int(i) => {
                                    el.children.push(Child::Text(i.to_string()));
                                },
                                _ => todo!("{:?}", prop)
                            }
                        },
                        "title" => {},
                        "style" => {
                            match &prop.value {
                                Value::Obj(obj) => {
                                    for prop in &obj.props {
                                        match prop.name.as_ref() {
                                            "display" => {
                                                match &prop.value {
                                                    Value::Str(str) => {
                                                        el.style.display = Display::try_from(str).unwrap()
                                                    },
                                                    _ => todo!("{:?}", prop)
                                                }
                                                
                                            },
                                            "flexDirection" => {
                                                
                                            },
                                            _ => todo!("{:?}", prop)
                                        }
                                    }
                                },
                                _ => todo!("{:?}", prop)
                            }
                        }
                        _ => {}
                    }
                }

                el
            },
            Value::List(list) => {
                let mut el = HtmlEl {
                    typ: HtmlElType::Div,
                    style: CSSProps::default(),
                    children: vec![]
                };

                for item in list {
                    if let Value::Str(s) = item {
                        el.children.push(Child::Text(s.to_string()));
                    } else {
                        el.children.push(Child::HtmlEl(HtmlEl::from(item.clone())));
                    }
                }

                el
            },
            _ => todo!("{:?}", value)
        }
    }
}

#[cfg(test)]
mod test {
    use flexscript::RunResult;
    use flexscript::Vm;

    use super::*;

    #[test]
    fn empty_html_page() {
        let html = Html::default();

        assert_eq!(html.to_string(), r#"<html>
<head>
<title></title>
</head>
<body>

</body>
</html>"#);
    }

    #[test]
    fn html_page_from_value() {
        let mut vm = Vm::new();

        let res = vm.run_code(r#"
        return Html {
            head: Head {
                title: "hello"
            },
            body: []
        }"#);

        match res {
            RunResult::Value(value) => {
                let html = Html::from(value).to_string();
                assert_eq!(html, "<html>\n<head>\n<title>hello</title>\n</head>\n<body>\n\n</body>\n</html>");
            },
            _ => todo!()
        }
    }

    #[test]
    fn three_titles() {
        let mut vm = Vm::new();

        let res = vm.run_code(r#"
        return Html {
            head: Head {
                title: "hello"
            },
            body: [1, 2, 3].map((p) => {
                return H1 {
                    text: p
                }
            })
        }"#);

        println!("{:?}", res);

        match res {
            RunResult::Value(value) => {
                let html = Html::from(value).to_string();
                assert_eq!(html, "<html>\n<head>\n<title>hello</title>\n</head>\n<body>\n<h1>\n1\n</h1>\n<h1>\n2\n</h1>\n<h1>\n3\n</h1>\n</body>\n</html>");
            },
            _ => todo!()
        }
    }
}