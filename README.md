# CubeEngine
CubeEngine is the Rust library implementation for CubePump.

### What is CubePump?
CubePump is a WebSocket-based protocol to allow clients to connect to cube-based sandbox game servers by visiting a static webpage.

### A client-oriented game protocol
The CubePump protocol specification mainly describes the client, not the server.

The CubePump client is designed to reflect user interactions to the server and display data from the server, virtually a "cube world browser" rather than a game itself. (You can play web games using Google Chrome, but Google Chrome is not a game itself). CubePump/CubeEngine is not responsible for implementing new game features so as to prevent catastrophic updates from affecting all players.

### AGPL-3.0 licensed
CubeEngine and CubePump are open-source under GNU Affero General Public License 3.0. In particular, Everyone is free to modify and redistribute modifications, but **redistributed modifications must be open-source in the spirit of Free Software**. This does not apply to libraries using CubeEngine.

### Hosted on GitHub Pages
The client is hosted on GitHub Pages to ensure that the client is entirely open-source with no hidden code.

### Non-affiliation
CubePump is in no way affiliated with Minecraft, Mojang AB, Microsoft or other commercial game developers. While CubePump is mostly inspired by Minecraft servers, there is no official and direct compatibility between Minecraft and CubePump.

While the author of CubePump is part of the PMMP Team, this project is no way affiliated with PMMP either.

## Protocol documentation
The protocol is documented using TOML format in the file [protocol/spec.toml](protocol/spec.toml). 
