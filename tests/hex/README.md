# Hex tests
This directory contains hex-encoded exchange buffers intended for integration tests. Each tests/hex/*.txt file contains one valid conversation between client and server. Implementations of cube-pump protocol should be able to produce identical result based on given configurations.

## Syntax
### Step
A "step" is one websocket binary message. Each step starts with a `<` (server-to-client) or `>` (client-to-server), followed by buffer data on the same line or following lines until the next step is encountered.

### Buffer data
Buffer data are bytes encoded in hexadecimal nibble pairs. There are some special patterns to improve readability of the buffer data:

#### UTF-8 literals
`#{...}` encodes the content inside as a UTF-8 payload. No padding are allowed, i.e. `#{ B }` would be encoded as `20 42 20` instead of just `42`. No length prefix would be written implicitly. Special characters like `\` and `}`can be escaped by putting a `\` in front, e.g. `#{\}}` is encoded as `7d`.

#### Float literals
`F{...}`/`D{...}` encode literal decimal numbers using IEEE-754 binary32/binary64 standards respectively.

### Includes
A line that starts with `+` would work like a cpp `#include` line. `+ login` would be replaced with the contents inside `tests/hex/lib/login.txt`.
