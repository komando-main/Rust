fn main() {
    let mut data = Some(3);

    while let Some(i) = data {
        println!("loop {:?}", i);
        data = None;//무한 루프를 막기위한 방법
    }
    println!("{:?}", data);

    let numbers = vec![1, 2, 3];
    let mut number_iter = numbers.iter();//Iter([1, 2, 3])
    println!("{:?}", number_iter);
    while let Some(num) = number_iter.next() {//number_iter.next() 데이터를 1개씩 넘긴다 더이상 넘길 데이터가 없을경우 None 을 넘긴다
        println!("num = {:?}", num);
    }
    println!("{:?}", number_iter);//루프에서 값이 사용되어진 후 상태
    println!("loop 끝!");
}
