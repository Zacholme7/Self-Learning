# 9.6: A WSGI application
def application(env, start_response):
    start_response("200 OK", [("Content-Type", "text/html")])
    return [b"WSGI hello"]

