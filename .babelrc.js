module.exports = (api) => {
  api.cache(false)
  return {
    "presets": [
      ["@babel/env", {
        "targets": {
          "node": "current"
        }
      }]
    ],
    "plugins": [require.resolve("./src/babel-plugin/ident-counter.js")]
  }
}