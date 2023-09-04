import { editor } from "monaco-editor";

const initialCode = `
addi x1, x0, 1
loop:
  add x1, x1, x1
  beq x0, x0, loop
`.trimStart();


let monaco: editor.IStandaloneCodeEditor;
let currentCode = initialCode;

export function create(parent: HTMLElement) {
  monaco = editor.create(parent, {});

  monaco.setValue(initialCode)

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