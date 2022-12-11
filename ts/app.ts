import { Deboncer } from "./debouncer.ts";
import { createLogger } from "./logger.ts";
import { MessageSender } from "./message_sender.ts";
import { getPathItem } from "./path.ts";
import { renderItem } from "./render.ts";
import { Context, SrvMessage } from "./types.ts";

const logger = createLogger("app")

window.onload = () => {
    const res = document.querySelector("body")

    if (!res) {
        return
    }

    res.innerHTML = ""

    res.style.display = "flex"
    res.style.flexDirection = "row"

    const content = document.createElement("div")
    content.style.border = "1px solid black"
    content.style.flexGrow = "1"

    res.appendChild(content)

    // const logger = new UILogger()

    // res.appendChild(logger.root)

    const root = document.createElement("div")
    content.appendChild(root)


    logger.info("res", res)

    const ws = new WebSocket("ws://localhost:8080/ws")

    const sender = new MessageSender(ws)

    const ctx: Context = {
        debouncer: new Deboncer(),
        sender: sender,
    }

    ws.onmessage = (e) => {
        const data = e.data.toString()

        logger.info("rawdata", data)
        const messages = JSON.parse(data) as SrvMessage[]
        logger.info("received", messages)
        // logger.log(`processing ${messages.length} messages`)

        for (const message of messages) {
            logger.info("process", message)

            const element = getPathItem(message.path, root)

            // logger.info("processing message", message)

            // const el = getPathItem(message.path, content)

            logger.info("element", element)

            if (!element) {
                logger.info(`cannot find element with path ${message.path}`)
                // logger.log(`cannot find element with path ${message.path}`)

                continue
            }

            if (message.type === "replace") {
                logger.info("replace", message)
                // logger.log(`replacing ${message.path} with ${message.item.type}`)

                const newEl = renderItem(message.item, ctx, element)
            
                if (newEl) {
                    element.replaceWith(newEl)
                }
            }   
            
            if (message.type === "addBack") {
                logger.info("addBack", message)
                const newEl = renderItem(message.item, ctx)

                if (newEl) {
                    element.appendChild(newEl)
                }
            }

            if (message.type === "removeInx") {
                element.children.item(message.inx)?.remove()
            }
        }
    }

    ws.onopen = () => {
        logger.info("connected")

        // ws.close()
    }

    ws.onclose = () => {
        logger.info("disconnected")
    }
}