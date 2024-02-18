#include <new>
#include <vector>
using namespace std;
int main()
{
	size_t n = 14ll*1000*1000*1000/8;
	try{
	std::vector<size_t> x;
	x.reserve(n);
	for (size_t i = 0; i<n; i++){
		x.push_back(i);
	}
	}
	catch (std::bad_alloc) {
	return 1;
	}
	return 0;

}
