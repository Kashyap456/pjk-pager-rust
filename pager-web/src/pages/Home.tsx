import { event } from "@tauri-apps/api";
import React from "react";
import { useState, useEffect } from "react";
import WebSocket from "tauri-plugin-websocket-api";
import axios from "axios";
import { Store } from "tauri-plugin-store-api";
import { Key } from "../utils";
import { GroupCard } from "./components/GroupCard";

const URL = "ws://localhost:7777/ws";

const onJoin = async (group: string, ws: WebSocket) => {
  const store = new Store(".keys.dat");
  let token: Key | null = await store.get("access");
  if (token === null) {
    throw new Error("Authentication failed.");
  }
  const { username, access, refresh } = token;
  let user = username;
  console.log(user);
  let res = await axios
    .post(
      "http://localhost:8000/memberships",
      {
        name: group,
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
        console.log(err.response && err.response.data);
      }
    });
  if (!!!res) {
    return;
  }
  console.log(res.data);
  await ws.send(`join ${name}`).catch((err) => {
    if (err) {
      console.log(err);
    }
  });
};

const onCreate = async (name: string, ws: WebSocket) => {
  const store = new Store(".keys.dat");
  let token: Key | null = await store.get("access");
  if (token === null) {
    throw new Error("Authentication failed.");
  }
  const { username, access, refresh } = token;
  let user = username;
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
  const [updateTrigger, setUpdateTrigger] = useState(false);
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
          setMessage(messageText);
        }
      });
      setWebSocket(ws);
    };
    buildSocket();
  }, []);

  useEffect(() => {
    const getGroups = async () => {
      const store = new Store(".keys.dat");
      let token: Key | null = await store.get("access");
      if (token === null) {
        throw new Error("Authentication failed.");
      }
      const { username, access, refresh } = token;
      console.log(username);
      let user = username;
      const res = await axios.get(
        `http://localhost:8000/userin?user=${username}`
      );
      const groups = res.data;
      console.log(groups);
      setGroups(groups);
    };
    getGroups();
  }, [message, updateTrigger]);

  return (
    <div className="shadow-md">
      {websocket &&
        groups.map((group) => (
          <GroupCard
            key={group}
            group={group}
            ws={websocket}
            update={setUpdateTrigger}
          />
        ))}
      <form>
        <input
          type="text"
          placeholder="group name"
          value={groupmute}
          onChange={(e) => setGroupMute(e.target.value)}
        />
        <button
          type="submit"
          value="create"
          onClick={async (event) => {
            event.preventDefault();
            event.persist();
            if (websocket) {
              await onCreate(groupmute, websocket);
              setGroupMute("");
              setUpdateTrigger((prev) => !prev);
            }
          }}
        >
          Create
        </button>
        <button
          type="submit"
          value="join"
          onClick={async (event) => {
            event.preventDefault();
            event.persist();
            if (websocket) {
              await onJoin(groupmute, websocket);
              setGroupMute("");
              setUpdateTrigger((prev) => !prev);
            }
          }}
        >
          Join
        </button>
      </form>
    </div>
  );
};
