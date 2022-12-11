// deno-fmt-ignore-file
// deno-lint-ignore-file
// This code was bundled using `deno bundle` and it's not recommended to edit it manually

var LogLevel;
(function(LogLevel) {
    LogLevel[LogLevel["Debug"] = 1] = "Debug";
    LogLevel[LogLevel["Info"] = 2] = "Info";
    LogLevel[LogLevel["Warn"] = 3] = "Warn";
    LogLevel[LogLevel["Error"] = 4] = "Error";
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
const log = (...data)=>{
    console.log(`[MessageSender]`, ...data);
};
class MessageSender {
    ws;
    queue = [];
    timeout = 0;
    constructor(ws){
        this.ws = ws;
    }
    send(msg) {
        log("send", msg);
        this.queue.push(msg);
        this.sendNext();
    }
    sendNext() {
        log("sendNext");
        if (this.timeout) {
            log("timeout already exist");
            return;
        }
        this.timeout = setTimeout(()=>{
            log("timeout");
            this.sendNow();
        }, 500);
    }
    sendNow() {
        log("sendNow");
        clearInterval(this.timeout);
        this.timeout = 0;
        if (this.queue.length === 0) {
            log("queue is empty");
            return;
        }
        const jsonMsg = JSON.stringify(this.queue);
        log("sendingNow", jsonMsg);
        this.queue = [];
        this.ws.send(jsonMsg);
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
        case "div":
            {
                outerLogger.info("render div");
                if (old instanceof HTMLDivElement) {
                    old.innerHTML = "";
                    for(let i = 0; i < item.body.length; i++){
                        const el = renderItem(item.body[i], ctx);
                        old.appendChild(el);
                    }
                    return;
                }
                const div = document.createElement("div");
                for (const i1 of item.body){
                    const el1 = renderItem(i1, ctx);
                    div.appendChild(el1);
                }
                return div;
            }
        case "button":
            {
                const logger = outerLogger.child(`button:${item.name}:${item.id}`);
                logger.info("render button");
                if (old instanceof HTMLButtonElement) {
                    old.textContent = item.title;
                    return;
                }
                const button = document.createElement("button");
                button.innerText = item.title;
                button.onclick = ()=>{
                    logger.info("button clicked");
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
                const logger1 = outerLogger.child(`textInput:${item.name}:${item.id}`);
                logger1.info(`render textInput`, item);
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
                    logger1.info(`oninput ${input.value}`);
                    ctx.debouncer.change(e.target.value);
                };
                input.onkeydown = (e)=>{
                    logger1.info(`keydown: ${e.key}`);
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
                    logger1.info("focus");
                    ctx.debouncer.register((v)=>{
                        logger1.info(`changed to ${v}`);
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
                    logger1.info("blur");
                    ctx.debouncer.trigger();
                    ctx.debouncer.unregister();
                    registered = false;
                };
                return input;
            }
        case "checkbox":
            {
                const logger2 = outerLogger.child(`checkbox:${item.name}:${item.id}`);
                logger2.info("render checkbox");
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
const logger2 = createLogger("app");
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
    logger2.info("res", res);
    const ws = new WebSocket("ws://localhost:8080/ws");
    const sender = new MessageSender(ws);
    const ctx = {
        debouncer: new Deboncer(),
        sender: sender
    };
    ws.onmessage = (e)=>{
        const data = e.data.toString();
        logger2.info("rawdata", data);
        const messages = JSON.parse(data);
        logger2.info("received", messages);
        for (const message of messages){
            logger2.info("process", message);
            const element = getPathItem(message.path, root);
            logger2.info("element", element);
            if (!element) {
                logger2.info(`cannot find element with path ${message.path}`);
                continue;
            }
            if (message.type === "replace") {
                logger2.info("replace", message);
                const newEl = renderItem(message.item, ctx, element);
                if (newEl) {
                    element.replaceWith(newEl);
                }
            }
            if (message.type === "addBack") {
                logger2.info("addBack", message);
                const newEl1 = renderItem(message.item, ctx);
                if (newEl1) {
                    element.appendChild(newEl1);
                }
            }
            if (message.type === "removeInx") {
                element.children.item(message.inx)?.remove();
            }
        }
    };
    ws.onopen = ()=>{
        logger2.info("connected");
    };
    ws.onclose = ()=>{
        logger2.info("disconnected");
    };
};
