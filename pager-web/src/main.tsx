import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import {
  RouterProvider,
  createMemoryRouter,
  Route,
  createRoutesFromElements
} from "react-router-dom";
import "./styles.css";
import { LoginPage } from "./pages/Login";

const router = createMemoryRouter(
  createRoutesFromElements(
    <Route path="/" element={<LoginPage/>}></Route>
  )
);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
