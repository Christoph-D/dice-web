import * as wasm from "dice-web";

wasm.set_panic_hook();

function update() {
    const spec = document.getElementById("spec");
    const result = document.getElementById("result");
    var results = []
    for (let i = 0; i < 5; ++i) {
        results.push(wasm.roll(spec.value));
    }
    result.innerHTML = results.join('<br>');
}

spec.oninput = update;
spec.onkeydown = function (e) {
    if (e.key === 'Enter') {
        update();
    }
}
update();

const reroll = document.getElementById("reroll");
reroll.onclick = update;
