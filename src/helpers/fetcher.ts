import {isPermissionGranted, requestPermission, sendNotification} from "@tauri-apps/api/notification";

export default async function fetcher<JSON = any>(
  input: RequestInfo,
  init?: RequestInit
): Promise<JSON> {
  const bearerToken = localStorage.getItem("token");

  if (!bearerToken) {
    throw new Error("No token found");
  }

  const res = await fetch(input, {
    headers: { Authorization: `Bearer ${bearerToken}` },
  });

  if (!res.ok) {
    throw new Error("Not authenticated");
  }

  return res.json();
}

export async function infoFetcher<JSON = any>(
    input?: RequestInfo,
    init?: RequestInit
): Promise<JSON> {
  const accessUrl = localStorage.getItem("access-url");

  if (!accessUrl) {
    throw new Error("No access url found");
  }

  const res = await fetch(accessUrl, {
    method: "POST",
    body: JSON.stringify({action: "info", version: 1})
  });

  if (!res.ok) {
    throw new Error("Not authenticated");
  }

  return res.json();
}

export async function botsFetcher<JSON = any>(
    input?: RequestInfo,
    init?: RequestInit
): Promise<JSON> {
  const accessUrl = localStorage.getItem("access-url");

  if (!accessUrl) {
    throw new Error("No access url found");
  }

  const res = await fetch(accessUrl, {
    method: "POST",
    body: JSON.stringify({action: "bots", version: 1})
  });

  if (!res.ok) {
    throw new Error("Not authenticated");
  }

  return res.json();
}
