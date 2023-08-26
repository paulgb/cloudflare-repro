import WebSocket from 'ws';
const ws = new WebSocket('ws://127.0.0.1:8787/');

ws.on('message', (d) => {
  console.log('received:', d)
});
