module.exports = {
    presets: ["module:metro-react-native-babel-preset"],
    plugins: [
        [
            "module-resolver",
            {
                alias: {
                    stream: "stream-browserify",
                    buffer: "@craftzdog/react-native-buffer"
                }
            }
        ]
    ]
}
