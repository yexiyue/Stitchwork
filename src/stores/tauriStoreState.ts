import type { StateStorage } from "zustand/middleware";
import { isTauri } from "@/utils/platform";

// Tauri Store 类型
type TauriStore = {
  get: <T>(key: string) => Promise<T | null | undefined>;
  set: (key: string, value: unknown) => Promise<void>;
  delete: (key: string) => Promise<boolean>;
  save: () => Promise<void>;
};

export class TauriStoreState implements StateStorage {
  private store: TauriStore | null = null;
  private prefix: string;
  private initPromise: Promise<void> | null = null;

  constructor(public storeName: string) {
    // 用于 localStorage 的 key 前缀
    this.prefix = storeName.replace(".json", "");
  }

  init() {
    if (!this.initPromise) {
      this.initPromise = this.doInit();
    }
    return this.initPromise;
  }

  private async doInit() {
    if (isTauri()) {
      try {
        const { load } = await import("@tauri-apps/plugin-store");
        // 加超时避免卡死
        this.store = await Promise.race([
          load(this.storeName),
          new Promise<never>((_, reject) =>
            setTimeout(() => reject(new Error("Store load timeout")), 3000)
          ),
        ]);
      } catch (e) {
        // Tauri store 加载失败，回退到 localStorage
        console.warn("Tauri store not available, using localStorage:", e);
      }
    }
  }

  private async ensureInit() {
    if (this.initPromise) {
      await this.initPromise;
    }
  }

  async getItem(name: string) {
    await this.ensureInit();
    if (this.store) {
      return (await this.store.get<string>(name)) || null;
    }
    // 浏览器环境使用 localStorage
    return localStorage.getItem(`${this.prefix}:${name}`);
  }

  async setItem(name: string, value: string) {
    await this.ensureInit();
    if (this.store) {
      await this.store.set(name, value);
      await this.store.save();
    } else {
      // 浏览器环境使用 localStorage
      localStorage.setItem(`${this.prefix}:${name}`, value);
    }
  }

  async removeItem(name: string) {
    await this.ensureInit();
    if (this.store) {
      await this.store.delete(name);
      await this.store.save();
    } else {
      // 浏览器环境使用 localStorage
      localStorage.removeItem(`${this.prefix}:${name}`);
    }
  }
}
