const AVATAR_COLORS = [
  "bg-blue-500",
  "bg-green-500",
  "bg-orange-500",
  "bg-purple-500",
  "bg-pink-500",
  "bg-teal-500",
];

export function Avatar({ name }: { name: string }) {
  const firstChar = name.charAt(0);
  const colorIndex = name.charCodeAt(0) % AVATAR_COLORS.length;
  return (
    <div
      className={`w-10 h-10 rounded-full ${AVATAR_COLORS[colorIndex]} flex items-center justify-center text-white font-medium`}
    >
      {firstChar}
    </div>
  );
}
