def generator():
    yield 1
    yield 2
    return 3

for i in generator():
    print(i)
