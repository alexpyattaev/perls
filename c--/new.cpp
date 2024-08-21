#include <new>
#include <vector>
#include <iostream>
using namespace std;
int main(int argc, char** argv)
{
	cout<<"Running this will likely trigger OOM conditions, your system may freeze/crash\n";
	cout<<"How many GB of available RAM do you have?\n";

	size_t n;
	cin >> n;
	n = n + 1;
	cout<<"Trying to reserve " << n << " GB of memory"<<endl;
	n = n * 1024*1024*1024;
	try{
		std::vector<unsigned char> x;
		x.reserve(n);
		cout<<"Alloc ok, lets fill it with zeros..."<<endl;
		for (size_t i = 0; i<n; i++){
			x.push_back(i);
		}
	}
	catch (std::bad_alloc) {
		cout<<"Got bad alloc!";
		return 1;
	}
	cout<<"Program finished without exceptions";
	return 0;

}
