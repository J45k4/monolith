use flexscript::Value;

pub struct Html {
    pub head: Head,
    pub body: HtmlEl
}

impl Default for Html {
    fn default() -> Self {
        Html {
            head: Head {
                title: "".to_string()
            },
            body: HtmlEl {
                typ: HtmlElType::Body,
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
                            html.body = HtmlEl::from(prop.value.clone());
                        },
                        _ => todo!()
                    }
                }

                html
            },
            _ => todo!()
        }
    }
}

pub struct Head {
    pub title: String
}

impl Default for Head {
    fn default() -> Self {
        Head {
            title: "".to_string()
        }
    }
}

impl ToString for Head {
    fn to_string(&self) -> String {
        format!("<head>\n<title>{}</title>\n</head>", self.title)
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
    Input
}

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

pub struct  HtmlEl {
    pub typ: HtmlElType,
    pub children: Vec<Child>
}

impl ToString for HtmlEl {
    fn to_string(&self) -> String {
        let children = self.children.iter()
            .map(|child| child.to_string())
            .collect::<Vec<String>>().join("\n");

        match self.typ {
            HtmlElType::H1 => format!("<h1>\n{}\n</h1>", children),
            HtmlElType::H2 => format!("<h2>\n{}\n</h2>", children),
            HtmlElType::H3 => format!("<h3>\n{}\n</h3>", children),
            HtmlElType::H4 => format!("<h4>\n{}\n</h4>", children),
            HtmlElType::H5 => format!("<h5>\n{}\n</h5>", children),
            HtmlElType::H6 => format!("<h6>\n{}\n</h6>", children),
            HtmlElType::Div => format!("<div>\n{}\n</div>", children),
            HtmlElType::Body => format!("<body>\n{}\n</body>", children),
            HtmlElType::Button => format!("<button>\n{}\n</button>", children),
            HtmlElType::Input => format!("<input>\n{}\n</input>", children),
        }
    }
}

impl From<Value> for HtmlEl {
    fn from(value: Value) -> Self {
        match value {
            Value::Obj(obj) => {
                let typ = match obj.name.unwrap().as_str() {
                    "H1" => HtmlElType::H1,
                    "H2" => HtmlElType::H2,
                    "H3" => HtmlElType::H3,
                    "H4" => HtmlElType::H4,
                    "H5" => HtmlElType::H5,
                    "H6" => HtmlElType::H6,
                    "Button" => HtmlElType::Button,
                    "Div" => HtmlElType::Div,
                    "Input" => HtmlElType::Input,
                    _ => todo!()
                };


                let mut el = HtmlEl {
                    typ: typ,
                    children: vec![]
                };

                for prop in &obj.props {
                    match prop.name.as_ref() {
                        "type" => {
                            
                        },
                        "children" => {
                            if let Value::Array(arr) = &prop.value {
                                for item in arr {
                                    if let Value::Str(s) = item {
                                        el.children.push(Child::Text(s.to_string()));
                                    } else {
                                        el.children.push(Child::HtmlEl(HtmlEl::from(item.clone())));
                                    }
                                }
                            }
                        },
                        "text" => {
                            if let Value::Str(s) = &prop.value {
                                el.children.push(Child::Text(s.to_string()));
                            }
                        }
                        _ => todo!("{:?}", prop)
                    }
                }

                el
            },
            Value::Array(arr) => {
                let mut el = HtmlEl {
                    typ: HtmlElType::Div,
                    children: vec![]
                };

                for item in arr {
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