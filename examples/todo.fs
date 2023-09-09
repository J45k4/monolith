
// return "hello"

// return Html {
//     head: Head {
//         title: "LOOL"
//     }
//     body: []
// }

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
                        print("Hello")
                    }
                }
            ]
        }
    ]
}

