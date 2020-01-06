"use strict";

import autocannon, { track } from "autocannon";

const body = JSON.stringify({
  query: "{\n  authors {\n    id\n    name\n    md5\n    books {\n      id\n      name\n    }\n  }\n}"
});

function runAutoCannon() {
  const result = autocannon({
    url: "http://localhost:8080/graphql",
    method: "POST",
    body,
    connections: 5,
    duration: 5,
    headers: {
      "content-type": "application/json"
    }
  });
  track(result, {
    renderProgressBar: true,
    renderLatencyTable: false,
    renderResultsTable: true
  });
}
runAutoCannon();