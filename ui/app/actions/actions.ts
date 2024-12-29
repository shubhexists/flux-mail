"use server";

import { parseEmailContent } from "@/hooks/parseEmail";
import { pool } from "@/lib/db";

export async function searchEmails(recipientQuery: string) {
  try {
    const result = await pool.query(
      `SELECT date, sender, recipients, data 
       FROM mail 
       WHERE recipients LIKE $1`,
      [`%${recipientQuery}%`]
    );
    const output = [];
    for (const i of result.rows) {
      const parsedMail = await parseEmailContent(i.data);
      output.push({
        sender: i.sender,
        date: i.date,
        recipients: i.recipients,
        data: parsedMail,
      });
    }
    return output;
  } catch (error) {
    console.error("Database error:", error);
    throw new Error("Failed to search emails");
  }
}
