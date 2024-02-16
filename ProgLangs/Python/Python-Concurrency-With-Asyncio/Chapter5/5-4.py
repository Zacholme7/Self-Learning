# 5.4: Inserting and selecting brands 
import asyncpg
import asyncio
from asyncpg import Record
from typing import List

async def main():
    # Connect to the database
    connection = await asyncpg.connect(host='127.0.0.1',
                                      port=5432,
                                      user='postgres',
                                      database='products',
                                      password='password')

    # insert information
    await connection.execute("INSERT INTO brand VALUES(DEFAULT, 'Levis')")
    await connection.execute("INSERT INTO brand VALUES(DEFAULT, 'Seven')")

    # select the brands
    brand_query = "SELECT brand_id, brand_name FROM brand"
    results: List[Record] = await connection.fetch(brand_query)

    for brand in results:
        print(f"id: {brand['brand_id']}, name: {brand['brand_name']}")

    # close the connection
    await connection.close()

asyncio.run(main())
