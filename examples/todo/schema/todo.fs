
@db
@graphql
type TodoItem = {
    @db(autoincrement=true, primary_key=true) 
    @graphql
    id int
    title string @db @graphql
    completed bool @db @graphql
}

@db
@graphql
type User = {
    @db(autoincrement=true, primary_key=true) 
    @graphql
    id int
    name string @graphql
    @perm(
        allowCreate=true, 
        allowRead=true, 
        allowUpdate=true, 
        allowDelete=true
    )
    todos TodoItem[] @graphql
    password string @db(password=true)
}