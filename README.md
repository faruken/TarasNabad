# TarasNabad
[WIP] Multi threaded Key/Value storage in Rust.

### Supported Actions
- GET
- SET
- DELETE

### Example

Simple Python client example to communicate with TarasNabad.

```
import json
import urllib.request

def set_value(value: str) -> str:
  req = urllib.request.Request(url='http://127.0.0.1:8000/', data=value.encode(), method="put", headers={"Content-Type": "application/json"})
  resp = urllib.request.urlopen(req)
  return resp.read().decode()


def get_value(key: str) -> dict:
  req = urllib.request.Request(url=f'http://127.0.0.1:8000/{key}', method="get", headers={"Content-Type": "application/json"})
  resp = urllib.request.urlopen(req)
  return json.loads(resp.read().decode())


def delete_value(key: str) -> str:
  req = urllib.request.Request(url=f'http://127.0.0.1:8000/{key}', method="delete", headers={"Content-Type": "application/json"})
  resp = urllib.request.urlopen(req)
  return resp.read().decode()

```
