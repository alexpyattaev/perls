// compile with g++ -O3 true.cpp
//
int table[4];

bool find(int v)
{
	for (int i=0;i<=4;i++)
	{
		if (v==table[i])
		{
			return true;
		}
	}
	return false;
}

#include <cstdio>

int main(){

	table[0] = 1;
	table[1] = 1;
	table[2] = 1;
	table[3] = 1;


	printf("Result is %d \n",find(42));
	printf("Result is %d \n",find(666));
	return 0;
}
