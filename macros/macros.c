#undef BOO
#include "macros1.h"
#include "macros2.h"


#define FUUU(a) a + X


float main (){
        float x =  FUUU(5);
	#undef float
	float z = 4.3;
	return x;
}
