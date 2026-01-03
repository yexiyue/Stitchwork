import { NavBar } from "antd-mobile";
import { ChevronLeft } from "lucide-react";
import { useNavigate } from "@tanstack/react-router";

interface PageHeaderProps {
  title: string;
  onBack?: () => void;
  right?: React.ReactNode;
}

export function PageHeader({ title, onBack, right }: PageHeaderProps) {
  const navigate = useNavigate();

  return (
    <NavBar
      onBack={onBack ?? (() => navigate({ to: "/profile" }))}
      backIcon={<ChevronLeft size={24} />}
      right={right}
    >
      {title}
    </NavBar>
  );
}
