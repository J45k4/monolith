import { createLogger } from "./logger.ts";
import { MessageSender } from "./message_sender.ts";
import { MessageToSrv, SrvMessage } from "./types.ts";

const logger = createLogger("ws")

type OnMessage = (sender: MessageSender, msgs: SrvMessage[]) => void

export const connectWebsocket = (
    onMessage: OnMessage
) => {
    let ws: WebSocket | undefined

    const sender = new MessageSender((msgs: MessageToSrv[]) => {
        if (!ws) {
            return
        }

        ws.send(JSON.stringify(msgs))
    })

    const createConnection = () => {
        ws = new WebSocket("ws://localhost:33445/ui")

        ws.onmessage = (e) => {
            const data = e.data.toString()
    
            logger.info("rawdata", data)
            const messages = JSON.parse(data) as SrvMessage[]
            logger.info("received", messages)
    
            onMessage(sender, messages)
        }
    
        ws.onopen = () => {
            logger.info("connected")
        }
    
        ws.onclose = () => {
            logger.info("disconnected")
    
            setTimeout(() => {
                createConnection()
            }, 1000)
        }
    }

    createConnection()

    return () => {
        logger.debug("close")

        if (!ws) {
            return
        }

        ws.close()
    }
}