// Watch this leak your memorys!
#include <vector>

struct V : std::vector<V> {};

int main(){
	V v;
	v.emplace_back();
	v.swap(v.front());
}
