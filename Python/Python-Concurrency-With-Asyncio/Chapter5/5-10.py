# 5.10: Handling an error in a transaction
import asyncio
import logging
import asyncpg

async def main():
    # Connect to the database
    connection = await asyncpg.connect(host='127.0.0.1',
                                      port=5432,
                                      user='postgres',
                                      database='products',
                                      password='password')

    try:
        async with connection.transaction():
            insert_brand = "INSERT INTO brand VALUES(9999, 'big_brand')"
            await connection.execute(insert_brand)
            await connection.execute(insert_brand) # will error because htere is a duplicate primary key
    except Exception: 
        logging.exception("Error while running transaction") # if we have exception, log the error
    finally: 
        query = """SELECT brand_name FROM brand
                   WHERE brand_name LIKE 'big_%'"""
        brands = await connection.fetch(query)
        print(f"Query result was: {brands}")

        await connection.close()

asyncio.run(main())

