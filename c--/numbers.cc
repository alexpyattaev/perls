// run on a 64-bit host
#include<assert.h>
#include<stdlib.h>
#include<string>

int main (void){
	// stoul interprets an unsigned integer value in the string str. 
	assert(std::stoul("-1") == std::stoul("18446744073709551615"));
	
	// as does strtoul from C stdlib.
	assert(strtoul("-1", nullptr, 10) == strtoul("18446744073709551615", nullptr, 10));
}
