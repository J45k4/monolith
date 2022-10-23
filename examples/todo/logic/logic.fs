
export const completeTodo = (args, ctx) => {
    let todo = ctx.db.todo.fetch(args.todoId)

    if todo == None {
        throw "Todo not found"
    }

    todo.completed = true

    ctx.db.todo.save(todo)

    todo
}

export const login = (args, ctx) => {
    let user = ctx.db.user.fetch(args.username)

    if user == None {
        throw "User not found"
    }

    if user.password != args.password {
        throw "Invalid password"
    }

    let token = await(jwt.sign({
        userId: user.id
        name: user.name
    }, ctx.jwt_secret))

    return {
        token: token
    }
}