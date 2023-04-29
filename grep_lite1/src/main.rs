fn main() {
    let search_term = "늑대";
    let quote ="\
    오늘도 하루를 시작하며 내가 가고자 하는 길
    하고 싶은 일 하고 싶다고 전부 할 수는 없으나 도전하고자 하는
    사람의 열정으로 야생 늑대 처럼 혼자 버티고 박살나고 망가저도
    살아 남아서 그저 남들 하는만큼 만 먹고살수 있으면 좋은대
    삶이 팍팍하고 실력이 없으니 무시만 당하고 조롱당하고 욕먹고
    뭐 다 내탓이려니 하고 버티는 방법뿐이 없군";

    let mut line_num: usize = 1;

    for line in quote.lines(){
        if line.contains(search_term) {
            println!("{}: {}", line_num, line);
        }
        line_num+=1;
    }
}