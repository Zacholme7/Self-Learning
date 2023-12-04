#5.12: Manually managing a transaciton
import asyncio
import asyncpg
from asyncpg.transaction import Transaction

async def main():
    # Connect to the database
    connection = await asyncpg.connect(host='127.0.0.1',
                                      port=5432,
                                      user='postgres',
                                      database='products',
                                      password='password')

    # start a transaction
    transaction: Transaction = connection.transaction()
    await transaction.start() # start the transaction

    try:
        # duplicate insert statements
        await connection.execute("INSERT INTO brand VALUES(DEFAULT, 'brand_1')")
        await connection.execute("INSERT INTO brand VALUES(DEFAULT, 'brand_2')")
    except asyncpg.PostgresError:
        print("Errors, rolling back transactions")
        # there was an error, we want to undo the insert
        await transaction.rollback()
    else:
        # there was no errors, commit the transaction
        print("no errors, committing transaction")
        await transaction.commit()

    # fetch
    query = "SELECT brand_name FROM brand WHERE brand_name LIKE 'brand%'"
    brands = await connection.fetch(query)

    print(brands)

    await connection.close()

asyncio.run(main())






