#include <iostream>


struct B{
int x;
};

struct A{
int x;

A();
};

A::A(){
	  std::cout<<"LOL"<<std::endl;
}

void readb(B &b){
	std::cout<<b.x<<std::endl;
}

int main(){

	A a1;
	std::cout<<a1.x<<std::endl;
	A a2 = A();
	std::cout<<a2.x<<std::endl;
	B b1;
//	std::cout<<b1.x<<std::endl; //avoid the warning by not reading b1 directly
	readb(b1);
	B b2 = B();
	std::cout<<b2.x<<std::endl;
	return 0;
}
