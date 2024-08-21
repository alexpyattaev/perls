//g++ -c -std=c++20 auto.cc
//Types are for suckers and losers!
auto add(auto a, auto b) {return a+b;}

#include <iostream>

int main()
{
add(4,5);
add(6.5, 4.3);
auto x = add(4, 3.2);
std::cout<<x<<std::endl;
}
