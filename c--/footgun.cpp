void fun(int* a, int*b){
	// Make sure a does not point to null, if so set b to 42
	if (a==nullptr){
		*b = 42;
	}
	*a += 1;

}



int main(){
	int a=0;
	fun(&a,&a);
	// should return 43, right?
	return a;
}
