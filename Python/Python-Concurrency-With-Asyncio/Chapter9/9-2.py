# 9.2: Connecting to a product database
import asyncpg
from aiohttp import web
from aiohttp.web_app import Application
from aiohttp.web_request import Request
from aiohttp.web_response import Response
from asyncpg import Record
from asyncpg.pool import Pool
from typing import List, Dict

routes = web.RouteTableDef()
DB_KEY = "database"

async def create_database_pool(app: Application):
    print("Creating database pool")
    pool: Pool = await asyncpg.create_pool(host = "127.0.0.1",
                                           port = 5432,
                                           user = "postgres",
                                           password = "password",
                                           database = "products",
                                           min_size = 6, 
                                           max_size = 6)
    # set our database pool on the applicaiton
    app[DB_KEY] = pool

# destroy the pool in the application instance
async def destroy_database_pool(app: Application):
    print("Destroying database pool")
    pool: Pool = app[DB_KEY]
    await pool.close()

# query all brands and return results to the client
@routes.get("/brands")
async def brands(request: Request) -> Response:
    # get the pool from through the request.app value
    connection: Pool = request.app[DB_KEY]
    # query the database
    brand_query = "SELECT brand_id, brand_name FROM brand"
    results: List[Record] = await connection.fetch(brand_query)
    # convert results
    result_as_dict: List[Dict] = [dict(brand) for brand in results]
    return web.json_response(result_as_dict)


app = web.Application()
# register the startup and cleanup handlers
app.on_startup.append(create_database_pool)
app.on_cleanup.append(destroy_database_pool)

app.add_routes(routes)
web.run_app(app)


