# 5.1: Connecting to a Postgres database as the default user
import asyncpg
import asyncio

async def main():
    # connect to the database
    connection = await asyncpg.connect(host = "127.0.0.1",
                                       port = 5432,
                                       user = "postgres",
                                       database = "postgres",
                                       password = "password")
    # get the database version
    version = connection.get_server_version()
    print(f"Connected, Postgres version is {version}")
    # close the connection
    await connection.close()

asyncio.run(main())
