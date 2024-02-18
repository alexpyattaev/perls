class Countable(object):
    counter=0
    def __new__(cls, *args, **kwargs):
        #if 'counter' not in cls.__dict__:
        #    cls.counter = 0
        obj = object.__new__(cls, *args, **kwargs)
        cls.increment_counter()
        return

    @classmethod
    def increment_counter(cls):
        cls.counter += 1
        if issubclass(cls.__base__,Countable):
            cls.__base__.increment_counter()

class A(Countable):
    pass

class B(Countable):
    pass

class B2(B):
    pass



a = A()
a = A()
a = A()
b = B()
b = B()
a = A()
b2 = B2()
b2 = B2()


print('Countables: {}  As: {}  Bs: {}  B2s: {}'.format(Countable.counter, A.counter, B.counter, B2.counter))
