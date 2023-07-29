fn main() {
    let numbers = vec![1,2,3,4,5];

    // let mut plus_one = vec![];
    // for num in numbers{
    //     plus_one.push(num + 1);
    // }
    // println!("{:?}", plus_one);

    let plus_one:Vec<_> = numbers
        .iter() //백터나 리스트의 값을 1개씩 호출할떄 쓴다
        .map(|num| num + 1)//백터나 리스트의 값을 연산 할때 쓴다
        .collect();//반복자 iter() 자채로는 아무일도 하지 않기 때문에 콜랙터 collect()로 값들을 벡터로 변환한다
    println!("{:?}", plus_one);
}
