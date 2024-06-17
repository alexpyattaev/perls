template <typename T>
void answer(T a) {
	// this is copy, right?
	a = 42;
}

int main(void){
    int a = 0;
    answer<int&>(a);
    return a;
}
