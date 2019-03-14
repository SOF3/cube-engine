#!/bin/bash

# cube-engine
#
# Copyright (C) 2019 SOFe
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

set -e

cd `dirname "$0"`

npm install -g typescript ts-node
NODE_ENV=development npm install

PORT=8765
ts-node protocolTest server $PORT &
SERVER_PID=$!
sleep 1
ts-node protocolTest client $PORT &
CLIENT_PID=$1
wait $SERVER_PID $CLIENT_PID

echo OK, protocolTest.ts seems to be working correctly
