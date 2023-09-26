fn main() {
    let numbers = vec![1,2,3,4,5];

    println!("0번 {:?}", numbers);
    // let mut plus_one = vec![];
    // for num in numbers{
    //     plus_one.push(num + 1);
    // }
    // println!("{:?}", plus_one);

    let plus_one:Vec<_> = numbers
        .iter() //백터나 리스트의 값을 1개씩 호출할떄 쓴다
        .map(|num| num + 1)//백터나 리스트의 값을 연산 할때 쓴다
        .collect();//반복자 iter() 자채로는 아무일도 하지 않기 때문에 콜랙터 collect()로 값들을 벡터로 변환한다
    println!("1번 {:?}", plus_one);

    let new_numbers: Vec<_> = numbers
        .iter()
        .filter(|num| num <= &&3)//특정 조건에 관여 하여 걸러주는 함수
        .collect();
    println!("2번 {:?}", new_numbers);

    let find_me:Option<i32> = numbers //찾는 값이 없는 겨우도 있기 때문에 옵션타입을 부과하여 none 값을 대응토록 한다
        .iter()
        .find(|num| num == &&2)//찾는 값이 없는 경우 none 반환
        .copied();//복사 할때 사용
    println!("3번 {:?}", find_me);
    println!("{}\n", find_me.unwrap());

    let count =numbers
        .iter()
        .count();//백터나 리스트의 값으 총 갯수 확인할때
    println!("4번 {}", count);

    let last: Option<i32> = numbers
        .iter()
        .last()
        .copied();
    println!("5번 {:?}\n", last);
    println!("{}", last.unwrap());

    let numbers1 = vec![10,2,3,4,5];
    let min:Option<&i32> = numbers1
        .iter()
        .min();//배열중 가장 작은 배열의 값 (배열의 순서와는 상관없다!)
    println!("6번 {:?}", min);
    let max:Option<&i32> = numbers1
        .iter()
        .max();//배령중 가장 큰 배열의 값 (배열의 순서와는 상관없다!)
    println!("7번 {:?}", max);

    let numbers2 = vec![5,6,7,8,9];
    let take: Vec<_> = numbers2
        .iter()
        .take(3) //뒤에서 3개의 원소만 가저 온다
        .copied() //위의 참조를 전부 복사 하여 분리된 데이터로 만든다
        .collect();
    println!("8번 {:?}", take);

    let numbers2 = vec![5,6,7,8,9];
    let take: Vec<_> = numbers2
        .iter()
        .take(numbers.len() - 3) //맨끝을 재외 하고 원하는 배열 범위 가저올때 
        .copied() //위의 참조를 전부 복사 하여 분리된 데이터로 만든다
        .collect();
    println!("9번 {:?}", take);
    
    let numbers2 = vec![5,6,7,8,9];
    let take: Vec<&_> = numbers2//참조를 하고자 할때 &정의 해야 한다
        .iter()
        .take(3)//뒤에서 3개의 원소만 가저 온다
        .collect();
    println!("10번 {:?}", take);
}