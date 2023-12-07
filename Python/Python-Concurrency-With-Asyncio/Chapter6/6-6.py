# 6.6: Single threaded mapreduce
import functools
from typing import Dict

def map_frequency(text: str) -> Dict[str, int]:
    words = text.split(" ") # split all the words
    frequencies = {}
    for word in words:
        if word in frequencies:
            # increment count for this word
            frequencies[word] = frequencies[word] + 1
        else:
            # if we have not seen this word yet, set the count to 1
            frequencies[word] = 1
    return frequencies

def merge_dictionaries(first: Dict[str, int], second: Dict[str, int]) -> Dict[str, int]:
    merged = first

    # merge all the keys in the second
    for key in second:
        if key in merged:
            # combine frequncies if in both dicts
            merged[key] = merged[key] + second[key]
        else:
            # ifw e have not seen it yet, copy frequency over
            merged[key] = second[key]
    return merged


lines = ["I know what I know",
         "I know that I know",
         "I don't know much",
         "They don't know much"]

# get the frequency for each line
mapped_results = [map_frequency(line) for line in lines]

for result in mapped_results:
    print(result)

# reduce all our intermediate frequency counts into one result
print(functools.reduce(merge_dictionaries, mapped_results))


