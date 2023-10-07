fn main() {
    let context_lines = 2;
    let needle ="oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with book.
What do we seek
through millions of pages?";

    let mut tags:Vec<usize> = Vec::new();//tags에 일치하는 행번호 저장 하는 비어 있는 백터(배열)를 생성한다
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();//ctx는 일치하는 항목마다 문맥 앞뒤 행들을 저장하는 비어있는 벡터를 생성한다

    for (i, line) in haystack.lines().enumerate(){ //lines()한줄식 반환한다 enumerate()1줄을 반환 할때마다 0부터 차래로 숫자를 기입한다 
        // println!("{}", line.contains(needle));
        if line.contains(needle){//반환된 줄에서 needle일치가 있는지 확인한다 (흠... bedroom 이랑 book 라인이 일치 할탠대 oo 면 ) 5: It is the same with book. 2: bedroom window, public-house, and  따로 나와야 하는대 뭘 잘 못햇지?
            println!("{}: {}", i, line);
            tags.push(i);//일치하는 항목의 번호를 저장 한다

            let v = Vec::with_capacity(2*context_lines + 1);//Vec::with_capacity(n)은 n개 의 항목을 위한 공간 예약한다 명시적인 타입 시그너처는 필요하지 않다 14행에서 ctx의 정의를 토대로 유추할수 있기때문이다.
            ctx.push(v);//할당된 용량을 입력 하다
        }
    }
    println!();
    println!();
    println!("{:?}", tags);
    println!();
    println!();
    if tags.is_empty() {//일치하는 항목이 없으면 끝낸다
        return;
    }

    for (i, line) in haystack.lines().enumerate(){//모든 태그에 대해 매 행마다 해당 행이 일치하는 곳 근처인지 검사한다 해당하는 경우라면 ctx안에 있는 Vec<T>에 그 행을 추가한다
        println!("{}: {}", i, line);
        for (j, tag) in tags.iter().enumerate(){//일치하는 번호를 나열하면 나열된수를 0부터 카운트를 센다
            println!("{}: {}", j, tag);
            let lower_bound = tag.saturating_sub(context_lines);//.saturating_sub()는 뺄셈을 정할때 정수가 0보다 작아지면 프로그램을 강재종료하는 대신 0안에 있는 Vec<T>에 그행을 추가한다
            let upper_bound = tag + context_lines;
            
            if (i >= lower_bound) && (i <= upper_bound){//전혀 걸러내질 못하는대...
                println!("lower_bound {} upper_bound {}", lower_bound, upper_bound);
                let line_as_string = String::from(line);//해당 행을 새로운 String으로 복사해서 일치할 때마다 지역 변수에 저장한다.
                println!("line_as_string ->>>>>>>>>{}", line_as_string);
                let local_ctx = (i, line_as_string);//일치하는 항목과 번호를 튜플로 저장 한다
            
                ctx[j].push(local_ctx);//해당 태그에 local_ctx튜플을 저장한다
            }
        }
    }
    println!();
    for local_ctx in ctx.iter(){//ctx안의 튜플로 저장되어진 값을 1개씩 꺼넨다
        for &(i, ref line) in local_ctx.iter(){ //***ref*** 은 컴파일러에 이 값을 이동하는대신 대여하려 한다고 알린다, local_ctx 튜플로 된 데이터를 꺼넨다
           
            let line_num = i + 1;//튜플 라인넘버에 + 1 해준다
            println!("{}: {}", line_num, line);
        }
    }
}