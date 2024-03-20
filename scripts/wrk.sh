wrk -c 40 -d 30s -t 4 http://localhost:3000/
wrk -c 40 -d 30s -t 4 'http://localhost:3000/id/1?name=bun'
wrk -c 40 -d 30s -t 4 -s body.lua http://localhost:3000/json
wrk -c 40 -d 30s -t 4 http://localhost:3000/ely.png
wrk -c 40 -d 30s -t 4 'http://localhost:3000/page.html?name=hello'
