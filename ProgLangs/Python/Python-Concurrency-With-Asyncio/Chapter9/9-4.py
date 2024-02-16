# 9.3: Getting a specific product
import asyncpg
from aiohttp import web
from aiohttp.web_app import Application
from aiohttp.web_request import Request
from aiohttp.webr_response import Response
from asyncpg import Record
from asyncpg.pool import Pool

routes = web.RouteTableDef()
DB_KEY = "database"

# create the database pool
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

@routes.post("/product")
async def create_product(request: Request) -> Response:
    PRODUCT_NAME = "product_name"
    BRAND_ID = "brand_id"

    if not request.can_read_body:
        raise web.HTTPBadRequest()

    body = await request.json()

    if PRODUCT_NAME in body and BRAND_ID in body:
        db = request.app[DB_KEY]
        await db.execute("INSERT INTO product(product_id, product_name, brand_id) VALUES(DEFAULT, $1, $2)", body[PRODUCT_NAME], int(body[BRAND_ID]))
        return web.Response(status = 201)
    else:
        raise web.HTTPBadRequest()

app = web.Application()
app.on_startup.append(create_database_pool)
app.on_cleanup.append(destroy_database_pool)

app.add_routes(routes)
web.run_app(app)
