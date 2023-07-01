import axios from "axios";
import { access } from "fs";
import React from "react";
import { useState } from "react";
import { Store } from "tauri-plugin-store-api";

const onSubmit = async (
  username: string,
  password: string,
  event: React.FormEvent<HTMLFormElement>
) => {
  const store = new Store(".keys.dat");
  let response = await axios.post("http://localhost:8080/login_user", {
    username,
    password,
  });
  if (response.status != 200) {
    console.error(response.statusText);
    return;
  }
  const { access_token, refresh_token } = response.data;
  await store.set("access", { value: access_token });
  event.preventDefault();
};

const onChange = (
  e: React.ChangeEvent<HTMLInputElement>,
  setFunc: Function
) => {
  setFunc(e.target.value);
};

export const LoginPage = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <div>
      <form
        className="grid grid-cols-1"
        onSubmit={(event) => onSubmit(username, password, event)}
      >
        <input
          type="text"
          placeholder="username"
          value={username}
          onChange={(e) => onChange(e, setUsername)}
        />
        <input
          type="password"
          placeholder="password"
          value={password}
          onChange={(e) => onChange(e, setPassword)}
        />
        <button type="submit">Login</button>
      </form>
    </div>
  );
};
