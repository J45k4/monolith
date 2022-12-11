import { MessageToSrv } from "./types.ts";

const log = (...data: any[]) => {
    console.log(`[MessageSender]`, ...data)
}

export class MessageSender {
    private ws: WebSocket
    private queue: MessageToSrv[] = []
    private timeout = 0
    constructor(ws: WebSocket) {
        this.ws = ws
    }

    public send(msg: MessageToSrv) {
        log("send", msg)

        this.queue.push(msg)
        this.sendNext()
    }

    private sendNext() {
        log("sendNext")

        if (this.timeout) {
            log("timeout already exist")

            return
        }

        this.timeout = setTimeout(() => {
            log("timeout")

            this.sendNow()
        }, 500)
    }

    public sendNow() {
        log("sendNow")

        clearInterval(this.timeout)
        this.timeout = 0

        if (this.queue.length === 0) {
            log("queue is empty")

            return
        }

        const jsonMsg = JSON.stringify(this.queue)

        log("sendingNow", jsonMsg)

        this.queue = []
        
        this.ws.send(jsonMsg)
    }
}