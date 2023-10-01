
return Html {
    head: Head {
        title: "Hello World"
    }
    body: [
        Button {
            text: "Click me!"
            on_click: () => {
                console.log("Hello World!")
            }
        }
    ]
}