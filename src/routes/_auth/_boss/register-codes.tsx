import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { RegisterCodeManager } from "@/components";

export const Route = createFileRoute("/_auth/_boss/register-codes")({
  component: BossRegisterCodesPage,
});

function BossRegisterCodesPage() {
  const navigate = useNavigate();
  return (
    <RegisterCodeManager
      queryKey={["boss", "register-codes"]}
      onBack={() => navigate({ to: "/profile" })}
    />
  );
}
