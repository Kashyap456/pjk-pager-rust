import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import {
  RouterProvider,
  createBrowserRouter,
  Route,
  Routes,
  createRoutesFromElements,
  BrowserRouter,
} from "react-router-dom";
import "./styles.css";
import { LoginPage, Home } from "./pages/index";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LoginPage />}></Route>
        <Route path="home" element={<Home />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);
