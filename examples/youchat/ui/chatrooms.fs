
export const ChatroomList = ({}) => {
    return (
        <table>
            <thead>
                <tr>
                    <th>Chatroom</th>
                    <th>Participants</th>
                </tr>
            </thead>
            <tbody>
                {db.chatrooms.map(chatroom => (
                    <tr>
                        <td>{chatroom.name}</td>
                        <td>{chatroom.participants.length}</td>
                    </tr>
                ))}
            </tbody>
        </table>
    )
}