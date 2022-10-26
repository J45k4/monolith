
type Item = {
    id int @db(autoincrement=true, primary_key=true)
    name string
    quantity int
    completed bool
}

type Shoppinglist = {
    id int @db(autoincrement=true, primary_key=true)
    name string
    items Item[]
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
    shoppinglists Shoppinglist[] @graphql
    password string @db(password=true)
}