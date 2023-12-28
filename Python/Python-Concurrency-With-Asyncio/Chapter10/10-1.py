import asyncio
import random
from aiohttp import web
from aiohttp.web_response import Response

# routing table
routes = web.RouteTableDef()

# define a new round
@routes.get('/products/{id}/inventory')
async def get_inventory(request: Request) -> Response:
    delay: int = random.randint(0, 5)
    await asyncio.sleep(delay)
    inventory: int = random.randint(0, 100)
    return web.json_response({'inventory': inventory}) # returnt he response

# start the application
app = web.Application()
app.add_routes(routes)
web.run_app(app, port=8001)
