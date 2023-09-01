export function fitToParent(parent, monaco) {
    let observer = new ResizeObserver((entries) => {
        for (const entry of entries) {
            let {width, height} = entry.contentRect;
            monaco.layout({width, height})
        }
    })

    observer.observe(parent);
}