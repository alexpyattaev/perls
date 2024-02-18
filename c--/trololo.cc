#include <vector>
#include <cstdio>

void foo(std::vector<int> boo)
{
    for (auto i : boo)
    {
        switch (i)
        {
            case 1: 
                goto label;
                break;
                
            case 2: 
		printf("%d", i);
                break;
            default: 
		throw(i);
        }
        
    }
    label: 
	printf("done\n");
}

int main(void)
{
    std::vector<int>  x = {1,2,3};
    foo(x);
}
