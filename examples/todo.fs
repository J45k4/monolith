

PostRoute {
    path: "/"
    body: Html {
        head: Head {
            title: "Todo"
        }
        body: [
            H1 {
                text: "Todo"
            }
            Form {
                body: [
                    Input {
                        type: "text"
                    }
                    Button {
                        text: "Add"
                    }
                ]
            }
        ]
    }
}