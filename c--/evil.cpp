//Compile with clang++ -O3 evil.cpp
#include <cstdlib>

using FUN=void();


static  FUN* fun_ptr=nullptr;

void fun(){
   system("echo Evil!");
}

void set(){
   fun_ptr = fun;
}


int main(){
	fun_ptr();
	return 0;
}
