interface Header {
  name: string;
  value: string;
}

export class HttpClient {
  async get(url: string) {
    return this.sendData("GET", url);
  }

  async post(url: string, body: any) {
    let headers: Header[] = [];
    if (typeof body == "object") {
      body = JSON.stringify(body);
      headers.push({
        name: "Content-Type",
        value: "application/json"
      });
    }

    return this.sendData("POST", url, body, headers);
  }

  private sendData(method: string, url: string, body?: string, headers: Header[] = []) {
    return new Promise<{ content: any }>((resolve, reject) => {
      let requester = new XMLHttpRequest();
      requester.withCredentials = true;
      requester.onreadystatechange = () => {
        if (requester.readyState === XMLHttpRequest.DONE) {
          if (requester.status === 200 || requester.status === 204) {
            resolve({
              content: this.parseResponse(requester)
            });
          } else {
            reject();
          }
        }
      };

      requester.open(method, url, true);
      headers.forEach((header) => {
        requester.setRequestHeader(header.name, header.value);
      });
      requester.send(body);
    });
  }

  private parseResponse(requester: XMLHttpRequest) {
    if (requester.getResponseHeader("Content-Type") === "application/json") {
      return JSON.parse(requester.responseText);
    } else {
      return requester.responseText;
    }
  }
}