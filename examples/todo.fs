
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
                    // on_click: () => {
                    //     print("Hello")
                    // }
                }
            ]
        }
        Div {
            children: [1, 2, 3].map((p) => {
                return Div {
                    style: {
                        display: "flex"
                    }
                    children: [
                        H1 {
                            text: p
                        }
                        Button {
                            text: "Delete"
                            on_click: () => {
                                print("Hello")
                            }
                        }
                    ]
                }
            })
        }
    ]
}

