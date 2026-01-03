const AVATAR_COLORS = [
  "bg-blue-500",
  "bg-green-500",
  "bg-orange-500",
  "bg-purple-500",
  "bg-pink-500",
  "bg-teal-500",
];

interface AvatarProps {
  name: string;
  src?: string | null;
  size?: "sm" | "md" | "lg";
}

const sizeClasses = {
  sm: "w-8 h-8 text-sm",
  md: "w-10 h-10 text-base",
  lg: "w-16 h-16 text-2xl",
};

export function Avatar({ name, src, size = "md" }: AvatarProps) {
  const firstChar = name.charAt(0);
  const colorIndex = name.charCodeAt(0) % AVATAR_COLORS.length;

  if (src) {
    return (
      <img
        src={src}
        alt={name}
        className={`${sizeClasses[size]} rounded-full object-cover`}
      />
    );
  }

  return (
    <div
      className={`${sizeClasses[size]} rounded-full ${AVATAR_COLORS[colorIndex]} flex items-center justify-center text-white font-medium`}
    >
      {firstChar}
    </div>
  );
}
