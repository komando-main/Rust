console.log('Hello, World!');

const showPost = (event) => {
    const cell = event.target;
    const text = cell.textContent;
    console.log(text);
    alert(text);
};

document.querySelectorAll('.hello').forEach(function(cell) {
    cell.addEventListener('click', showPost);
});