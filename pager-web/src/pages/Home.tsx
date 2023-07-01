import React from "react";
import { useState } from "react";
import useWebSocket from "react-use-websocket";

const URL = "ws://0.0.0.0:7777/ws";

export const Home = () => {
  const [groups, setGroups] = useState(["pjk"]);
  useWebSocket(URL, {
    onOpen: () => console.log("websocket connection established."),
  });
  return (
    <div>
      {groups.map((group) => (
        <p>{group}</p>
      ))}
    </div>
  );
};
