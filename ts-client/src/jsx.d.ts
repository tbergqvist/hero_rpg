declare namespace JSX {
  interface ElementAttributesProperty {
    attributes; // specify the property name to use
  }

  interface ElementClass {
    attributes?: any;
    render(): HTMLElement;
  }

  interface Element extends HTMLElement {
  }
}