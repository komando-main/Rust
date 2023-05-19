console.log('Hello, World!');
alert("안녕 세상");
const showPost = (event) => {//인자값에 바인드 이름 명시해 줘야 오류 안나더라
    const cell = event.target;//이밴트 처리
    const text = cell.textContent;//태그에 텍스트를 가저 온다
    console.log(text);
    alert(text);
    const data = { text: text };

    // POST 요청 보내기
    fetch('/json', {//  /json 경로로 POST 요청
        method: 'POST',//'POST'를 지정하여 요청
        headers: {
            'Content-Type': 'application/json'// 요청 본문의 타입을 JSON으로 지정
        },
        body: JSON.stringify(data)// JSON 형식의 데이터를 지정
    })
    .then(response => response.json())//래스폰스 방식을 json 으로 설정한다
    .then((data) => {
                console.log(data);
                return data; //{"text":"클릭 값"} ex){"text":"래몬"} 저장 하면 새로고침해줘야 한다
                // return text; // "클릭 값" ex)"래몬"
            }
        )// 콘솔에 표현
    .then((data) => {
        // HTML 요소 생성
        const mainDiv = document.createElement('div');
        mainDiv.innerHTML = JSON.stringify(data); //return 위에서 리턴한 방법에따라 div가 위와 동일하게 생성된다 

        // body에 요소 추가
        document.body.appendChild(mainDiv);//현재 패이지에 추가
    })
    .catch(error => console.error(error));
};

document.querySelectorAll('.hello world').forEach(function(cell) {//클레스 이름 띄어쓰기로 구분 가능
    cell.addEventListener('click', showPost);//이밴트 사용 1번인자(어떤이밴트인지) 2번인자(뭘 사용 할지)
});