
type Message = {
    id Int @db(autoincrement=true, primary_key=true)
    text String
    user User
    timestamp DateTime
    chatroom Chatroom
}

type Chatroom = {
    participants Participant[]
    messages Message[]
}

type Participant = {
    chatrooms Chatroom
    user User
    is_admin bool
}

type User = {
    participants Participant[]
}
