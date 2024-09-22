const allowedWebBranches = input('allowedWebBranches', '');

const aaa = JSON.parse(allowedWebBranches);
console.log(aaa);

function input(name, def) {
  let inp = core.getInput(name).trim();
  if (inp === '' || inp.toLowerCase() === 'false') {
    return def;
  }

  return inp;
}
