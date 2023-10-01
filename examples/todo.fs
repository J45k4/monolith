
todoItem = (name, completed, on_click) => {
    return Div {
        style: {
            display: "flex"
        }
        children: [
            H1 {
                text: name
            }
            Input {
                type: "checkbox"
                checked: completed
                on_click
            }
        ]
    }
}

todos = []

return Html {
    head: Head {
        title: "Todo"
    }
    body: [
        H1 {
            text: "Todo"
        }
        Div {
            children: [
                Input {
                    type: "text"
                }
                Button {
                    text: "Add"
                    on_click: () => {
                        todos.push({
                            name: "Hello",
                            completed: false
                        })
                    }
                }
            ]
        }
        Div {
            children: todos.map((todo) => {
                return todoItem {
                    name: todo.name
                    completed: todo.completed
                }
            })
        }
    ]
}

