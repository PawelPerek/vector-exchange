import { editor, languages } from "monaco-editor";
import { monarchDefinition } from "./monarch";

const scalarCode = `
addi x1, x0, 1
loop:
  add x1, x1, x1
  beq x0, x0, loop
`.trimStart();

const vectorCode =`
# void
# saxpy(size_t n, const float a, const float *x, float *y)
# {
#   size_t i;
#   for (i=0; i<n; i++)
#     y[i] = a * x[i] + y[i];
# }
#
# register arguments:
#     a0      n
#     fa0     a
#     a1      x
#     a2      y

saxpy:
  vsetvli a4, a0, e32, m8, ta, ma
  vle32.v v0, (a1)
  sub a0, a0, a4
  slli a4, a4, 2
  add a1, a1, a4
  vle32.v v8, (a2)
  vfmacc.vf v8, fa0, v0
  vse32.v v8, (a2)
  add a2, a2, a4
  bnez a0, saxpy
  ret`.trimStart();

const RISCV = "risc-v";

languages.setMonarchTokensProvider(RISCV, monarchDefinition);
languages.register({
  id: RISCV,
  extensions: ["S"]
})

let monaco: editor.IStandaloneCodeEditor;
let currentCode = scalarCode;

export function create(parent: HTMLElement) {
  console.log(languages.getLanguages().map(language => language.id).includes(RISCV))
  monaco = editor.create(parent, {
    value: currentCode,
    language: RISCV,
    fontSize: 18,
    theme: "vs-dark"
  });

  let observer = new ResizeObserver((entries) => {
    for (const entry of entries) {
      let { width, height } = entry.contentRect;
      monaco.layout({ width, height })
    }
  })

  observer.observe(parent);
}

export function onInput(listener: (value: string) => void) {
  listener(currentCode);
  monaco.getModel().onDidChangeContent(_ => {
    let code = monaco.getValue();
    currentCode = code;
    listener(code)
  });
}