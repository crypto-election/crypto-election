const merge = require('webpack-merge');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const {
  mergeStrategy, htmlOptions, pickCfg, ...commonCfg
} = require('./webpack.common.js');


const mergeClient = merge.smartStrategy(mergeStrategy.client);

const client = mergeClient(commonCfg.client, {
  mode: 'development',
  devtool: "inline-source-map",
  devServer: {
    contentBase: './front/dist',
  },
  module: {
    rules: [
      {
        test: /\.(sa|sc|c)ss$/,
        use: [
          'vue-style-loader',
        ]
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin(htmlOptions),
  ]
});

const server = merge(commonCfg.server, { mode: 'development' });


module.exports = pickCfg(client, server);
