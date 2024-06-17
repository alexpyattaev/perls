//build with g++ -std=c++20  serialize.cc -lfmt  
#include <string>
#include <fmt/format.h>

void serialize(bool)               { fmt::print("bool\n"); }
void serialize(int)                { fmt::print("int\n"); }
void serialize(std::string const&) { fmt::print("string const&\n"); }
void serialize(std::string_view)   { fmt::print("string_view\n"); }

enum Lel{Foo, Bar};

int main() {
    // what will this print?
    serialize("hello");
    // what about this one?
    serialize(Foo);
}

