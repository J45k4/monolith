

let server = http.Server()

server.jwt = {
    type: "bearer",
    identity: schema.User
}

server.graphql = {
   completeItem
   shareShoppinglist
}

server.listen(80)