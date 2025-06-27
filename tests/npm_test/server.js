const http = require("node:http");
const fs = require("node:fs");
const path = require("node:path");

const port = 8080;
const projectRoot = path.join(__dirname, '..');

// MIME types
const mimeTypes = {
  '.html': 'text/html',
  '.js': 'application/javascript',
  '.css': 'text/css',
  '.json': 'application/json',
  '.wasm': 'application/wasm',
  '.docx': 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  '.d.ts': 'text/typescript'
};

const server = http.createServer((req, res) => {
  console.log(`${new Date().toISOString()} - ${req.method} ${req.url}`);
  
  // CORS headers for local development
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
  
  if (req.method === 'OPTIONS') {
    res.writeHead(204);
    res.end();
    return;
  }
  
  let filePath = req.url === '/' ? '/tests/npm_test/browser_test_npm.html' : req.url;
  filePath = path.join(projectRoot, filePath);
  
  // Security check
  if (!filePath.startsWith(projectRoot)) {
    res.writeHead(403);
    res.end('Forbidden');
    return;
  }
  
  const extname = path.extname(filePath).toLowerCase();
  const contentType = mimeTypes[extname] || 'application/octet-stream';
  
  fs.readFile(filePath, (err, content) => {
    if (err) {
      if (err.code === 'ENOENT') {
        res.writeHead(404);
        res.end(`File not found: ${req.url}`);
      } else {
        res.writeHead(500);
        res.end(`Server error: ${err.code}`);
      }
    } else {
      res.writeHead(200, { 'Content-Type': contentType });
      res.end(content);
    }
  });
});

server.listen(port, () => {
  console.log(`🚀 Server running at http://localhost:${port}`);
  console.log(`📄 Browser test: http://localhost:${port}/tests/npm_test/browser_test_npm.html`);
  console.log(`📁 Project root: ${projectRoot}`);
});

server.on('error', (err) => {
  console.error('Server error:', err);
});
