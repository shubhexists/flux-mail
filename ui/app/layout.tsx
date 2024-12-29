import { ThemeProvider } from "@/contexts/ThemeContext";
import "@/styles/globals.css";
import { SpeedInsights } from "@vercel/speed-insights/next";
import { Analytics } from "@vercel/analytics/react";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <ThemeProvider>
          <SpeedInsights />
          <Analytics />
          <main className="min-h-screen p-8">{children}</main>
        </ThemeProvider>
      </body>
    </html>
  );
}
