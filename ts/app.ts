import { Deboncer } from "./debouncer.ts";
import { createLogger } from "./logger.ts";
import { MessageSender } from "./message_sender.ts";
import { getPathItem } from "./path.ts";
import { renderItem } from "./render.ts";
import { Context, SrvMessage } from "./types.ts";
import { connectWebsocket } from "./ws.ts";

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

    const root = document.createElement("div")
    content.appendChild(root)


    logger.info("root", res)

    const debouncer = new Deboncer()

    const close = connectWebsocket((sender, msgs: SrvMessage[]) => { 
        const ctx: Context = {
            sender,
            debouncer
        }
        
        for (const message of msgs) {
            logger.info("process", message)

            const element = getPathItem(message.path, root)

            logger.info("element", element)

            if (!element) {
                logger.info(`cannot find element with path ${message.path}`)
                continue
            }

            if (message.type === "replace") {
                logger.info("replace", message)
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
    })
}