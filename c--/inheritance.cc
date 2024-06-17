//Clang will compile this, g++ will not.

struct Base {
protected:
    void foo();
};

struct Derived : public Base {
private:
    using Base::foo;
};

struct TestingDerived : public Derived {
public:
    using Base::foo;
};


