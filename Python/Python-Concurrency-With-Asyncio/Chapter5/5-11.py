# 5.11: A nested transaction
import asyncio
import asyncpg
import logging

async def main():
    # Connect to the database
    connection = await asyncpg.connect(host='127.0.0.1',
                                      port=5432,
                                      user='postgres',
                                      database='products',
                                      password='password')

    # start the transaction
    async with connection.transaction():
        # execute the insert statement
        await connection.execute("INSERT INTO brand VALUES(DEFAULT,'my_new_brand')")

        try:
            # start another transaction
            async with connection.transaction():
                # this should fail since it is a duplicate key error
                await connection.execute("INSERT INTO brand VALUES(DEFAULT,'my_new_brand')")
        except Exception as ex:
            logging.warning('Ignoring error inserting product color',exc_info=ex)
    await connection.close()

asyncio.run(main())




