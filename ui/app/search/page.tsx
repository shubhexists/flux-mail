"use client";

import { useSearchParams } from "next/navigation";
import Link from "next/link";
import ThemeToggle from "../../components/ThemeToggle";

const animeList = [
  { id: 1, title: "Attack on Titan", genre: "Action" },
  { id: 2, title: "Death Note", genre: "Mystery" },
  { id: 3, title: "One Punch Man", genre: "Comedy" },
  { id: 4, title: "My Hero Academia", genre: "Superhero" },
];

export default function SearchResults() {
  const searchParams = useSearchParams();
  const query = searchParams.get("q");

  const filteredAnime = animeList.filter(
    (anime) =>
      anime.title.toLowerCase().includes(query?.toLowerCase() || "") ||
      anime.genre.toLowerCase().includes(query?.toLowerCase() || "")
  );

  return (
    <div className="space-y-8">
      <header className="flex justify-between items-center mb-12">
        <Link
          href="/"
          className="text-6xl font-bold hover:text-accent transition-colors"
        >
          Flux Mail
        </Link>
        <ThemeToggle />
      </header>
      <h2 className="text-4xl font-bold mb-6">Mails for "{query}@flux.shubh.sh"</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        {filteredAnime.map((anime) => (
          <Link href={`/mail/${query}/${anime.id}`} key={anime.id}>
            <div className="neutro-box p-6 hover:bg-accent hover:text-background transition-colors">
              <h2 className="text-4xl font-bold mb-2">
                {anime.title.toUpperCase()}
              </h2>
              <p className="text-2xl">{anime.genre}</p>
            </div>
          </Link>
        ))}
      </div>
      {filteredAnime.length === 0 && (
        <p className="text-2xl text-center">
          No results found. Try a different search term.
        </p>
      )}
    </div>
  );
}
