import { create } from "zustand";

interface AppState {
  // 示例状态
}

export const useAppStore = create<AppState>()(() => ({
  // 初始状态
}));
