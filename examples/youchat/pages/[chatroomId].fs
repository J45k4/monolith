

export default ({ chatroomId: Int }, ctx) => {
    return (
        <div>
            {ctx.db.chatroom.fetch(chatroomId).messages.map(p => (
                <div>
                    <p>{p.text}</p>
                    <p>{p.user.name}</p>
                </div>
            ))}
            <input type="text" value={ctx.state.message} />
            <button onClick={() => {
                ctx.sendMessage({
                    text: ctx.state.message
                    chatroomId: chatroomId 
                })
            }}>
                Send
            </button>
        </div>
    )
}