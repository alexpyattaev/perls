


fn main() {
    let (tx,rx) = std::sync::mpsc::channel();
    tx.send(1).unwrap();
    tx.send(2).unwrap();
    tx.send(3).unwrap();
    drop(tx);
    for i in 0..10{
        let res = rx.recv();
        dbg!(res);
    }
//    drop(rx);
//        let res = rx.recv();
/*                c <- 1
                c <- 2
                c <- 3
                close(c)
                for i := 0; i < 10; i++ {
                                fmt.Printf("%d ", <-c)
                }

                c = nil
                fmt.Printf("%d ", <-c)
*/
}

