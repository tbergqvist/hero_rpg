interface Construct<T> {
  new (...params: any[]): T;
}

export module GuiLib {
  function parseAttributes(element: HTMLElement, attributes: any) {
    for (let key in attributes) {
      switch (key) {
        case "href": 
        case "tabindex": 
        case "class": 
        case "id": 
        case "type": 
        case "value": 
          element.setAttribute(key, attributes[key]);
          break;
        case "onclick": 
        case "onkeydown": 
        case "onkeyup": 
        case "onsubmit": 
        case "onchange": 
          element[key] = attributes[key];
          break;
        default:
        console.error(`unknown html attribute ${key}`);
          break;
      }
    }
  }

  export function createElement(tag: string | Construct<JSX.ElementClass>, attributes: any, ...children: (Node | number | string | boolean | Node[])[]) {
    let element: HTMLElement;
    if (typeof tag === "string") {
      element = document.createElement(tag);
      parseAttributes(element, attributes);
    } else {
      let customElement = new tag(attributes);
      customElement.attributes = attributes;
      element = customElement.render();
    }

    children.forEach((child) => {
      if (child instanceof Node) {
        element.appendChild(child);
      } else if (Array.isArray(child)) {
        child.forEach((child) => {
          element.appendChild(child);
        });
      } else {
        let node = document.createTextNode(child.toString());
        element.appendChild(node);
      }
    });

    return element;
  }
}