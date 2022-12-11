// deno-fmt-ignore-file
// deno-lint-ignore-file
// This code was bundled using `deno bundle` and it's not recommended to edit it manually

var LogLevel;
(function(LogLevel1) {
    LogLevel1[LogLevel1["Debug"] = 1] = "Debug";
    LogLevel1[LogLevel1["Info"] = 2] = "Info";
    LogLevel1[LogLevel1["Warn"] = 3] = "Warn";
    LogLevel1[LogLevel1["Error"] = 4] = "Error";
})(LogLevel || (LogLevel = {}));
let loglevel = LogLevel.Info;
const createLogger = (name)=>{
    return {
        info: (...data)=>{
            if (loglevel < LogLevel.Info) {
                return;
            }
            console.log(`[${name}]`, ...data);
        },
        error: (...data)=>{
            if (loglevel < LogLevel.Error) {
                return;
            }
            console.error(`[${name}]`, ...data);
        },
        warn: (...data)=>{
            if (loglevel < LogLevel.Warn) {
                return;
            }
            console.warn(`[${name}]`, ...data);
        },
        debug: (...data)=>{
            if (loglevel < LogLevel.Debug) {
                return;
            }
            console.debug(`[${name}]`, ...data);
        },
        child: (childName)=>{
            return createLogger(`${name}:${childName}`);
        }
    };
};
const logger = createLogger("debouncer");
class Deboncer {
    timeout;
    value = "";
    valueChanged = false;
    cb = null;
    change(text) {
        logger.info("change", text);
        this.valueChanged = true;
        this.value = text;
        clearTimeout(this.timeout);
        this.timeout = setTimeout(()=>{
            logger.info("timeout");
            this.trigger();
        }, 500);
    }
    unregister() {
        logger.info("unregister");
        this.cb = null;
    }
    register(cb) {
        logger.info("register");
        this.cb = cb;
    }
    trigger() {
        logger.info("trigger", this.value, this.valueChanged);
        if (this.timeout) {
            clearTimeout(this.timeout);
            this.timeout = null;
            logger.info("timeout cleared");
        }
        if (!this.valueChanged) {
            logger.info("value is not changed");
            return;
        }
        this.valueChanged = false;
        if (this.cb) {
            logger.info("debouncer is triggered with", this.value);
            this.cb(this.value);
        }
        this.value = "";
    }
}
const logger1 = createLogger("path");
const getPathItem = (path, element)=>{
    logger1.info(`getPathItem`, {
        path,
        element
    });
    const p = path[0];
    logger1.info(`first path item: ${p}`);
    if (p == null) {
        logger1.info("returning element", element);
        return element;
    }
    const child = element.children[p];
    logger1.info("child", child);
    if (!child) {
        logger1.info(`child not found: ${p}`);
        return;
    }
    logger1.info(`child found: ${p}`);
    return getPathItem(path.slice(1), child);
};
const outerLogger = createLogger("render");
const renderItem = (item, ctx, old)=>{
    outerLogger.info("renderItem", item, old);
    switch(item.type){
        case "text":
            if (old instanceof Text) {
                old.textContent = item.text;
                return;
            }
            return document.createTextNode(item.text);
        case "view":
            {
                outerLogger.info("render view");
                if (old instanceof HTMLDivElement) {
                    old.innerHTML = "";
                    for(let i = 0; i < item.body.length; i++){
                        const el = renderItem(item.body[i], ctx);
                        old.appendChild(el);
                    }
                    return;
                }
                const div = document.createElement("div");
                for (const i of item.body){
                    const el = renderItem(i, ctx);
                    div.appendChild(el);
                }
                return div;
            }
        case "button":
            {
                const logger5 = outerLogger.child(`button:${item.name}:${item.id}`);
                logger5.info("render button");
                if (old instanceof HTMLButtonElement) {
                    old.textContent = item.title;
                    return;
                }
                const button = document.createElement("button");
                button.innerText = item.title;
                button.onclick = ()=>{
                    logger5.info("button clicked");
                    ctx.sender.send({
                        type: "onClick",
                        id: item.id,
                        name: item.name
                    });
                    ctx.sender.sendNow();
                };
                return button;
            }
        case "textInput":
            {
                const logger6 = outerLogger.child(`textInput:${item.name}:${item.id}`);
                logger6.info(`render textInput`, item);
                let registered = false;
                if (old instanceof HTMLInputElement) {
                    if (!registered || !ctx.debouncer.valueChanged) {
                        old.value = item.value;
                    }
                    return;
                }
                const input = document.createElement("input");
                input.placeholder = item.placeholder;
                input.value = item.value;
                input.oninput = (e)=>{
                    logger6.info(`oninput ${input.value}`);
                    ctx.debouncer.change(e.target.value);
                };
                input.onkeydown = (e)=>{
                    logger6.info(`keydown: ${e.key}`);
                    if (e.key === "Enter") {
                        ctx.debouncer.trigger();
                        ctx.sender.send({
                            type: "onKeyDown",
                            id: item.id,
                            name: item.name,
                            keycode: e.key
                        });
                        ctx.sender.sendNow();
                    }
                };
                input.onfocus = ()=>{
                    logger6.info("focus");
                    ctx.debouncer.register((v)=>{
                        logger6.info(`changed to ${v}`);
                        ctx.sender.send({
                            type: "onTextChanged",
                            id: item.id,
                            name: item.name,
                            value: v
                        });
                        ctx.sender.sendNow();
                    });
                    registered = true;
                };
                input.onblur = ()=>{
                    logger6.info("blur");
                    ctx.debouncer.trigger();
                    ctx.debouncer.unregister();
                    registered = false;
                };
                return input;
            }
        case "checkbox":
            {
                const logger7 = outerLogger.child(`checkbox:${item.name}:${item.id}`);
                logger7.info("render checkbox");
                if (old instanceof HTMLInputElement) {
                    old.checked = item.checked;
                    return;
                }
                const checkbox = document.createElement("input");
                checkbox.type = "checkbox";
                checkbox.checked = item.checked;
                checkbox.onclick = ()=>{
                    ctx.sender.send({
                        type: "onClick",
                        id: item.id,
                        name: item.name
                    });
                    ctx.sender.sendNow();
                };
                return checkbox;
            }
        default:
            return document.createTextNode("Unknown item type");
    }
};
const logger2 = createLogger("message_sender");
class MessageSender {
    sender;
    queue = [];
    timeout = 0;
    constructor(send){
        this.sender = send;
    }
    send(msg) {
        logger2.info("send", msg);
        this.queue.push(msg);
        this.sendNext();
    }
    sendNext() {
        logger2.info("sendNext");
        if (this.timeout) {
            logger2.info("timeout already exist");
            return;
        }
        this.timeout = setTimeout(()=>{
            logger2.info("timeout");
            this.sendNow();
        }, 500);
    }
    sendNow() {
        logger2.info("sendNow");
        clearInterval(this.timeout);
        this.timeout = 0;
        if (this.queue.length === 0) {
            logger2.info("queue is empty");
            return;
        }
        logger2.info("sendingNow", this.queue);
        this.sender(this.queue);
        this.queue = [];
    }
}
const logger3 = createLogger("ws");
const connectWebsocket = (onMessage)=>{
    let ws;
    const sender = new MessageSender((msgs)=>{
        if (!ws) {
            return;
        }
        ws.send(JSON.stringify(msgs));
    });
    const createConnection = ()=>{
        ws = new WebSocket("ws://localhost:33445/ui");
        ws.onmessage = (e)=>{
            const data = e.data.toString();
            logger3.info("rawdata", data);
            const messages = JSON.parse(data);
            logger3.info("received", messages);
            onMessage(sender, messages);
        };
        ws.onopen = ()=>{
            logger3.info("connected");
        };
        ws.onclose = ()=>{
            logger3.info("disconnected");
            setTimeout(()=>{
                createConnection();
            }, 1000);
        };
    };
    createConnection();
    return ()=>{
        logger3.debug("close");
        if (!ws) {
            return;
        }
        ws.close();
    };
};
const logger4 = createLogger("app");
window.onload = ()=>{
    const res = document.querySelector("body");
    if (!res) {
        return;
    }
    res.innerHTML = "";
    res.style.display = "flex";
    res.style.flexDirection = "row";
    const content = document.createElement("div");
    content.style.border = "1px solid black";
    content.style.flexGrow = "1";
    res.appendChild(content);
    const root = document.createElement("div");
    content.appendChild(root);
    logger4.info("root", res);
    const debouncer = new Deboncer();
    connectWebsocket((sender, msgs)=>{
        const ctx = {
            sender,
            debouncer
        };
        for (const message of msgs){
            logger4.info("process", message);
            const element = getPathItem(message.path, root);
            logger4.info("element", element);
            if (!element) {
                logger4.info(`cannot find element with path ${message.path}`);
                continue;
            }
            if (message.type === "replace") {
                logger4.info("replace", message);
                const newEl = renderItem(message.item, ctx, element);
                if (newEl) {
                    element.replaceWith(newEl);
                }
            }
            if (message.type === "addBack") {
                logger4.info("addBack", message);
                const newEl = renderItem(message.item, ctx);
                if (newEl) {
                    element.appendChild(newEl);
                }
            }
            if (message.type === "removeInx") {
                element.children.item(message.inx)?.remove();
            }
        }
    });
};
