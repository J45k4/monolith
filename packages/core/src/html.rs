use flexscript::{ASTNode, Value};


pub fn build_node_html(html: &mut String, ast: &ASTNode) {
    println!("build_node_html");

    match ast {
        ASTNode::Ident(_) => todo!(),
        ASTNode::Assign(_) => todo!(),
        ASTNode::ObjIns(obj) => {
            let name 
            println!("Obj name: {}", obj.name);
            match obj.name.as_ref() {
                "Html" => {
                    *html += "<html>";
                    for prop in &obj.probs {
                        println!("prop.key: {}", prop.name);

                        match prop.name.as_ref() {
                            "head" => {
                                build_node_html(html, &prop.value);
                            },
                            "body" => {
                                *html += "<body>";
                                build_node_html(html, &prop.value);
                                *html += "</body>";
                            },
                            _ => todo!()
                        }
                    }
                    *html += "</html>";
                },
                "H1" => {
                    for prop in &obj.probs {
                        match prop.name.as_ref() {
                            "text" => {
                                *html += "<h1>";
                                build_node_html(html, &prop.value);
                                *html += "</h1>";
                            },
                            _ => todo!()
                        }
                    }
                },
                "Form" => {
                    for prop in &obj.probs {
                        match prop.name.as_ref() {
                            "body" => {
                                *html += "<form>";
                                build_node_html(html, &prop.value);
                                *html += "</form>";
                            },
                            _ => todo!()
                        }
                    }
                },
                "Input" => {
                    for prop in &obj.probs {
                        match prop.name.as_ref() {
                            "type" => {
                                *html += "<input type=\"";
                                build_node_html(html, &prop.value);
                                *html += "\">";
                            },
                            _ => todo!()
                        }
                    }
                },
                "Button" => {
                    for prop in &obj.probs {
                        match prop.name.as_ref() {
                            "text" => {
                                *html += "<button>";
                                build_node_html(html, &prop.value);
                                *html += "</button>";
                            },
                            _ => todo!()
                        }
                    }
                },
                "Head" => {
                    *html += "<head>";
                    for prop in &obj.probs {
                        match prop.name.as_ref() {
                            "title" => {
                                *html += "<title>";
                                build_node_html(html, &prop.value);
                                *html += "</title>";
                            },
                            _ => todo!()
                        }
                    }
                    *html += "<head>";
                },
                _ => todo!()
            }
        },
        ASTNode::Array(arr) => {
            for node in arr.items.iter() {
                build_node_html(html, node);
            }
        },
        ASTNode::Call(_) => todo!(),
        ASTNode::Property(_, _) => todo!(),
        ASTNode::Lit(lit) => {
            match lit {
                Value::Str(s) => {
                    *html += s;
                },
                _ => todo!()
            };
        },
        ASTNode::LiteralPercent(_) => todo!(),
        ASTNode::Fun(_) => todo!(),
        ASTNode::StructDef(_) => todo!(),
        ASTNode::TypeDef(_) => todo!(),
        ASTNode::Var(_) => todo!(),
        ASTNode::ProbAccess(_) => todo!(),
        ASTNode::Obj(_) => todo!(),
        ASTNode::Ret(_) => todo!(),
        ASTNode::BinOp(_) => todo!(),
        _ => {}
    };
}



pub fn build_nodes_html(ast: &Vec<ASTNode>) -> String {
    let mut html = String::new();

    for node in ast {
        build_node_html(&mut html, node);
    }

    html
}

#[cfg(test)]
mod tests {
    use flexscript::Parser;

    use super::*;

    #[test]
    fn test_build_html() {
        let code = r#"
            Html {
                head: Head {
                    title: "Hello World"
                },
                body: [
                    H1 {
                        text: "Hello World"
                    }
                ]
            }
        "#;

        let ast = Parser::new(code).parse();

        let html = build_nodes_html(&ast);

        println!("{}", html);
    }
}