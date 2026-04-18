export interface StatusProps {
  variant?: "online" | "offline";
  className?: string;
}

export function Status({ variant = "online", className }: StatusProps) {
  const dotColor = variant === "online" ? "bg-green-500" : "bg-red-500";

  return (
    <span
      className={`relative inline-flex h-2 w-2 rounded-full ${dotColor} ${className ?? ""}`}
    >
      <span
        className={`absolute inline-flex h-full w-full rounded-full opacity-75 animate-ping ${dotColor}`}
      />
    </span>
  );
}

export default Status;
