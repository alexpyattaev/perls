function* count(){
	yield 1;
	yield 2;
	return 3;
}

for (const v of count()){
console.log(v);
}
