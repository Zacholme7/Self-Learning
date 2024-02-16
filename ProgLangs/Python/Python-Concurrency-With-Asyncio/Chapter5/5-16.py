# 5.16: Moving the cursor and fetching records
import asyncpg
import asyncio

async def main():
    # Connect to the database
    connection = await asyncpg.connect(host='127.0.0.1',
                                      port=5432,
                                      user='postgres',
                                      database='products',
                                      password='password')

    # start a transaction
    async with connection.transaction():
        query = 'SELECT product_id, product_name from product'
        # create a cursor for the query
        cursor = await connection.cursor(query)
        await cursor.forward(500) # move it forward 500
        products = await cursor.fetch(100) # get next 100 records

        for product in products:
            print(product)

    await connection.close()

asyncio.run(main())

