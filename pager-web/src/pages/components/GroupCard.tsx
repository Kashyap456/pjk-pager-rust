import React from "react";
import { useState, useEffect } from "react";
import WebSocket from "tauri-plugin-websocket-api";

interface GroupCardProps {
  name: string;
  ws: WebSocket;
}

const send = async (message: string, ws: WebSocket) => {
  await ws.send(message);
};

export const GroupCard = ({ name, ws }: GroupCardProps) => {
  return (
    <div>
      <h3>{name}</h3>
      <button onClick={() => send(`page ${name}`, ws)}>Page</button>
    </div>
  );
};
