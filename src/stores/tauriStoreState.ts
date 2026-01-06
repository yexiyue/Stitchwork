import { load, type Store } from "@tauri-apps/plugin-store";
import type { StateStorage } from "zustand/middleware";

export class TauriStoreState implements StateStorage {
  private store: Store | null = null;

  constructor(public storeName: string) {}

  async init() {
    this.store = await load(this.storeName);
  }

  async getItem(name: string) {
    return (await this.store?.get<string>(name)) || null;
  }

  async setItem(name: string, value: string) {
    await this.store?.set(name, value);
    await this.store?.save();
  }

  async removeItem(name: string) {
    await this.store?.delete(name);
    await this.store?.save();
  }
}
