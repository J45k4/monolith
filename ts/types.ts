import { Deboncer } from "./debouncer.ts";
import { MessageSender } from "./message_sender.ts";

export type Text = {
    type: "text"
    text: string
}

export enum FlexDirection {
    Row = "row",
    Column = "column"
}

export type View = {
    type: "view"
    flex?: {
        grow: number
        direction: FlexDirection
    }
    height: number
    width: number
    body: Item[]
}

export type Button = {
    type: "button"
    id: string
    name: string
    title: string
}

export type TextInput = {
    type: "textInput"
    id: string
    name: string
    placeholder: string
    value: string
}

export type Table = {
    type: "table"
    headers: string[]
    rows: Item[][]
}

export type Checkbox = {
    type: "checkbox"
    id: string
    name: string
    checked: boolean
}

export type Item = View | 
    Text | 
    Button | 
    TextInput | 
    Table | 
    Checkbox

export type Replace = {
    type: "replace"
    path: number[]
    item: Item
}

export type ReplaceAt = {
    type: "replaceAt"
    inx: number
    path: number[]
    item: Item
}

export type AddBack = {
    type: "addBack"
    path: number[]
    item: Item
}

export type AddFront = {
    type: "addFront"
    path: number[]
    item: Item
}

export type InsertAt = {
    type: "insertAt"
    inx: number
    item: Item
    path: number[]
}

export type RemoveInx = {
    type: "removeInx"
    inx: number
    path: number[]
}

export type SrvMessage = Replace |
    ReplaceAt |
    AddBack | 
    AddFront | 
    InsertAt | 
    RemoveInx

export type OnClick = {
    type: "onClick"
    id: string
    name: string
}

export type OnTextChange = {
    type: "onTextChanged"
    id: string
    name: string
    value: string
}

export type OnKeyDown = {
    type: "onKeyDown"
    id: string
    name: string
    keycode: string
}

export type ParametersChanged = {
    type: "parametersChanged"
    params: any
    query: any
    headers: any
}

export type MessageToSrv = OnClick | OnTextChange | OnKeyDown | ParametersChanged

export type MessagesToSrv = MessageToSrv[]

export type Context = {
    debouncer: Deboncer
    sender: MessageSender
}