import React from "react";
import { useState, useEffect } from "react";
import WebSocket from "tauri-plugin-websocket-api";
import { Store } from "tauri-plugin-store-api";
import axios from "axios";
import { Key } from "../../utils";

interface GroupCardProps {
  group: [name: string, admin: number];
  ws: WebSocket;
  update: React.Dispatch<React.SetStateAction<boolean>>;
}

const send = async (message: string, ws: WebSocket) => {
  await ws.send(message);
};

const onDelete = async (group: string, ws: WebSocket) => {
  const store = new Store(".keys.dat");
  let token: Key | null = await store.get("access");
  if (token === null) {
    throw new Error("Authentication failed.");
  }
  const { username, access, refresh } = token;
  let user = username;
  let res = await axios
    .delete(`http://localhost:8000/groups?user=${username}&name=${group}`)
    .catch(async (err) => {
      if (err) {
        console.log("error");
        console.log(err.response && err.response.data);
      }
    });
  if (!!!res) {
    return;
  }
  console.log(res.data);
  await ws.send(`delete ${name}`).catch((err) => {
    if (err) {
      console.log(err);
    }
  });
  console.log("end");
};

export const GroupCard = ({ group, ws, update }: GroupCardProps) => {
  const { 0: name, 1: admin } = group;
  return (
    <div>
      <h3>{name}</h3>
      <button onClick={() => send(`page ${name}`, ws)}>Page</button>
      {admin === 1 && (
        <button
          onClick={async (event) => {
            event.preventDefault();
            event.persist();
            await onDelete(name, ws);
            update((prev) => !prev);
          }}
        >
          Delete
        </button>
      )}
    </div>
  );
};
