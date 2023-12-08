# 6.8: Parallel MapReduce with process pools
import asyncio
import concurrent.futures
import functools
import time
from typing import Dict, List

# parititon the data up
def partition(data: List, chunk_size: int) -> List:
    for i in range(0, len(data), chunk_size):
        yield data[i:i + chunk_size]

# map the words
def map_frequencies(chunk: List[str]) -> Dict[str, int]:
    counter = {}
    for line in chunk:
        word, _, count, _ = line.split('\t')
        # update the counter 
        if counter.get(word):
            counter[word] = counter[word] + int(count)
        else:
            counter[word] = int(count)
    return counter

def merge_dictionaries(first: Dict[str, int], second: Dict[str, int]) -> Dict[str, int]:
    # merge the dictionaries
    merged = first
    for key in second:
        if key in merged:
            merged[key] = merged[key] + second[key]
        else:
            merged[key] = second[key]
    return merged

async def main(partition_size: int):
    with open('googlebooks-eng-all-1gram-20120701-a', encoding='utf-8') as f:
        # read all the lines
        contents = f.readlines()

        loop = asyncio.get_running_loop()
        tasks = []

        # start the time
        start = time.time()

        # make our process pool
        with concurrent.futures.ProcessPoolExecutor() as pool:
            # parition into chunks
            for chunk in partition(contents, partition_size):
                # construt our tasks
                tasks.append(loop.run_in_executor(pool, functools.partial(map_frequencies, chunk)))
            # wait for all the mpa operations to complete
            intermediate_results = await asyncio.gather(*tasks)
            # reduce all the results
            final_result = functools.reduce(merge_dictionarires, intermediate_results)

            print(f"Aardvark has appered {final_result['Aardvark']} times")

            # end the timer
            end = time.time()
            print(f"MapReduce took: {(end - start):.4f} seconds")

if __name__ == "__main__":
    asyncio.run(main(partition_size = 60000))









