import requests


r = requests.post("http://localhost:8080/api/messages", json={
    "content": "Hello, world!",
    "id": "2323123",
    "clean_content": "Hello, world!"
})
print(r.text)