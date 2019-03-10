"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : new P(function (resolve) { resolve(result.value); }).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
var fs = require("fs");
var http = require("http");
var path = require("path");
var process = require("process");
var ws = require("websocket");
var zlib = require("zlib");
var input = fs.readFileSync(path.join(__dirname, "test.txt"), { encoding: "utf8" }).split("\n");
var ByteStream = /** @class */ (function () {
    function ByteStream() {
        this.buffer = Buffer.alloc(16);
        this.offset = 0;
    }
    ByteStream.prototype.put = function (bytes, start, end) {
        if (start === void 0) { start = 0; }
        if (end === void 0) { end = bytes.length; }
        if (this.offset + end - start > this.buffer.length) {
            var length_1 = this.buffer.length;
            while (this.offset + end - start > length_1) {
                length_1 *= 2;
            }
            var newBuffer = Buffer.alloc(length_1);
            this.buffer.copy(newBuffer, 0, 0, this.offset);
            this.buffer = newBuffer;
        }
        bytes.copy(this.buffer, this.offset, start, end);
        this.offset += end - start;
    };
    ByteStream.prototype.getBuffer = function () {
        return this.buffer.slice(0, this.offset);
    };
    return ByteStream;
}());
var tests = [];
var currentTest;
var currentStep;
while (input.length > 0) {
    var line = input.shift().trim();
    if (line[0] === ";" || line.length === 0) {
        continue;
    }
    if (line.match(/^\.test\b/)) {
        currentTest = {
            name: line.substr(6),
            steps: [],
        };
        tests.push(currentTest);
        currentStep = undefined;
        continue;
    }
    if (line[0] === "<" || line[0] === ">") {
        currentStep = {
            fromServer: line[0] === "<",
            buffer: new ByteStream(),
        };
        currentTest.steps.push(currentStep);
        line = line.substr(1);
    }
    if (currentStep === undefined) {
        throw "Unexpected data line before step declaration on line " + line;
    }
    scanLine(line, 0, currentStep.buffer);
}
function scanLine(line, start, to) {
    for (var i = start; i < line.length; i++) {
        if (line[i] === "}") {
            return i;
        }
        if (line[i + 1] === "{") {
            switch (line[i]) {
                case "Z": {
                    var stream = new ByteStream();
                    i = scanLine(line, i + 2, stream) - 1;
                    var d = zlib.deflateSync(stream.getBuffer());
                    to.put(d);
                    continue;
                }
                case "#": {
                    var _a = readLiteral(line, i + 2), literal = _a.literal, newI = _a.i;
                    i = newI;
                    to.put(Buffer.from(literal, "utf8"));
                    continue;
                }
                case "F": {
                    var _b = readLiteral(line, i + 2), literal = _b.literal, newI = _b.i;
                    i = newI;
                    to.buffer.writeFloatBE(parseFloat(literal), to.offset);
                    to.offset += 4;
                    continue;
                }
                case "D": {
                    var _c = readLiteral(line, i + 2), literal = _c.literal, newI = _c.i;
                    i = newI;
                    to.buffer.writeDoubleBE(parseFloat(literal), to.offset);
                    to.offset += 8;
                    continue;
                }
            }
        }
        if (line[i] === "," || line[i] === " " || line[i] === "\t") {
            continue;
        }
        if (line.substr(i, 2).match(/[0-9a-fA-F]{2}/)) {
            var buf = Buffer.from(line.substr(i, 2), "hex");
            to.put(buf);
            i++;
            continue;
        }
        throw "Unexpected token " + line.substr(i);
    }
    return line.length;
}
function readLiteral(line, offset) {
    var literal = "";
    var i = offset;
    for (; i < line.length; i++) {
        if (line[i] === "}") {
            break;
        }
        if (line[i] === "\\") {
            i++;
        }
        literal += line[i];
    }
    return { literal: literal, i: i };
}
function createTestServer(resolve, reject, testId, port, host) {
    if (host === void 0) { host = "127.0.0.1"; }
    var httpServer = http.createServer();
    httpServer.listen(port, host, function () {
        var server = new ws.server({
            httpServer: httpServer,
            autoAcceptConnections: false,
        });
        server.on("request", function (request) {
            var conn = request.accept("cube-pump");
            performTests(conn, testId, true)
                .then(function () { return new Promise(function (resolve) { return httpServer.close(resolve); }); })
                .then(resolve)["catch"](reject);
        });
    });
}
function createTestClient(resolve, reject, testId, port, host) {
    if (host === void 0) { host = "127.0.0.1"; }
    var address = "ws://" + host + ":" + port;
    var client = new ws.client({});
    client.on("connect", function (conn) {
        performTests(conn, testId, false)
            .then(function () { return conn.close(); })
            .then(resolve)["catch"](reject);
    });
    client.connect(address, "cube-pump");
}
function performTests(socket, testId, isServer) {
    return __awaiter(this, void 0, void 0, function () {
        function receiveMessage(receive) {
            if (messageQueue.length !== 0) {
                receive(messageQueue.shift());
            }
            else {
                messageReceiveQueue.push(receive);
            }
        }
        var test, messageQueue, messageReceiveQueue, _loop_2, i;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    test = tests[testId];
                    console.log("[" + testId + "] Testing " + test.name);
                    messageQueue = [];
                    messageReceiveQueue = [];
                    socket.on("message", function (message) {
                        if (messageReceiveQueue.length !== 0) {
                            messageReceiveQueue.shift()(message);
                        }
                        else {
                            messageQueue.push(message);
                        }
                    });
                    _loop_2 = function (i) {
                        var step;
                        return __generator(this, function (_a) {
                            switch (_a.label) {
                                case 0:
                                    step = test.steps[i];
                                    if (!(step.fromServer === isServer)) return [3 /*break*/, 1];
                                    socket.sendBytes(step.buffer.getBuffer());
                                    return [3 /*break*/, 3];
                                case 1: return [4 /*yield*/, new Promise(function (resolve, reject) {
                                        receiveMessage(function (message) {
                                            if (message.type !== "binary") {
                                                return reject("Expected binary message");
                                            }
                                            if (!message.binaryData.equals(step.buffer.getBuffer())) {
                                                return reject("Incorrect data.\nExpected: " + step.buffer.getBuffer().toString("hex") + "\nActual: " + message.binaryData.toString("hex"));
                                            }
                                            resolve();
                                        });
                                    })];
                                case 2:
                                    _a.sent();
                                    _a.label = 3;
                                case 3: return [2 /*return*/];
                            }
                        });
                    };
                    i = 0;
                    _a.label = 1;
                case 1:
                    if (!(i < test.steps.length)) return [3 /*break*/, 4];
                    return [5 /*yield**/, _loop_2(i)];
                case 2:
                    _a.sent();
                    _a.label = 3;
                case 3:
                    i++;
                    return [3 /*break*/, 1];
                case 4: return [2 /*return*/];
            }
        });
    });
}
var port = parseInt(process.argv[3] || "8765");
if (!(1024 <= port && port <= 65535)) {
    throw "Usage: node protocolTest (server|client) <port>";
}
if (process.argv[2] === "server" || process.argv[2] === "client") {
    var promises = [];
    var _loop_1 = function (testId) {
        var promise = new Promise(function (resolve, reject) {
            if (process.argv[2] === "server") {
                createTestServer(resolve, reject, testId, port + testId, process.argv[4] || "127.0.0.1");
            }
            else {
                createTestClient(resolve, reject, testId, port + testId, process.argv[4] || "127.0.0.1");
            }
        });
        promises.push(promise.then(function () {
            console.log("[" + testId + "] Test passed");
        })["catch"](function (err) {
            console.error("[" + testId + "] Test failed: " + err);
            return Promise.reject(err);
        }));
    };
    for (var testId = 0; testId < tests.length; testId++) {
        _loop_1(testId);
    }
    Promise.all(promises).then(function () {
        console.log("All tests passed");
        process.exit(0);
    });
}
else {
    throw "Usage: node protocolTest (server|client) <port> [host]";
}
