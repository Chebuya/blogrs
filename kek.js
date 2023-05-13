const http = require('http');

// Define username and password for basic authentication
const USERNAME = 'demo';
const PASSWORD = 'password';

// Create HTTP server
const server = http.createServer((req, res) => {
  // Parse basic authentication header
  const auth = req.headers.authorization;
  if (auth) {
    const [type, encoded] = auth.split(' ');
    if (type === 'Basic') {
      const [username, password] = Buffer.from(encoded, 'base64')
        .toString('utf-8')
        .split(':');
      if (username === USERNAME && password === PASSWORD) {
        // If the authentication is successful, show the secret message
        res.writeHead(200, { 'Content-Type': 'text/plain' });
        res.end('Secret message: You are authenticated!');
        return;
      }
    }
  }

  // Display basic authentication prompt
  res.setHeader('WWW-Authenticate', 'Basic realm="Access to the secret message"');
  res.writeHead(401);
  res.end();
});

// Start server
server.listen(3000, () => {
  console.log('Server is running on http://localhost:3000');
});
