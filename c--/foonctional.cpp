#include <functional>

struct Foo {};
Foo get_foo();
void with_foo(std::function<Foo const&()>);

int main() {
    // gcc >=13 refuses to compile this
    // clang is happy with either libstdc++ or libc++
    // msvc at least warns
    with_foo(get_foo);
}

