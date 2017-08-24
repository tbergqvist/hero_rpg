import { GuiLib } from "./gui-lib";
import { HttpClient } from "./http-client";

enum Method {
  Get = "GET",
  Post = "POST"
}

interface Field {
  name: string;
  type: string;
  value: string | number;
}

interface GameAction {
  name: string;
  method: Method;
  link: string;
  fields: Field[];
}

interface GameResponse {
  message: string;
  actions: GameAction[];
}

interface Link {
  method: Method;
  link: string;
  body?: any;
}

export function createGui(game: GameResponse, followLink: (link: Link)=>void) {
  function requestNewState(event: Event, action: GameAction) {
    event.preventDefault();
    followLink(Object.assign(action, {
      body: action.fields.reduce((obj, field)=> {
        obj[field.name] = field.value;
        return obj;
      }, {})
    }));
  }

  let actions = game.actions.map((action) => {
    return (
    <form onsubmit={(e)=>requestNewState(e, action)}>
      {action.name}
      {action.fields.map((field)=> {
        return <input type={field.type} value={field.value} onchange={(event)=>field.value = event.target.value} />
      })}
      <button>Confirm</button>
    </form>
    );
  });

  let gui = {
    render() {
      return (
      <div>
        <span>{game.message}</span>
        <div>
          {actions}
        </div>
      </div>
      );
    },
  };
  return gui;
}

function start() {
  let httpClient = new HttpClient();
  let container = document.getElementById("game");
  
  function followLink(link: Link) {
    container.innerHTML = "";

    function getPromise(link: Link) {
      switch(link.method) {
        case Method.Get:
          return httpClient.get(link.link);
        case Method.Post:
          return httpClient.post(link.link, link.body);
      }
    }

    getPromise(link).then((response)=> {
      let gui = createGui(response.content, followLink);
      container.appendChild(gui.render());
    });
  }

  followLink({method: Method.Get, link: "http://localhost:4000"});
}

start();