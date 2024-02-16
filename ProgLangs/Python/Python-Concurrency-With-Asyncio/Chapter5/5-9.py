# 5.9: Creating a transaction
import asyncio
import asyncpg

async def main():
    # Connect to the database
    connection = await asyncpg.connect(host='127.0.0.1',
                                      port=5432,
                                      user='postgres',
                                      database='products',
                                      password='password')

    # start a database transaction
    async with connection.transaction():
        await connection.execute("INSERT INTO brand "
        "VALUES(DEFAULT, 'brand_1')")
        await connection.execute("INSERT INTO brand "
        "VALUES(DEFAULT, 'brand_2')")

    query = """SELECT brand_name FROM brand
    WHERE brand_name LIKE 'brand%'"""

    # select brands to ensure that our transaction was committed
    brands = await connection.fetch(query)
    print(brands)

    await connection.close()

asyncio.run(main())



