
export const completeItem = (args, ctx) => {
    let item = ctx.item.fetch(args.id)

    if item == None {
        throw "Item not found"
    }

    item.completed = true

    ctx.item.save(item)
}

export const shareShoppinglist = (args, ctx) => {
    let shoppinglist = ctx.shoppinglist.fetch(args.id)

    if shoppinglist == None {
        throw "Shoppinglist not found"
    }

    let user = ctx.user.fetch(args.userId)

    if user == None {
        throw "User not found"
    }

    shoppinglist.users.push(user)

    ctx.shoppinglist.save(shoppinglist)
}