const path = require('path')
const { VueLoaderPlugin } = require('vue-loader')
require('babel-polyfill')

const argParser = require("yargs-parser");


module.exports.client = {
  target: 'web',
  entry: {
    build: [
      'babel-polyfill',
      './src/app',
    // ],
    // libs: [
      'bootstrap',
      'jquery',
      'popper.js',
    ],
  },
  output: {
    path: path.resolve(__dirname, 'dist/front'),
    filename: '[name].js',
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        use: [ 'babel-loader' ],
      },
      {
        test: /\.vue$/,
        use: [ 'vue-loader' ],
      },
      {
        test: /\.(sa|sc|c)ss$/,
        use: [
          'css-loader',
          'sass-loader',
        ]
      },
      {
        test: /\.(jpe?g|gif|png)$/,
        use: 'file-loader',
      },
      {
        test: require.resolve('jquery'),
        use: [{
          loader: 'expose-loader',
          options: 'jQuery'
        },{
          loader: 'expose-loader',
          options: '$'
        }]
      },
    ]
  },
  plugins: [
    new VueLoaderPlugin(),
  ],
};

module.exports.server = {
    target: 'node',
    entry: './src/server.js',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: 'server.js'            
    },
    node: { __dirname: false },
};

module.exports.mergeStrategy = {
  client: { "module.rules.use": "prepend" }
};

module.exports.htmlOptions = {
  inject: false,
  template: require('html-webpack-template'),

  appMountId: 'app',
  lang: 'ru',
  title: 'Cryptoelection demo',
  mobile: true,
  scripts: [
    'build.js',
  ],
};

module.exports.pickCfg = function pickCfg(clientCfg, serverCfg) {
  const { clientOnly, serverOnly } = argParser(process.argv.slice(2));

  if (serverOnly && clientOnly)
    throw TypeError("--server-only and --client-only combining not allowed!");
  
  if (clientOnly) return clientCfg;
  if (serverOnly) return serverCfg;
  return [ clientCfg, serverCfg ]
};
