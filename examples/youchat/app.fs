import monolith

let app = monolith.create()

app.graphql = {
    sendMessage
    inviteUser
}

app.http_port = 80