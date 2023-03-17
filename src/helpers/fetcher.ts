import { getStore } from "./store";

export default async function fetcher<JSON = any>(
  input: RequestInfo,
  init?: RequestInit
): Promise<JSON> {
  const bearerToken = await getStore("token");

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

export async function actionFetcher<JSON = any>(
    action: string,
    content?: Object,
): Promise<JSON> {
  const accessUrl = await getStore("access-url");

  if (!accessUrl) {
    throw new Error("No access url found");
  }

  let res;
  try {
     res = await fetch(accessUrl as string, {
      method: "POST",
      body: JSON.stringify({action: action, version: 1, content: content})
    });
  } catch (err) {
    console.log("request",err);
    throw new Error("Request error");
  }

  if (!res.ok) {
    throw new Error("Not authenticated");
  }

  return res.json();
}
