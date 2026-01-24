import { useAuthStore } from "@/stores/auth";
import { UIMessage } from "ai";

export async function* suggestion(messages: UIMessage[]) {
  const token = useAuthStore.getState().token;
  const response = await fetch(
    `${import.meta.env.VITE_API_URL}/api/chat/suggestion`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({
        messages: messages,
      }),
    },
  );

  if (!response.ok) {
    throw new Error(`Failed to generate suggestion: ${response.statusText}`);
  }

  const dataReader = response.body?.getReader();
  const decoder = new TextDecoder();
  let fullText = "";

  while (dataReader) {
    const { done, value } = await dataReader.read();
    if (done) break;

    const chunk = decoder.decode(value, { stream: true });
    const lines = chunk.split("\n");

    for (const line of lines) {
      if (line.startsWith("data:")) {
        // 不要 trim，保留原始内容以检测空行
        const data = line.substring(5);
        if (data.trim() === "") {
          // 空行表示一个建议结束，yield 并重置
          if (fullText.trim()) {
            yield [{ prompt: fullText.trim() }];
            fullText = "";
          }
        } else if (data.trim().startsWith("[ERROR]")) {
          console.error("Error:", data.trim().substring(7));
        } else {
          fullText += data.trim();
        }
      }
    }
  }

  // 处理最后一个建议（可能没有换行符结尾）
  if (fullText.trim()) {
    yield [{ prompt: fullText.trim() }];
  }
}
