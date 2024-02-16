# 5.6: Inserting random products and SKUs
import asyncio
import asyncpg
from random import randint, sample
from typing import List, Tuple

# load the words from the file
def load_common_words() -> List[str]:
    with open("common_words.txt") as common_words:
        return common_words.readlines()

# generate random products
def gen_products(common_words: List[str], brand_id_start: int, brand_id_end: int, products_to_create: int) -> List[Tuple[str, int]]:
    products = []
    for _ in range(products_to_create):
        description = [common_words[index] for index in sample(range(1000), 10)]
        brand_id = randint(brand_id_start, brand_id_end)
        products.append((" ".join(description), brand_id))
    return products

# generate random dkus
def gen_skus(product_id_start: int, product_id_end: int, skus_to_create: int) -> List[Tuple[int, int, int]]:
    skus = []
    for _ in range(skus_to_create):
        product_id = randint(product_id_start, product_id_end)
        size_id = randint(1, 3)
        color_id = randint(1, 2)
        skus.append((product_id, size_id, color_id))
    return skus

async def main():
    # load the common words
    common_words = load_common_words()

    # Connect to the database
    connection = await asyncpg.connect(host='127.0.0.1',
                                      port=5432,
                                      user='postgres',
                                      database='products',
                                      password='password')

    # generate all the products that we want to insert
    product_tuples = gen_products(common_words,brand_id_start = 1,brand_id_end = 100,products_to_create = 1000)
    # insert everything into products tapel
    await connection.executemany("INSERT INTO product VALUES(DEFAULT, $1, $2)", product_tuples)

    # generate all the skus
    sku_tuples = gen_skus(product_id_start = 1, product_id_end = 1000, skus_to_create = 100000)
    # insert the skus into the talbe
    await connection.executemany("INSERT INTO sku VALUES(DEFAULT, $1, $2, $3)", sku_tuples)

    # close the connection
    await connection.close()

asyncio.run(main())
                        



