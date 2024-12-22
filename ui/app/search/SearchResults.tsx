"use client";

import Link from "next/link";
import ThemeToggle from "../../components/ThemeToggle";
import { Suspense, useEffect, useState } from "react";
import { searchEmails } from "@/app/actions/actions";
import { useSearchParams, useRouter } from "next/navigation";

export interface Email {
  date: string;
  sender: string;
  recipients: string;
  data: string;
}

function SearchResultsContent() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const query = searchParams.get("q") || "";
  const selectedEmailId = searchParams.get("email");

  useEffect(() => {
    if (!query) {
      router.push("/");
    }
  }, [query, router]);

  const [emails, setEmails] = useState<Email[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchEmails() {
      if (query) {
        try {
          const result = await searchEmails(`${query}@flux.shubh.sh`);
          setEmails(result);
        } catch (error) {
          console.error("Failed to fetch emails:", error);
        } finally {
          setLoading(false);
        }
      }
    }
    fetchEmails();
  }, [query]);

  if (loading) {
    return <div>Loading...</div>;
  }

  const selectedEmail = selectedEmailId
    ? emails[parseInt(selectedEmailId)]
    : null;

  return (
    <div className="space-y-8">
      {selectedEmail ? (
        <>
          <Link
            href={`/search?q=${query}`}
            className="neutro-button inline-block mb-8 text-2xl"
          >
            BACK
          </Link>
          <div className="neutro-box p-8">
            <h1 className="text-6xl font-bold mb-4">
              {selectedEmail.sender.toUpperCase()}
            </h1>
            <p className="text-3xl mb-4">
              {new Date(selectedEmail.date).toLocaleDateString()}
            </p>
            <p className="text-2xl mb-4">To: {selectedEmail.recipients}</p>
            <p className="text-2xl">{selectedEmail.data}</p>
          </div>
        </>
      ) : (
        <>
          <h2 className="text-4xl font-bold mb-6">
            Mails for &quot;{query}@flux.shubh.sh&quot;
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            {emails.map((email, index) => (
              <Link href={`/search?q=${query}&email=${index}`} key={index}>
                <div className="neutro-box p-6 hover:bg-accent transition-colors">
                  <h2 className="text-4xl font-bold mb-2">
                    {email.sender.toUpperCase()}
                  </h2>
                  <p className="text-2xl">
                    {new Date(email.date).toLocaleDateString()}
                  </p>
                  <p className="text-lg truncate">{email.data}</p>
                </div>
              </Link>
            ))}
          </div>
          {emails.length === 0 && (
            <p className="text-2xl text-center">
              No mails found. Try sending a mail to <br></br>&apos;{query}
              @flux.shubh.sh&apos; <br></br> and Try Again.
            </p>
          )}
        </>
      )}
    </div>
  );
}

export default function SearchResults() {
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
      <Suspense fallback={<div>Loading...</div>}>
        <SearchResultsContent />
      </Suspense>
    </div>
  );
}
