import { createLogger } from "./logger.ts";
import { Context, Item } from "./types.ts";

const outerLogger = createLogger("render")

export const renderItem = (item: Item, ctx: Context, old?: Element) => {
    outerLogger.info("renderItem", item, old)

    switch (item.type) {
        case "text": {
            if (old instanceof HTMLSpanElement) {
                old.innerHTML = item.text

                return
            }

            const span = document.createElement("span")

            span.innerText = item.text
            return span
        }
        case "view": {
            outerLogger.info("render view")

            if (old instanceof HTMLDivElement) {
                old.innerHTML = ""

                for (let i = 0; i < item.body.length; i++) {
                    const el = renderItem(item.body[i], ctx)
                    old.appendChild(el as any)
                }

                return
            }

            const div = document.createElement("div")

            if (item.width != null) {
                div.style.width = item.width + "px"
            }
            
            if (item.height != null) {
                div.style.height = item.height + "px"
            }

            div.style.overflow = "auto"
            
            if (item.flex) {
                div.style.display = "flex"

                const flex = item.flex

                div.style.flexDirection = flex.direction
                
                if (flex.grow) {
                    div.style.flexGrow = flex.grow.toString()
                }
            }

            for (const i of item.body) {
                const el = renderItem(i, ctx)
                div.appendChild(el as any)
            }

            return div
        }
        case "button": {
            const logger = outerLogger.child(`button:${item.name}:${item.id}`)

            logger.info("render button")

            if (old instanceof HTMLButtonElement) {
                old.textContent = item.title

                return
            }

            const button = document.createElement("button")
            button.innerText = item.title

            button.onclick = () => {
                logger.info("button clicked")

                ctx.sender.send({
                    type: "onClick",
                    id: item.id,
                    name: item.name,
                })

                ctx.sender.sendNow()
            }

            return button
        }
        case "textInput": {
            const logger = outerLogger.child(`textInput:${item.name}:${item.id}`)

            logger.info(`render textInput`, item)

            let registered = false

            if (old instanceof HTMLInputElement) {
                if (!registered || !ctx.debouncer.valueChanged) {
                    old.value = item.value
                }

                return
            }

            const input = document.createElement("input")
            input.placeholder = item.placeholder
            input.value = item.value

            input.oninput = (e: any) => {
                logger.info(`oninput ${input.value}`)

                ctx.debouncer.change(e.target.value)
            }
            
            input.onkeydown = (e) => {
                logger.info(`keydown: ${e.key}`)

                if (e.key === "Enter") {
                    ctx.debouncer.trigger()

                    ctx.sender.send({
                        type: "onKeyDown",
                        id: item.id,
                        name: item.name,
                        keycode: e.key,
                    })

                    ctx.sender.sendNow()
                }
            }

            input.onfocus = () => {
                logger.info("focus")

                ctx.debouncer.register(v => {
                    logger.info(`changed to ${v}`)

                    ctx.sender.send({
                        type: "onTextChanged",
                        id: item.id,
                        name: item.name,
                        value: v,
                    })

                    ctx.sender.sendNow()
                })

                registered = true
            }

            input.onblur = () => {
                logger.info("blur")

                ctx.debouncer.trigger()
                ctx.debouncer.unregister()

                registered = false
            }

            return input
        }
        case "checkbox": {
            const logger = outerLogger.child(`checkbox:${item.name}:${item.id}`)

            logger.info("render checkbox")

            if (old instanceof HTMLInputElement) {
                old.checked = item.checked
                
                return
            }

            const checkbox = document.createElement("input")
            checkbox.type = "checkbox"
            checkbox.checked = item.checked

            checkbox.onclick = () => {
                ctx.sender.send({
                    type: "onClick",
                    id: item.id,
                    name: item.name,
                })

                ctx.sender.sendNow()
            }

            return checkbox
        }
        default:
            return document.createTextNode("Unknown item type")
    }
}