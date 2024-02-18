#include <iostream>

template<typename T>            
T fuckme (T t){ 
return t + 42;
}                                   
                                                                                                 
int main (){       
 {
	 float x = fuckme(4);
	 std::cout << "looks legit "<< x<< std::endl;
 }
 {
	 auto x = fuckme(4.2);
	 std::cout << "looks legit? "<< x<< std::endl;
 }
 {
	 auto x = fuckme("abcdef");
	 std::cout << "trololo "<< x<< std::endl;
 }
 return  fuckme(69.69);
}  
