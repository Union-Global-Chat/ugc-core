import websockets
import asyncio
import json


async def main():
    async with websockets.connect('ws://localhost:8080/api/ws') as ws:
        async for message in ws:
            print(message)
            data = json.loads(message)
            if data["type"] == "hello":
                await ws.send(json.dumps({
                    "type": "login",
                    "data": {
                        "token": "123"
                    }
                }))

asyncio.run(main())