"""I/O bound and CPU bound operations"""
import requests

response = requests.get("http://example.com") # this is I/o bound, we have to wait for the request to complete before moving on
items = response.headers.items()

headers = [f"{key}: {header}" for key, header in items] # this is cpu bound, we have to wait for the cpu to compute before moving on

with open("header.txt", "w") as f:
    f.write(headers)  # i/o bound write to dis
