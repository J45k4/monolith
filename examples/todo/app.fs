

let server = http.create_server()

server.jwt = {
    type: "bearer",
    identity: schema.User
}

server.graphql = {
    logic.completeTodo
}

server.listen(80)