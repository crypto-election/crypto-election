const merge = require('webpack-merge');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const {
  mergeStrategy, htmlOptions, pickCfg, ...commonCfg
} = require('./webpack.common.js');


const mergeClient = merge.smartStrategy(mergeStrategy.client);

const client = mergeClient(commonCfg.client, {
  mode: 'production',
  module: {
    rules: [
      {
        test: /\.(sa|sc|c)ss$/,
        use: [
          MiniCssExtractPlugin.loader,
        ]
      },
    ],
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: 'style.css'
    }),
    new HtmlWebpackPlugin(htmlOptions),
  ]
});

const server = merge(commonCfg.server, { mode: 'production' });


module.exports = pickCfg(client, server);
