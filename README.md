# CubeEngine
CubeEngine is the Rust library implementation for a proposed websocket-based network protocol to connect to cube-based sandbox game servers via a static webpage.

The goal of this project is to bring multiplayer sandbox games further into the open-source world such that it is free (as in freedom, not just free-of-charge) for everyone.

Principles of this project include:
- The client is open-source under Affero GNU Public License 3.0. Everyone is free to modify and redistribute modifications, but redistributed modifications must be open-source in the spirit of Free Software.
- The client is hosted on GitHub Pages. There is no centralized vendor of the client; everyone can opt to stick to the same version. Any tracking scripts in the client must be opt-in. GitHub Pages is chosen as a vendor that can ensure that what is hosted is what is in the source code.
- Features are mostly implemented server-side. Instead of having a centralized source determining behaviour like movement speed, textures, etc., these should be provided server-side. The server side should power the user interface, and the client is mostly relaying the signals.

