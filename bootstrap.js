// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./pkg/potential.js")
    .then(module => {
        module.run_app();
        componentHandler.upgradeDom();
    })
    .catch(e => console.error("Error importing `./pkg`:", e))
