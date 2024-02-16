# 5.5: Inserting random brands
import asyncpg
import asyncio
from typing import List, Tuple, Union
from random import sample

# load the words from the file
def load_common_words() -> List[str]:
    with open("common_words.txt") as common_words:
        return common_words.readlines()

# generate random brand names from the words
def generate_brand_names(words: List[str]) -> List[Tuple[Union[str, ]]]:
    return [(words[index],) for index in sample(range(100), 100)]

# insert the names into the database
async def insert_brands(common_words, connection) -> int:
    brands = generate_brand_names(common_words)
    insert_brands = "INSERT INTO brand VALUES(DEFAULT, $1)"
    return await connection.executemany(insert_brands, brands)

async def main():
    # load the words
    common_words = load_common_words()

    # Connect to the database
    connection = await asyncpg.connect(host='127.0.0.1',
                                      port=5432,
                                      user='postgres',
                                      database='products',
                                      password='password')

    # insert the data
    await insert_brands(common_words, connection)

asyncio.run(main())
