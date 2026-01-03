import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { LoginUser } from "@/types";
import { authApi } from "@/api";
import { setToken, clearToken } from "@/api/client";

interface AuthState {
  token: string | null;
  user: LoginUser | null;
  login: (username: string, password: string) => Promise<void>;
  logout: () => void;
  updateUser: (updates: Partial<LoginUser>) => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      token: null,
      user: null,
      login: async (username, password) => {
        const res = await authApi.login({ username, password });
        setToken(res.token);
        set({ token: res.token, user: res.user });
      },
      logout: () => {
        clearToken();
        set({ token: null, user: null });
      },
      updateUser: (updates) =>
        set((state) => ({
          user: state.user ? { ...state.user, ...updates } : null,
        })),
    }),
    {
      name: "auth-storage",
      partialize: (state) => ({ token: state.token, user: state.user }),
    }
  )
);

// Helper selectors
export const selectIsAuthenticated = (state: AuthState) => !!state.token;
export const selectIsBoss = (state: AuthState) => state.user?.role === "boss";
