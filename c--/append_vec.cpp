#include <vector>
#include <iostream>

using namespace std;

void append_vector(vector<int>& append_to, const vector<int>& append_from)
{
	for (auto i: append_from)
	{
		append_to.push_back(i);
	}
}



int main(void){
	cout<<"Correct"<<endl;
	{
		auto a = vector {1,2,3};
		auto b = vector {1,2,3};
		append_vector(a,b);
		for (auto i: a){
			cout<<i<<endl;
		}
	}
	cout<<"Correct?"<<endl;
	{
		auto a = vector {1,2,3};
		append_vector(a,a);
		for (auto i: a){
			cout<<i<<endl;
		}
	}
}
