console.log('Hello, World!');

const showPost = (event) => {//인자값에 바인드 이름 명시해 줘야 오류 안나더라
    const cell = event.target;//이밴트 처리
    const text = cell.textContent;//태그에 텍스트를 가저 온다
    console.log(text);
    alert(text);
};

document.querySelectorAll('.hello world').forEach(function(cell) {//클레스 이름 띄어쓰가로 구분 가능 하고 띄어쓰기 전부적으면 중복으로 실행 된다
    cell.addEventListener('click', showPost);
});

document.querySelectorAll('.asd').forEach(function(cell) {
    cell.addEventListener('click', showPost);
});