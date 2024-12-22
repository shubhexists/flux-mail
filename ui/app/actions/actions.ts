"use server";

import { pool } from "@/lib/db";

export async function searchEmails(recipientQuery: string) {
  try {
    const result = await pool.query(
      `SELECT date, sender, recipients, data 
       FROM mail 
       WHERE recipients LIKE $1`,
      [`%${recipientQuery}%`]
    );
    return result.rows;
  } catch (error) {
    console.error("Database error:", error);
    throw new Error("Failed to search emails");
  }
}
