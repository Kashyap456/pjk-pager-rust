import { event } from "@tauri-apps/api";
import React from "react";
import { useState, useEffect } from "react";
import WebSocket from "tauri-plugin-websocket-api";
import axios from "axios";
import { Store } from "tauri-plugin-store-api";
import { Key } from "../utils";

const URL = "ws://localhost:7777/ws";

const onJoin = async (group: string, ws: WebSocket) => {
  const store = new Store(".keys.dat");
  let token: Key | null = await store.get("access");
  if (token === null) {
    throw new Error("Authentication failed.");
  }
  const { username, access, refresh } = token;
  let res = await axios.post(
    "http://localhost:8000/memberships",
    {
      name: group,
      user: username,
    },
    {
      headers: {
        Authorization: `Bearer ${access}`,
      },
    }
  );
  if (res.status != 200) {
    console.log(res.data);
  }
  await ws.send(`join ${group}`);
};

const onCreate = async (name: string, ws: WebSocket) => {
  const store = new Store(".keys.dat");
  let token: Key | null = await store.get("access");
  if (token === null) {
    throw new Error("Authentication failed.");
  }
  const { username, access, refresh } = token;
  let user = username;
  console.log("here");
  // let res = await axios.get("/api/").catch((err) => console.log(err));
  // console.log(res && res.data);
  let res = await axios
    .post(
      "http://localhost:8000/groups",
      {
        name,
        user,
      },
      {
        headers: {
          Authorization: `Bearer ${access}`,
        },
      }
    )
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
  await ws.send(`create ${name}`).catch((err) => {
    if (err) {
      console.log(err);
    }
  });
  console.log("end");
};

export const Home = () => {
  const [groups, setGroups] = useState([]);
  const [message, setMessage] = useState("");
  const [groupmute, setGroupMute] = useState("");
  const [websocket, setWebSocket] = useState<WebSocket | undefined>(undefined);

  useEffect(() => {
    const buildSocket = async () => {
      const ws = await WebSocket.connect("ws://0.0.0.0:80/ws")
        .then((r) => {
          return r;
        })
        .catch((err) => {
          if (err) {
            console.log(err);
          }
        });
      if (!!!ws) {
        return;
      }
      console.log(ws);
      ws.addListener((message) => {
        const messageText = message.data;
        console.log(messageText);
        if (typeof messageText === "string") {
          //setMessage(messageText);
        }
      });
      setWebSocket(ws);
    };
    buildSocket();
  }, []);
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
          onClick={(event) => {
            event.preventDefault();
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
          onClick={(event) => {
            event.preventDefault();
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
