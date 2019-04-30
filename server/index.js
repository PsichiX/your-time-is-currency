#!/usr/bin/env node

const ws = require('ws'),
  namegen = require('node-random-name'),
  port = 8090,
  version = 0,
  server = new ws.Server({ port }),
  clients = [],
  spawnRange = 400,
  initLifetime = 60,
  MSG_INIT_PLAYER = 1,
  MSG_NEW_PLAYER = 2,
  MSG_PLAYER_STATE = 3,
  MSG_PLAYER_DISCONNECTED = 4,
  MSG_PLAYER_EAT = 5;
var gid = 1;

console.log('* Listening on port: ' + port);

class ID {
  constructor(id) {
    this.id = id;
  }
}

server.on('connection', function(socket) {
  const pos = findInitPosition();
  const client = {
    socket,
    id: new ID(gid++),
    name: namegen(),
    time: initLifetime,
    alive: true,
    x: pos[0],
    y: pos[1],
  };
  socket.on('error', console.error);
  socket.on('close', function() {
    console.log('* Client disconnected: ' + client.name);
    client.alive = false;
    clearTimeout(client.timeoutInit);
    client.timeoutInit = null;
    clearInterval(client.intervalNew);
    client.intervalNew = null;
    clearInterval(client.intervalCollision);
    client.intervalCollision = null;
    sendAll(writeMessage(MSG_PLAYER_DISCONNECTED, [client.id]));
    clients.splice(clients.indexOf(client), 1);
  });
  socket.on('message', function(msg) {
    if (msg instanceof Buffer) {
      try {
        const ver = msg.readUInt32BE(4);
        if (ver === version) {
          const mid = msg.readUInt32BE(0);
          if (mid === MSG_PLAYER_STATE) {
            client.alive = true;
            client.time = msg.readFloatBE(12);
            client.x = msg.readFloatBE(16);
            client.y = msg.readFloatBE(20);
            if (client.time <= 0) {
              killClient(client);
            } else {
              sendAll(msg, client);
            }
          }
        }
      } catch (err) {
        console.error(err);
      }
    }
  });

  send(client, writeMessage(
    MSG_INIT_PLAYER,
    [client.id, client.name, client.time, client.x, client.y]
  ));
  clients.push(client);

  client.timeoutInit = setTimeout(function() {
    for (var i = 0; i < clients.length; ++i) {
      const other = clients[i];
      if (other !== client) {
        send(client, writeMessage(
          MSG_NEW_PLAYER,
          [other.id, other.name, other.time, other.x, other.y]
        ));
      }
    }
    sendAll(writeMessage(
      MSG_NEW_PLAYER,
      [client.id, client.name, client.time, client.x, client.y]
    ), client);
  }, 1000);
  console.log('* Client connected: ' + client.name);

  client.intervalNew = setInterval(function() {
    if (client.alive && client.time > 0) {
      sendAll(writeMessage(
        MSG_NEW_PLAYER,
        [client.id, client.name, client.time, client.x, client.y]
      ), client);
    }
  }, 1000);

  client.intervalCollision = setInterval(function() {
    if (client.alive && client.time > 0) {
      for (var i = 0; i < clients.length; ++i) {
        const other = clients[i];
        if (client.id.id !== other.id.id && playersCollide(client, other)) {
          if (client.time > other.time) {
            send(client, writeMessage(MSG_PLAYER_EAT, [other.time]));
            killPlayer(other);
          } else {
            send(other, writeMessage(MSG_PLAYER_EAT, [client.time]));
            killPlayer(client);
          }
          return;
        }
      }
    }
  }, 100);
});

// setInterval(function() {
//   console.log('CLIENTS: ' + clients.length + ' | ' + clients.map(function(c) {
//     return c.id.id;
//   }).join());
// }, 1000);

function findInitPosition() {
  if (clients.length > 0) {
    const nearby = clients[(Math.random() * clients.length | 0) % clients.length];
    var tries = 5;
    while (tries-- > 0) {
      const dir = Math.random() * Math.PI * 2;
      const pos = {
        x: nearby.x + Math.cos(dir) * spawnRange,
        y: nearby.y + Math.sin(dir) * spawnRange,
      };
      if (playersCollide(pos, nearby)) {
        continue;
      }
      return [pos.x, pos.y];
    }
  } else {
    return [0, 0];
  }
}

function playersCollide(client, other) {
  const dx = client.x - other.x;
  const dy = client.y - other.y;
  return dx * dx + dy * dy <= 10000; // 100 * 100
}

function killPlayer(client) {
  try {
    client.socket.terminate();
  } catch (err) {
    console.error(err);
  }
}

function writeHeader(id) {
  const buffer = Buffer.alloc(8);
  buffer.writeUInt32BE(id, 0);
  buffer.writeUInt32BE(version, 4);
  return buffer;
}

function writeID(value) {
  const buffer = Buffer.alloc(4);
  buffer.writeUInt32BE(value.id, 0);
  return buffer;
}

function writeNumber(value) {
  const buffer = Buffer.alloc(4);
  buffer.writeFloatBE(value, 0);
  return buffer;
}

function writeInteger(value) {
  const buffer = Buffer.alloc(4);
  buffer.writeUInt32BE(value, 0);
  return buffer;
}

function writeString(value) {
  const buffer = Buffer.from(value);
  return Buffer.concat([writeInteger(buffer.length), buffer]);
}

function writeMessage(id, data) {
  const parts = [writeHeader(id)];
  for (var i = 0; i < data.length; ++i) {
    const item = data[i];
    if (item instanceof Buffer) {
      parts.push(item);
    } else if (item instanceof ID) {
      parts.push(writeID(item));
    } else if (typeof item === 'number') {
      parts.push(writeNumber(item));
    } else if (typeof item === 'string') {
      parts.push(writeString(item));
    }
  }
  return Buffer.concat(parts);
}

function send(client, msg) {
  try {
    if (client.alive && client.socket.readyState == 1) {
      client.socket.send(msg);
    }
  } catch (err) {
    console.error(err);
  }
}

function sendAll(msg, exclude) {
  for (var i = 0; i < clients.length; ++i) {
    const client = clients[i];
    if (client !== exclude) {
      send(client, msg);
    }
  }
}
