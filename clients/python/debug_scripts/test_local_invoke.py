#!/usr/bin/env python3
import httpx
from httpx_sse import connect_sse
import sys
import json

uri = "http://localhost:8000"

def invoke(message: str):
    with httpx.Client() as client:
        with connect_sse(client, "POST", f"{uri}/invoke", data=json.dumps({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "user",
                    "content": message
                }
            ]
        }), headers={
            "Content-Type": "application/json"
        }) as event_source:
            for sse in event_source.iter_sse():
                print(sse.event, sse.data, sse.id, sse.retry)

if __name__ == "__main__":
    invoke(sys.argv[1])
