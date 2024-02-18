#include <iostream>

void unreachable()
{
	std::cout <<"Goodbye, sanity!"<<std::endl;
}

int main(){
    while(1);
    unreachable();
}

