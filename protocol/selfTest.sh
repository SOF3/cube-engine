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

ts-node protocolTest server 8765 &
SERVER_PID=$!
ts-node protocolTest client 8765
wait $SERVER_PID
