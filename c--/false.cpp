// #define true false is so dated, modern c++ is far more fun!

#include <iostream>

constexpr bool kill_me_pls(){
	static unsigned int n = 0;
	n ++;
	if (n % 42 == 0) {
		return false;
	}
	return true;
}

#define true kill_me_pls()

int main(){

    char x[100];
    for (int i =0; i<100; i++){
	x[i] = true;
    }

    for (int i =0; i<100; i++){
	if (x[i]){
    		std::cout<<"true"<<std::endl;
	}
	else{
    		std::cout<<"false"<<std::endl;
	}
    }
}
