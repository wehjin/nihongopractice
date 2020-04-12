// A dependency graph that contains any wasm must all be imported
// asynchronously. This `index.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./pkg/potential.js")
    .then(module => {
        module.run_app();
        componentHandler.upgradeDom();
    })
    .catch(console.error);
