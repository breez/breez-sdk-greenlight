export type EventListener = (breezEvent: BreezEvent) => void

export type LogStream = (logEntry: LogEntry) => void

export const connect = async (req: ConnectRequest, listener: EventListener): Promise<EmitterSubscription> => {
    const subscription = BreezSDKEmitter.addListener("breezSdkEvent", listener)
    
    await BreezSDK.connect(req)

    return subscription
}

export const setLogStream = async (logStream: LogStream): Promise<EmitterSubscription> => {
    const subscription = BreezSDKEmitter.addListener("breezSdkLog", logStream)

    try {
        await BreezSDK.setLogStream()
    } catch {}

    return subscription
}
