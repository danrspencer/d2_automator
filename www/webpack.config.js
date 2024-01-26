const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const fs = require('fs');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
  devServer: {

    compress: true,
    port: 8080, // You can specify your port here
    https: {
      key: fs.readFileSync(path.join(__dirname, 'localhost.key')), // Path to your key
      cert: fs.readFileSync(path.join(__dirname, 'localhost.crt')), // Path to your cert
    },
    open: true // Open browser after server had been started
  },
};
