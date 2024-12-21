import Link from "next/link";

const animeDetails = {
  1: {
    title: "Attack on Titan",
    genre: "Action",
    description: "Humanity fights for survival against man-eating giants.",
  },
  2: {
    title: "Death Note",
    genre: "Mystery",
    description:
      "A high school student discovers a notebook that kills anyone whose name is written in it.",
  },
  3: {
    title: "One Punch Man",
    genre: "Comedy",
    description:
      "A superhero who can defeat any opponent with a single punch seeks a worthy challenge.",
  },
  4: {
    title: "My Hero Academia",
    genre: "Superhero",
    description:
      "In a world where most people have superpowers, a boy born without them strives to become a hero.",
  },
};

export default function AnimePage({
  params,
}: {
  params: { id: string; query: string };
}) {
  const anime = animeDetails[params.id as unknown as keyof typeof animeDetails];

  if (!anime) {
    return <div>Anime not found</div>;
  }

  return (
    <div className="space-y-8">
      <Link
        href={`/search?q=${params.query}`}
        className="neutro-button inline-block mb-8 text-2xl"
      >
        BACK
      </Link>
      <div className="neutro-box p-8">
        <h1 className="text-6xl font-bold mb-4">{anime.title.toUpperCase()}</h1>
        <p className="text-3xl mb-4">{anime.genre}</p>
        <p className="text-2xl">{anime.description}</p>
      </div>
    </div>
  );
}
