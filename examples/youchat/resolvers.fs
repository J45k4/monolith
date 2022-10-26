
export const sendMessage = (args, ctx) => {
    let message = {
        id: ctx.db.message.nextId()
        text: args.text
        sender: ctx.user.id
        receiver: args.receiver
    }

    ctx.db.message.save(message)

    message
}

export const inviteUser = (args, ctx) => {
    let chatroom = ctx.db.chatroom.fetch(args.chatroomId)

    if chatroom == None {
        throw "Chatroom not found"
    }

    if chatroom.has_participant(args.userId) {
        throw "User is already a participant"
    }

    chatroom.add_participant(args.userId)
}