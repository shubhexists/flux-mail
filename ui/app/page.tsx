"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import ThemeToggle from "@/components/ThemeToggle";
import AnimatedBackground from "@/components/AnimatedBackground";
import InteractiveTitle from "@/components/InteractiveTitle";

export default function Home() {
  const [searchTerm, setSearchTerm] = useState("");
  const router = useRouter();

  const handleSearch = () => {
    if (searchTerm.trim()) {
      router.push(`/search?q=${encodeURIComponent(searchTerm.toLowerCase())}`);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      handleSearch();
    }
  };

  return (
    <div className="min-h-screen flex flex-col justify-center items-center relative overflow-hidden p-4">
      <AnimatedBackground />
      <div className="z-10 w-full max-w-4xl">
        <header className="flex flex-col sm:flex-row justify-between items-center mb-8 sm:mb-12 gap-4">
          <InteractiveTitle />
          <ThemeToggle />
        </header>
        <div className="neutro-box p-4 sm:p-6 mb-8 backdrop-blur-sm bg-background/30">
          <div className="flex flex-col sm:flex-row gap-4">
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder="Get your username"
              className="neutro-input flex-grow text-xl sm:text-2xl w-full"
            />
            <button
              onClick={handleSearch}
              className="neutro-button text-xl sm:text-2xl w-full sm:w-auto"
            >
              GO
            </button>
          </div>
        </div>
        <div className="text-center mt-8 sm:mt-12 neutro-box p-4 sm:p-6 backdrop-blur-sm bg-background/30">
          <h2 className="text-3xl sm:text-4xl font-bold mb-4">
            Temp Mail Service
          </h2>
          <p className="text-xl sm:text-2xl">
            Remember: Your mails are public. Don&apos;t use it for important
            mails. Use it to subscribe to all unwanted services.
          </p>
        </div>
      </div>
    </div>
  );
}
