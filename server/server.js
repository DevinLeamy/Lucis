let express = require('express');
let app = express();
 
express.static.mime.types['wasm'] = 'application/wasm';
 
app.use((_request, response, next) => {
    response.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
    response.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
    // response.setHeader('Content-Security-Policy', '');

    next()
})

console.log(process.cwd() + "./../dist")
app.use(express.static(process.cwd() + './../dist'));
app.listen(8080, () => {
    console.log("Listening on port 8080")
});
