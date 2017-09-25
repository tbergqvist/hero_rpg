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

interface Link {
  method: Method;
  link: string;
  body?: any;
}

interface Text {
  type: string;
  value: string;
}

interface Form {
  type: string;
  name: string;
  method: Method;
  link: string;
  fields: Field[];
}

function renderText(text: Text) {
  return <div>{text.value}</div>;
}

function renderForm(form: Form, followLink: (link: Link)=>void) {
  function requestNewState(event: Event, form: Form) {
    event.preventDefault();
    followLink(Object.assign(form, {
      body: form.fields.reduce((obj, field)=> {
        obj[field.name] = field.value;
        return obj;
      }, {})
    }));
  }

  return (
    <form onsubmit={(e)=>requestNewState(e, form)}>
      {form.name}
      {form.fields.map((field)=> {
        return <input type={field.type} value={field.value} onchange={(event)=>field.value = event.target.value} />
      })}
      <button>Confirm</button>
    </form>
    );
}

function renderItem(item: GuiItem, followLink: (link: Link)=>void) {
  switch(item.type) {
    case "text": 
      return renderText(item as Text);
    case "form":
      return renderForm(item as Form, followLink);
  }
}

type GuiItem = Text | Form;

export function createGui(game: GuiItem[], followLink: (link: Link)=>void) {
  

  let elements = game.map(i => renderItem(i, followLink));
  /*
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
*/

  let gui = {
    render() {
      return (
      <div>
        {elements}
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