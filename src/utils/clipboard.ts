import { isTauri } from "@/utils/platform";

/**
 * Copy text to clipboard
 * - Uses Tauri clipboard plugin in Tauri environment
 * - Falls back to browser APIs otherwise
 */
export async function copyToClipboard(text: string): Promise<boolean> {
  // Use Tauri plugin in Tauri environment
  if (isTauri()) {
    try {
      const { writeText } = await import("@tauri-apps/plugin-clipboard-manager");
      await writeText(text);
      return true;
    } catch {
      // Fall through to browser methods
    }
  }

  // Try modern clipboard API
  if (navigator.clipboard?.writeText) {
    try {
      await navigator.clipboard.writeText(text);
      return true;
    } catch {
      // Fall through to legacy method
    }
  }

  // Fallback: use execCommand (deprecated but widely supported)
  try {
    const textarea = document.createElement("textarea");
    textarea.value = text;
    textarea.style.position = "fixed";
    textarea.style.left = "-9999px";
    textarea.style.top = "-9999px";
    document.body.appendChild(textarea);
    textarea.focus();
    textarea.select();
    const success = document.execCommand("copy");
    document.body.removeChild(textarea);
    return success;
  } catch {
    return false;
  }
}
