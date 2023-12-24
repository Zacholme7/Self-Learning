from aiohttp import web
from datetime import datetime
from aiohttp.web_request import Request
from aiohttp.web_response import Response

routes = web.RouteTableDef()

# create a get endpoint
# when a client calls this endpoint, the time coroutine will run
@routes.get("/time")
async def time(request: Request) -> Response:
    today = datetime.today()

    result = {
        "month": today.month,
        "day": today.day,
        "time": str(today.time())
    }

    return web.json_response(result)

# create the web application
app = web.Application()
# register the routes
app.add_routes(routes)
# run the application
web.run_app(app)

