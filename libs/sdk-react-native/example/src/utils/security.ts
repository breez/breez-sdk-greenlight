const RE = /\w/g;

export const obfuscateString = (text: string): string => {
    return text.replace(RE, "*")
}
