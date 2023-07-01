import React from "react";
import { useState, useEffect } from "react";

interface GroupCardProps {
  name: string;
}

const GroupCard = ({ name }: GroupCardProps) => {
  return (
    <div>
      <h3>{name}</h3>
      <div>{}</div>
    </div>
  );
};
