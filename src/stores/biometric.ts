import { create } from "zustand";

/** 后台超时时间（毫秒）- 5分钟 */
const BACKGROUND_TIMEOUT_MS = 5 * 60 * 1000;

interface BiometricState {
  /** 是否已通过生物识别验证 */
  verified: boolean;
  /** 验证通过的时间戳 */
  verifiedAt: number | null;
  /** 应用进入后台的时间戳 */
  backgroundAt: number | null;
  /** 设置验证状态 */
  setVerified: (verified: boolean) => void;
  /** 记录进入后台时间 */
  setBackground: () => void;
  /** 检查并清除过期的验证状态（从后台恢复时调用） */
  checkAndClearIfExpired: () => void;
  /** 清除验证状态 */
  clear: () => void;
}

export const useBiometricStore = create<BiometricState>()((set, get) => ({
  verified: false,
  verifiedAt: null,
  backgroundAt: null,

  setVerified: (verified) =>
    set({
      verified,
      verifiedAt: verified ? Date.now() : null,
    }),

  setBackground: () =>
    set({
      backgroundAt: Date.now(),
    }),

  checkAndClearIfExpired: () => {
    const { backgroundAt, verified } = get();
    if (!verified || !backgroundAt) {
      set({ backgroundAt: null });
      return;
    }

    const elapsed = Date.now() - backgroundAt;
    if (elapsed >= BACKGROUND_TIMEOUT_MS) {
      set({
        verified: false,
        verifiedAt: null,
        backgroundAt: null,
      });
    } else {
      set({ backgroundAt: null });
    }
  },

  clear: () =>
    set({
      verified: false,
      verifiedAt: null,
      backgroundAt: null,
    }),
}));
