#include <iostream>

int f(bool x) noexcept 
{
	if (x){
		return 42;
	}
	std::cout<<"x is "<<x <<std::endl;
	return 1/x;
}


int main(int argc, char**argv){
	try{
	    return f( argc > 1);
	    std::cout<<"all good!"<<std::endl;
	}
	catch (int){
	    std::cout<<"Oh no!"<<std::endl;
	}
}
