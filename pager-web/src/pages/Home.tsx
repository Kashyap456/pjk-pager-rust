import { event } from "@tauri-apps/api";
import React from "react";
import { useState, useEffect } from "react";
import WebSocket from "tauri-plugin-websocket-api";
import axios from "axios";
import { Store } from "tauri-plugin-store-api";
import { Key } from "../utils";
import { useWebSocket } from "react-use-websocket/dist/lib/use-websocket";

const URL = "ws://localhost:7777/ws";

const onJoin = async (group: string, ws: WebSocket) => {
  const store = new Store(".keys.dat");
  let token: Key | null = await store.get("access");
  if (token === null) {
    throw new Error("Authentication failed.");
  }
  const { username, access, refresh } = token;
  // let res = await axios.post(
  //   "http://localhost:3000/memberships",
  //   {
  //     name: group,
  //     user: username,
  //   },
  //   {
  //     headers: {
  //       Authorization: `Bearer ${access}`,
  //     },
  //   }
  // );
  // if (res.status != 200) {
  //   console.log(res.data);
  // }
  //await ws.send(`join ${group}`);
};

const onCreate = async (group: string, ws: WebSocket) => {
  const store = new Store(".keys.dat");
  let token: Key | null = await store.get("access");
  if (token === null) {
    throw new Error("Authentication failed.");
  }
  const { username, access, refresh } = token;
  let res = await axios
    .post(
      "http://localhost:3000/groups",
      {
        name: group,
        user: username,
      },
      {
        headers: {
          Authorization: `Bearer ${access}`,
        },
      }
    )
    .catch(async (err) => {
      if (err) {
        console.log(err);
      }
    });
  if (!!!res) {
    return;
  }
  await ws.send(`create ${group}`).catch((err) => {
    if (err) {
      console.log(err);
    }
  });
};

export const Home = () => {
  const [groups, setGroups] = useState([]);
  const [message, setMessage] = useState("");
  const [groupmute, setGroupMute] = useState("");
  const [websocket, setWebSocket] = useState<WebSocket | undefined>(undefined);

  useWebSocket("ws://0.0.0.0:7777/ws", {
    onOpen: (event) => {
      console.log(event);
    },
  });
  useEffect(() => {}, [message]);
  return (
    <div className="shadow-md">
      {groups.map((group) => (
        <p>{group}</p>
      ))}
      <form>
        <input
          type="text"
          placeholder="group name"
          onChange={(e) => setGroupMute(e.target.value)}
        />
        <button
          type="submit"
          value="create"
          onClick={() => {
            if (websocket) {
              onCreate(groupmute, websocket);
            }
            setGroupMute("");
          }}
        >
          Create
        </button>
        <button
          type="submit"
          value="join"
          onClick={() => {
            if (websocket) {
              onJoin(groupmute, websocket);
            }
            setGroupMute("");
          }}
        >
          Join
        </button>
      </form>
    </div>
  );
};
