const path = require('path');
const htmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: path.resolve(__dirname, "src/bootstrap.tsx"),
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "bootstrap.js"
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: "ts-loader"
      },
    ]
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js"]
  },
  devServer: {
    contentBase: path.resolve(__dirname, "public"),
    port: 8080,
    host: '0.0.0.0',
    disableHostCheck: true
  },
  plugins: [
    new htmlWebpackPlugin({ template: './src/index.html' }),
  ],
  experiments: {
    asyncWebAssembly: true
  }
};