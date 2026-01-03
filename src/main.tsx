import React from "react";
import ReactDOM, { createRoot } from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { unstableSetRender } from "antd-mobile";
import { routeTree } from "./routeTree.gen";
import "./index.css";

// React 19 compatibility for antd-mobile
unstableSetRender((node, container) => {
  (container as any)._reactRoot ||= createRoot(container);
  const root = (container as any)._reactRoot;
  root.render(node);
  return async () => {
    await new Promise((resolve) => setTimeout(resolve, 0));
    root.unmount();
  };
});

const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
