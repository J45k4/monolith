pub trait ToJS {
    fn to_js(&self) -> String;
}

pub trait ToHTML {
    fn to_html(&self) -> String;
}

// pub fn js_code_for_node(buff: &mut String, ast: &ASTNode) {
//     match ast {
//         ASTNode::Assign(assign) => {
//             js_code_for_node(buff, &assign.left);
//             buff.push_str(" = ");
//             js_code_for_node(buff, &assign.right);
//         },
//         ASTNode::Ident(ident) => {
//             buff.push_str(&ident);
//         },
//         ASTNode::Lit(lit) => {
//             match lit {
//                 Value::Int(i) => {
//                     buff.push_str(&i.to_string());
//                 },
//                 _ => {
//                     todo!("unexpected value: {:?}", lit);
//                 }
//             }
//         },
//         _ => {
//             todo!("unexpected node: {:?}", ast);
//         }
//     }
// }

// pub fn js_code_for_nodes(buff: &mut String, ast: &Vec<ASTNode>) {
//     for node in ast {
//         js_code_for_node(buff, node);
//     }
// }

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

impl ToHTML for Child {
    fn to_html(&self) -> String {
        match self {
            Child::HtmlEl(el) => el.to_html(),
            Child::Text(s) => s.clone()
        }
    }
}

pub struct  HtmlEl {
    pub typ: HtmlElType,
    pub children: Vec<Child>
}

impl ToHTML for HtmlEl {
    fn to_html(&self) -> String {
        let children = self.children.iter()
            .map(|child| child.to_html())
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
        format!("<html>\n{}\n{}\n</html>", self.head.to_string(), self.body.to_html())
    }
}

// impl ToHTML for HtmlNode {
//     fn to_html(&self) -> String {
//         match self {
//             HtmlNode::H1(s) => format!("<h1>{}</h1>", s),
//             HtmlNode::H2(s) => format!("<h2>{}</h2>", s),
//             HtmlNode::H3(s) => format!("<h3>{}</h3>", s),
//             HtmlNode::H4(s) => format!("<h4>{}</h4>", s),
//             HtmlNode::H5(s) => {
//                 format!("<h5>{}</h5>", s)
//             },
//             HtmlNode::H6(s) => {
//                 format!("<h6>{}</h6>", s)
//             },
//         }
//     }
// }


pub struct Fn {
    pub asyn: bool,
    pub args: Vec<String>,
    pub body: Vec<CodeNode>
}

impl ToJS for Fn {
    fn to_js(&self) -> String {
        let mut buff = String::new();
        if self.asyn {
            buff.push_str("async ");
        }
        buff.push_str("(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                buff.push_str(", ");
            }
            buff.push_str(arg);
        }
        buff.push_str(") => {\n");
        for node in &self.body {
            buff.push_str(&node.to_js());
            buff.push_str("\n");
        }
        buff.push_str("}");
        buff
    }
}

pub struct Call {
    pub calle: Box<CodeNode>,
    pub args: Vec<CodeNode>
}

impl ToJS for Call {
    fn to_js(&self) -> String {
        let mut buff = String::new();
        buff.push_str(&self.calle.to_js());
        buff.push_str("(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                buff.push_str(", ");
            }
            buff.push_str(&arg.to_js());
        }
        buff.push_str(")");
        buff
    }
}

pub struct ConstVar {
    pub name: String,
    pub value: Box<CodeNode>
}

impl ToJS for ConstVar {
    fn to_js(&self) -> String {
        format!("const {} = {}", self.name, self.value.to_js())
    }
}

pub enum ForCond {
    In {
        name: String,
        value: Box<CodeNode>
    },
    Of {
        name: String,
        value: Box<CodeNode>
    }
}

pub struct For {
    pub cond: ForCond,
    pub body: Vec<CodeNode>
}

impl ToJS for For {
    fn to_js(&self) -> String {
        let mut buff = String::new();
        buff.push_str("for (const ");
        match &self.cond {
            ForCond::In { name, value } => {
                buff.push_str(&name);
                buff.push_str(" in ");
                buff.push_str(&value.to_js());
            },
            ForCond::Of { name, value } => {
                buff.push_str(&name);
                buff.push_str(" of ");
                buff.push_str(&value.to_js());
            }
        }
        buff.push_str(") {\n");
        for node in &self.body {
            buff.push_str(&node.to_js());
            buff.push_str("\n");
        }
        buff.push_str("}");
        buff
    }
}

pub struct PropAccess {
    pub name: Box<CodeNode>,
    pub value: Box<CodeNode>
}

pub struct Assign {
    pub name: Box<CodeNode>,
    pub value: Box<CodeNode>
}

pub enum CodeNode {
    None,
    Fn(Fn),
    Call(Call),
    ConstVar(ConstVar),
    Num(i64),
    Str(String),
    Ident(String),
    PropAccess(PropAccess),
}

impl ToJS for CodeNode {
    fn to_js(&self) -> String {
        match self {
            CodeNode::None => "".to_string(),
            CodeNode::Fn(f) => f.to_js(),
            CodeNode::Call(c) => c.to_js(),
            CodeNode::ConstVar(c) => c.to_js(),
            CodeNode::Num(n) => n.to_string(),
            CodeNode::Str(s) => format!("\"{}\"", s),
            CodeNode::Ident(i) => i.to_string(),
            CodeNode::PropAccess(p) => format!("{}.{}", p.name.to_js(), p.value.to_js())
        }
    }
}

pub enum CssCode {
    None
}

impl ToString for CssCode {
    fn to_string(&self) -> String {
        match self {
            CssCode::None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_codegen() {
        let code_desc = ConstVar {
            name: "a".to_string(),
            value: Box::new(CodeNode::Num(5))
        };

        assert_eq!(code_desc.to_js(), "const a = 5");
    }

    #[test]
    fn test_js_async_fn() {
        let code_desc = Fn {
            asyn: true,
            args: vec!["a".to_string(), "b".to_string()],
            body: vec![
                CodeNode::Call(Call {
                    calle: Box::new(CodeNode::PropAccess(PropAccess {
                        name: Box::new(CodeNode::Ident("console".to_string())),
                        value: Box::new(CodeNode::Ident("log".to_string()))
                    })),
                    args: vec![
                        CodeNode::Str("hello".to_string())
                    ]
                })
            ]
        };

        assert_eq!(code_desc.to_js(), "async (a, b) => {\nconsole.log(\"hello\")\n}");
    }

    #[test]
    fn test_js_for() {
        let code_desc = For {
            cond: ForCond::In {
                name: "a".to_string(),
                value: Box::new(CodeNode::Ident("b".to_string()))
            },
            body: vec![
                CodeNode::Call(Call {
                    calle: Box::new(CodeNode::Ident("foo".to_string())),
                    args: vec![]
                })
            ]
        };

        assert_eq!(code_desc.to_js(), "for (const a in b) {\nfoo()\n}");
    }

    #[test]
    fn test_js_prop_access() {
        let code_desc = Call {
            calle: Box::new(CodeNode::PropAccess(PropAccess { 
                name: Box::new(CodeNode::Ident("document".to_string())), 
                value: Box::new(CodeNode::Ident("createElement".to_string())),
            })), 
            args: vec![CodeNode::Str("div".to_string())]
        };

        assert_eq!(code_desc.to_js(), r#"document.createElement("div")"#);
    }

    #[test]
    fn test_html_test_div() {
        let html_desc = HtmlEl {
            typ: HtmlElType::Div,
            children: vec![
                Child::Text("hello".to_string()),
            ]
        };

        assert_eq!(html_desc.to_html(), "<div>\nhello\n</div>");
    }
}