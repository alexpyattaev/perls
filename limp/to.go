package main
import "fmt"
func main() {
    learnGoTo()
}
func learnGoTo() {
    fmt.Println("a")
    goto FINISH
    fmt.Println("b")
FINISH:
    fmt.Println("c")
}
