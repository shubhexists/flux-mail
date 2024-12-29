"use client";

import Link from "next/link";
import ThemeToggle from "../../components/ThemeToggle";
import { Suspense, useEffect, useState } from "react";
import { searchEmails } from "@/app/actions/actions";
import { useSearchParams, useRouter } from "next/navigation";
import { SemiParserEmail } from "@/hooks/parseEmail";

export interface Email {
  date: string;
  sender: string;
  recipients: string;
  data: SemiParserEmail;
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
    return <div className="text-center text-2xl">Loading...</div>;
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
          <div className="neutro-box p-8 overflow-hidden">
            <h1 className="text-3xl sm:text-4xl font-bold mb-4 break-words">
              {selectedEmail.data.subject}
            </h1>
            <p className="text-xl sm:text-2xl mb-4">
              {new Date(selectedEmail.date).toLocaleDateString()}
            </p>
            <p className="text-lg sm:text-xl mb-2 break-words">
              From: {selectedEmail.data.from}
            </p>
            <p className="text-lg sm:text-xl mb-4 break-words">
              To: {selectedEmail.recipients}
            </p>
            <div className="border-t border-foreground pt-4 mt-4">
              <div
                className="text-lg sm:text-xl whitespace-pre-wrap break-words prose dark:prose-invert max-w-none"
                dangerouslySetInnerHTML={{ __html: selectedEmail.data.html }}
              ></div>
            </div>
          </div>
        </>
      ) : (
        <>
          <h2 className="text-3xl sm:text-4xl font-bold mb-6 break-words">
            Mails for &quot;{query}@flux.shubh.sh&quot;
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            {emails.map((email, index) => (
              <Link href={`/search?q=${query}&email=${index}`} key={index}>
                <div className="neutro-box p-6 hover:bg-accent transition-colors h-[200px] flex flex-col">
                  <h3 className="text-2xl sm:text-3xl font-bold mb-2 break-words line-clamp-2 flex-none">
                    {email.data.subject || "(No Subject)"}
                  </h3>
                  <p className="text-lg sm:text-xl mb-2 flex-none">
                    {new Date(email.date).toLocaleDateString()}
                  </p>
                  <p className="text-base sm:text-lg line-clamp-2 overflow-hidden">
                    {email.data.text}
                  </p>
                </div>
              </Link>
            ))}
          </div>
          {emails.length === 0 && (
            <p className="text-xl sm:text-2xl text-center neutro-box p-8">
              No mails found. Try sending a mail to <br />
              <span className="font-bold">
                &apos;{query}@flux.shubh.sh&apos;
              </span>{" "}
              <br />
              and Try Again.
            </p>
          )}
        </>
      )}
    </div>
  );
}

export default function SearchResults() {
  return (
    <div className="space-y-8 p-4 max-w-6xl mx-auto">
      <header className="flex flex-col sm:flex-row justify-between items-center mb-12 gap-4">
        <Link
          href="/"
          className="text-4xl sm:text-6xl font-bold hover:text-accent transition-colors"
        >
          Flux Mail
        </Link>
        <ThemeToggle />
      </header>
      <Suspense
        fallback={<div className="text-center text-2xl">Loading...</div>}
      >
        <SearchResultsContent />
      </Suspense>
    </div>
  );
}
