struct S{
	const int& m;
};

int main(){
	const S& s = S{1};
	return s.m;
}
