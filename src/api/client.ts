import type { ApiResponse } from "@/types/api";

const BASE_URL = import.meta.env.VITE_API_URL || "http://47.115.172.218:8080";

class ApiError extends Error {
  constructor(public code: number, message: string) {
    super(message);
    this.name = "ApiError";
  }
}

function getToken(): string | null {
  return localStorage.getItem("token");
}

export function setToken(token: string) {
  localStorage.setItem("token", token);
}

export function clearToken() {
  localStorage.removeItem("token");
}

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  const token = getToken();
  const headers: HeadersInit = {
    "Content-Type": "application/json",
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
    ...options.headers,
  };

  const res = await fetch(`${BASE_URL}${path}`, { ...options, headers });
  const json: ApiResponse<T> = await res.json();

  if (json.code !== 0) {
    throw new ApiError(json.code, json.message);
  }

  return json.data;
}

export const client = {
  get: <T>(path: string, params?: object) => {
    if (!params) return request<T>(path);
    const searchParams = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) {
      if (v === undefined || v === null) continue;
      if (Array.isArray(v)) {
        v.forEach((item) => searchParams.append(k, String(item)));
      } else {
        searchParams.append(k, String(v));
      }
    }
    const query = searchParams.toString();
    return request<T>(path + (query ? `?${query}` : ""));
  },
  post: <T>(path: string, body?: unknown) =>
    request<T>(path, { method: "POST", body: JSON.stringify(body) }),
  put: <T>(path: string, body?: unknown) =>
    request<T>(path, { method: "PUT", body: JSON.stringify(body) }),
  patch: <T>(path: string, body?: unknown) =>
    request<T>(path, { method: "PATCH", body: JSON.stringify(body) }),
  delete: <T>(path: string) => request<T>(path, { method: "DELETE" }),
};
