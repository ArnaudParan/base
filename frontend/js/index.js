() => null

console.log(Elm)
console.log(window.document.getElementById('myapp'))

var app = Elm.Main.init({
  node: window.document.getElementById('myapp')
});
