import express from 'express';
import proxy from 'http-proxy-middleware';
import parseArgs from 'yargs-parser';

// Initialize application
const app = express();

// Get app params
const argv = parseArgs(process.argv.slice(2));
const port = argv.port;
const apiRoot = argv.apiRoot;

if (typeof port === 'undefined') {
  throw new Error('--port parameter is not set.');
}

if (typeof apiRoot === 'undefined') {
  throw new Error('--api-root parameter is not set.');
}

// Set path to static files
app.use(express.static(__dirname + '/front'));

// Proxy middleware options
const apiProxy = proxy({
  target: apiRoot,
  ws: true,
  headers: {
    'Origin': 'http://localhost'
  }
});

app.use('/api', apiProxy);

app.listen(port);
