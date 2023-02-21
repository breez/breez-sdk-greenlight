import SecureStorage from "react-native-secure-storage"

const secureConfig = {}

export const setSecureItem = async (key: string, item: any): Promise<void> => {
    return await SecureStorage.setItem(key, JSON.stringify(item), secureConfig)
}

export const getSecureItem = async (key: string): Promise<any | null> => {
    const item = await SecureStorage.getItem(key, secureConfig)
    return item && JSON.parse(item)
}
