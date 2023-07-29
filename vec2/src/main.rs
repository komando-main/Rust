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

    let new_numbers: Vec<_> = numbers
        .iter()
        .filter(|num| num <= &&3)//특정 조건에 관여 하여 걸러주는 함수
        .collect();
    println!("{:?}", new_numbers);

    let find_me:Option<i32> = numbers //찾는 값이 없는 겨우도 있기 때문에 옵션타입을 부과하여 none 값을 대응토록 한다
        .iter()
        .find(|num| num == &&2)//찾는 값이 없는 경우 none 반환
        .copied();//복사 할때 사용
    println!("{:?}", find_me);
    println!("{}", find_me.unwrap());

    let count =numbers
        .iter()
        .count();//백터나 리스트의 값으 총 갯수 확인할때
    println!("{}", count);

    let last: Option<i32> = numbers
        .iter()
        .last()
        .copied();
    println!("{:?}", last);
    println!("{}", last.unwrap());

    let numbers1 = vec![1,2,3,4,5];
    let min:Option<&i32> = numbers1
        .iter()
        .min();
    println!("{:?}", min);
    let max:Option<&i32> = numbers1
        .iter()
        .max();
    println!("{:?}", max);

    let numbers2 = vec![5,6,7,8,9];
    let take: Vec<_> = numbers2
        .iter()
        .take(3) //앞에서 3개의 원소만 가저 온다
        .copied() //위의 참조를 전부 복사 하여 분리된 데이터로 만든다
        .collect();
    println!("{:?}", take);
    
    let numbers2 = vec![5,6,7,8,9];
    let take: Vec<&_> = numbers2
        .iter()
        .take(3)//앞에서 3개의 원소만 가저 온다
        .collect();
    println!("{:?}", take);
}