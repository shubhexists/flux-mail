"use client";

import { useState } from "react";

const InteractiveTitle = () => {
  const [isHovered, setIsHovered] = useState(false);

  return (
    <h1
      className="text-4xl sm:text-5xl md:text-6xl font-bold cursor-pointer transition-all duration-300 ease-in-out transform text-center sm:text-left"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      style={{
        textShadow: isHovered ? "4px 4px 0px var(--accent)" : "none",
        transform: isHovered ? "translateY(-4px)" : "none",
      }}
    >
      FLUX MAIL
    </h1>
  );
};

export default InteractiveTitle;
