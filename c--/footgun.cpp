
void fun(int* a, int*b){
if (a==nullptr){
	*b = 42;
}
*a = 0;

}



int main(){
	int a=777;
	fun(&a,&a);
	return a;
}
