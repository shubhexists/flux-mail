"use client";

import { useTheme } from "@/contexts/ThemeContext";

export default function ThemeToggle() {
  const { theme, toggleTheme } = useTheme();

  return (
    <button
      onClick={toggleTheme}
      className="neutro-button text-xl sm:text-2xl w-full sm:w-auto"
    >
      {theme === "light" ? "DARK" : "LIGHT"}
    </button>
  );
}
