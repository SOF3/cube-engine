import * as fs from "fs"
import * as http from "http"
import * as path from "path"
import * as process from "process"
import * as ws from "websocket"
import * as zlib from "zlib"

let input = fs.readFileSync(path.join(__dirname, "test.txt"), {encoding: "utf8"}).split("\n")

interface Test{
	name: string
	steps: Step[]
}

interface Step{
	fromServer: boolean
	buffer: ByteStream
}

class ByteStream{
	buffer = Buffer.alloc(16)
	offset = 0

	put(bytes: Buffer, start = 0, end = bytes.length){
		if(this.offset + end - start > this.buffer.length){
			let length = this.buffer.length
			while(this.offset + end - start > length){
				length *= 2
			}
			const newBuffer = Buffer.alloc(length)
			this.buffer.copy(newBuffer, 0, 0, this.offset)
			this.buffer = newBuffer
		}
		bytes.copy(this.buffer, this.offset, start, end)
		this.offset += end - start
	}

	getBuffer(){
		return this.buffer.slice(0, this.offset)
	}
}

const tests = [] as Test[]
let currentTest: Test
let currentStep: Step

while(input.length > 0){
	let line = input.shift().trim()
	if(line[0] === ";" || line.length === 0){
		continue
	}
	if(line.match(/^\.test\b/)){
		currentTest = {
			name: line.substr(6),
			steps: [],
		}
		tests.push(currentTest)
		currentStep = undefined
		continue
	}
	if(line[0] === "<" || line[0] === ">"){
		currentStep = {
			fromServer: line[0] === "<",
			buffer: new ByteStream(),
		}
		currentTest.steps.push(currentStep)
		line = line.substr(1)
	}

	if(currentStep === undefined){
		throw "Unexpected data line before step declaration on line " + line
	}

	scanLine(line, 0, currentStep.buffer)
}

function scanLine(line: string, start: number, to: ByteStream){
	for(let i = start; i < line.length; i++){
		if(line[i] === "}"){
			return i
		}
		if(line[i + 1] === "{"){
			switch(line[i]){
				case "Z":{
					const stream = new ByteStream()
					i = scanLine(line, i + 2, stream) - 1
					const d = zlib.deflateSync(stream.getBuffer())
					to.put(d)
					continue
				}
				case "#":{
					const {literal, i: newI} = readLiteral(line, i + 2)
					i = newI
					to.put(Buffer.from(literal, "utf8"))
					continue
				}
				case "F":{
					const {literal, i: newI} = readLiteral(line, i + 2)
					i = newI
					to.buffer.writeFloatBE(parseFloat(literal), to.offset)
					to.offset += 4
					continue
				}
				case "D":{
					const {literal, i: newI} = readLiteral(line, i + 2)
					i = newI
					to.buffer.writeDoubleBE(parseFloat(literal), to.offset)
					to.offset += 8
					continue
				}
			}
		}
		if(line[i] === "," || line[i] === " " || line[i] === "\t"){
			continue
		}
		if(line.substr(i, 2).match(/[0-9a-fA-F]{2}/)){
			const buf = Buffer.from(line.substr(i, 2), "hex")
			to.put(buf)
			i++
			continue
		}
		throw `Unexpected token ${line.substr(i)}`
	}
	return line.length
}

function readLiteral(line: string, offset: number){
	let literal = ""
	let i = offset
	for(; i < line.length; i++){
		if(line[i] === "}"){
			break
		}
		if(line[i] === "\\"){
			i++
		}
		literal += line[i]
	}
	return {literal, i}
}

function createTestServer(resolve: () => void, reject: (err: any) => void, testId: number, port: number, host = "127.0.0.1"){
	const httpServer = http.createServer()
	httpServer.listen(port, host, () => {
		const server = new ws.server({
			httpServer: httpServer,
			autoAcceptConnections: false,
		})
		server.on("request", request => {
			const conn = request.accept("cube-pump")
			performTests(conn, testId, true)
				.then(() => new Promise(resolve => httpServer.close(resolve)))
				.then(resolve)
				.catch(reject)
		})
	})
}

function createTestClient(resolve: () => void, reject: (err: any) => void, testId: number, port: number, host = "127.0.0.1"){
	const address = `ws://${host}:${port}`
	const client = new ws.client({})
	client.on("connect", conn => {
		performTests(conn, testId, false)
			.then(() => conn.close())
			.then(resolve)
			.catch(reject)
	})
	client.connect(address, "cube-pump")
}

async function performTests(socket: ws.connection, testId: number, isServer: boolean){
	const test = tests[testId]
	console.log(`[${testId}] Testing ${test.name}`)

	const messageQueue = [] as ws.IMessage[]
	const messageReceiveQueue = [] as ((message: ws.IMessage) => void)[]
	socket.on("message", message => {
		if(messageReceiveQueue.length !== 0){
			messageReceiveQueue.shift()(message)
		}else{
			messageQueue.push(message)
		}
	})

	function receiveMessage(receive: (message: ws.IMessage) => void){
		if(messageQueue.length !== 0){
			receive(messageQueue.shift())
		}else{
			messageReceiveQueue.push(receive)
		}
	}

	for(let i = 0; i < test.steps.length; i++){
		const step = test.steps[i]
		if(step.fromServer === isServer){
			socket.sendBytes(step.buffer.getBuffer())
		}else{
			await new Promise((resolve, reject) => {
				receiveMessage(message => {
					if(message.type !== "binary"){
						return reject("Expected binary message")
					}
					if(!message.binaryData.equals(step.buffer.getBuffer())){
						return reject(`Incorrect data.\nExpected: ${step.buffer.getBuffer().toString("hex")}\nActual: ${message.binaryData.toString("hex")}`)
					}
					resolve()
				})
			})
		}
	}
}

const port = parseInt(process.argv[3] || "8765")
if(!(1024 <= port && port <= 65535)){
	throw "Usage: node protocolTest (server|client) <port>"
}
if(process.argv[2] === "server" || process.argv[2] === "client"){
	const promises = [] as Promise<any>[]
	for(let testId = 0; testId < tests.length; testId++){
		const promise = new Promise((resolve, reject) => {
			if(process.argv[2] === "server"){
				createTestServer(resolve, reject, testId, port + testId, process.argv[4] || "127.0.0.1")
			}else{
				createTestClient(resolve, reject, testId, port + testId, process.argv[4] || "127.0.0.1")
			}
		})
		promises.push(promise.then(() => {
			console.log(`[${testId}] Test passed`)
		}).catch(err => {
			console.error(`[${testId}] Test failed: ${err}`)
			return Promise.reject(err)
		}))
	}
	Promise.all(promises).then(() => {
		console.log("All tests passed")
		process.exit(0)
	})
}else{
	throw "Usage: node protocolTest (server|client) <port> [host]"
}
