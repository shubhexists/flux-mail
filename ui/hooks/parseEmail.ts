import { Attachment, simpleParser } from "mailparser";

export interface SemiParserEmail {
  subject: string;
  from: string;
  text: string;
  html: string;
  text_as_html: string;
  attachments: Attachment[];
  date: Date;
}

export async function parseEmailContent(
  rawData: string
): Promise<SemiParserEmail> {
  try {
    const parsed = await simpleParser(rawData);
    let a = {
      subject: parsed.subject || "",
      from: parsed.from?.text || "",
      text: parsed.text || "",
      html: parsed.html || "",
      text_as_html: parsed.textAsHtml || "",
      attachments: parsed.attachments || [],
      date: parsed.date || new Date(),
    };

    console.log(a);
    return a;
  } catch (error) {
    console.error("Error parsing email:", error);
    throw error;
  }
}
