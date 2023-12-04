# 5.15: Streaming results one by one
import asyncpg
import asyncio

async def main():
    # Connect to the database
    connection = await asyncpg.connect(host='127.0.0.1',
                                      port=5432,
                                      user='postgres',
                                      database='products',
                                      password='password')

    # query
    query = "SELECT product_id, product_name FROM product"

    # start the transaction
    async with connection.transaction():
        # stream them in with a cursor
        async for product in connection.cursor(query):
            print(product)
    await connection.close()

asyncio.run(main())
