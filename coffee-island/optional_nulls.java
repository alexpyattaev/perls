//run with java optional_nulls.java
class HelloWorld {
    
    public static java.util.Optional<Integer> is_even(Integer x){
	 return null;
	 //return java.util.Optional.empty();
	 //return java.util.Optional.of(x);
    }

    public static void main(String[] args) {
        System.out.println(is_even(42).isPresent()); 
    }
}
