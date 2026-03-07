import { invoke } from "@tauri-apps/api/core";

export interface SendConfig {
  username?: string;
  password?: string;
  url_invoice?: string;
  url_perception_retention?: string;
  url_despatch?: string;
  beta: boolean;
}

export interface CdrResponse {
  type: "Cdr";
  cdr_base64: string;
  response_code: string;
  description: string;
  notes: string[];
}

export interface TicketResponse {
  type: "Ticket";
  ticket: string;
}

export interface ErrorResponse {
  type: "Error";
  code: string;
  message: string;
}

export type SendResponse = CdrResponse | TicketResponse | ErrorResponse;

export interface VerifyCdrResponse {
  type: "Cdr";
  cdr_base64: string;
  status_code: string;
  response_code: string;
  description: string;
  notes: string[];
}

export interface VerifyErrorResponse {
  type: "Error";
  code: string;
  message: string;
}

export type VerifyResponse = VerifyCdrResponse | VerifyErrorResponse;

export async function createXml(documentJson: string): Promise<string> {
  return invoke<string>("create_xml", { documentJson });
}

export async function signXml(
  xmlContent: string,
  privateKeyPem: string | null,
  certificatePem: string | null,
  beta: boolean
): Promise<string> {
  return invoke<string>("sign_xml", {
    xmlContent,
    privateKeyPem,
    certificatePem,
    beta,
  });
}

export async function sendXml(
  signedXml: string,
  config: SendConfig
): Promise<SendResponse> {
  return invoke<SendResponse>("send_xml", { signedXml, config });
}

export async function verifyTicket(
  ticket: string,
  config: SendConfig
): Promise<VerifyResponse> {
  return invoke<VerifyResponse>("verify_ticket", { ticket, config });
}
