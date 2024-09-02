import { FileLogger } from "react-native-file-logger"

export class Log {
    tag = ""

    constructor(tag: string) {
        this.tag = tag
    }

    debug(message: string, toFile: boolean = false) {
        const str = `${this.tag}: ${message}`

        console.debug(str)

        if (toFile) {
            FileLogger.debug(str)
        }
    }

    info(message: string, toFile: boolean = false) {
        const str = `${this.tag}: ${message}`

        console.info(str)

        if (toFile) {
            FileLogger.info(str)
        }
    }

    error(message: string, toFile: boolean = false) {
        const str = `${this.tag}: ${message}`

        console.error(str)

        if (toFile) {
            FileLogger.error(str)
        }
    }

    warn(message: string, toFile: boolean = false) {
        const str = `${this.tag}: ${message}`

        console.warn(str)

        if (toFile) {
            FileLogger.warn(str)
        }
    }
}
