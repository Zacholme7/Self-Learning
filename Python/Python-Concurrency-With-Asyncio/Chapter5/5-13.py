# 5.13: A synchronous generator

def positive_integers(until: int):
    for integer in range(until):
        yield integer

# returns a <class generator>
positive_iterator = positive_integers(2)


# can also use for loop
print(next(positive_iterator))
print(next(positive_iterator))
