const http = require('http');

const hostname = '127.0.0.1';
const port = 8888;

const server = http.createServer((req, res) => {
  let body = '';

  req.on('data', chunk => {
    body += chunk.toString();
  });

  req.on('end', () => {
    console.log('Request method:', req.method);
    console.log('Request URL:', req.url);
    console.log('Request headers:', req.headers);
    console.log('Request body:', body);

    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/plain');
    res.end(body);
  });
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
